use crate::{CheckoutObservation, LocalPath, ObservationId};
use devmeld_shared_kernel::RepositoryId;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckoutSelection {
    repository: RepositoryId,
    observation_id: ObservationId,
}
impl CheckoutSelection {
    pub fn new(repository: RepositoryId, observation_id: ObservationId) -> Self {
        Self {
            repository,
            observation_id,
        }
    }
    pub fn repository(&self) -> &RepositoryId {
        &self.repository
    }
    pub fn observation_id(&self) -> &ObservationId {
        &self.observation_id
    }
}
#[derive(Clone, Debug, Default)]
pub struct TaskContext {
    pub selections: Vec<CheckoutSelection>,
    pub working_area: Option<LocalPath>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectedSelection {
    source: Option<SelectionSource>,
    selection: Option<CheckoutSelection>,
    reason: RejectionReason,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RejectionReason {
    UnknownRepository,
    UnknownObservation,
    WrongRepository,
    IneligibleObservation,
    ConflictingSelection,
    DuplicateRepositoryIdentity,
    ObservationRepositoryAbsent,
    DuplicateObservationIdentity(ObservationId),
}
impl std::fmt::Display for RejectionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::UnknownRepository => "selected Repository is unknown",
            Self::UnknownObservation => "selected observation is unknown",
            Self::WrongRepository => "selected observation belongs to another Repository",
            Self::IneligibleObservation => {
                "selected observation is unavailable, stale or unverified"
            }
            Self::ConflictingSelection => "conflicting selections for Repository",
            Self::DuplicateRepositoryIdentity => "duplicate catalog Repository identity",
            Self::ObservationRepositoryAbsent => {
                "observation Repository absent from catalog snapshot"
            }
            Self::DuplicateObservationIdentity(id) => {
                return write!(f, "duplicate observation identity: {}", id.as_str());
            }
        };
        f.write_str(message)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionSource {
    ExplicitTask,
    WorkspaceSelection,
    LocalDefault,
}
impl RejectedSelection {
    /// Snapshot validation failures have no selection or selection source.
    pub fn source(&self) -> Option<SelectionSource> {
        self.source
    }
    pub fn selection(&self) -> Option<&CheckoutSelection> {
        self.selection.as_ref()
    }
    pub fn reason(&self) -> &RejectionReason {
        &self.reason
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidTaskContext {
    rejections: Vec<RejectedSelection>,
}
impl InvalidTaskContext {
    pub fn rejections(&self) -> &[RejectedSelection] {
        &self.rejections
    }
}
impl std::fmt::Display for InvalidTaskContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, rejection) in self.rejections.iter().enumerate() {
            if i > 0 {
                f.write_str("; ")?;
            }
            write!(f, "{}", rejection.reason())?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidTaskContext {}

#[derive(Clone, Debug)]
pub struct ValidatedTaskContext {
    known: BTreeSet<RepositoryId>,
    observations: BTreeMap<ObservationId, CheckoutObservation>,
    selections: BTreeMap<RepositoryId, CheckoutSelection>,
    workspace: BTreeMap<RepositoryId, CheckoutSelection>,
    defaults: BTreeMap<RepositoryId, CheckoutSelection>,
    ignored: Vec<RejectedSelection>,
    working_area: Option<LocalPath>,
}
impl TaskContext {
    pub fn validate(
        &self,
        known: Vec<RepositoryId>,
        observations: Vec<CheckoutObservation>,
        workspace: Vec<CheckoutSelection>,
        defaults: Vec<CheckoutSelection>,
    ) -> Result<ValidatedTaskContext, InvalidTaskContext> {
        let mut errors = Vec::new();
        let known_set: BTreeSet<_> = known.iter().cloned().collect();
        if known_set.len() != known.len() {
            errors.push(RejectedSelection {
                source: None,
                selection: None,
                reason: RejectionReason::DuplicateRepositoryIdentity,
            });
        }
        let mut observed = BTreeMap::new();
        for observation in observations {
            if !known_set.contains(observation.repository()) {
                errors.push(RejectedSelection {
                    source: None,
                    selection: None,
                    reason: RejectionReason::ObservationRepositoryAbsent,
                });
            }
            let id = observation.id().to_owned();
            if observed.insert(id.clone(), observation).is_some() {
                errors.push(RejectedSelection {
                    source: None,
                    selection: None,
                    reason: RejectionReason::DuplicateObservationIdentity(id),
                });
            }
        }
        let (selections, rejected) = normalize(
            &self.selections,
            SelectionSource::ExplicitTask,
            &known_set,
            &observed,
        );
        errors.extend(rejected);
        let (workspace, mut ignored) = normalize(
            &workspace,
            SelectionSource::WorkspaceSelection,
            &known_set,
            &observed,
        );
        let (defaults, rejected) = normalize(
            &defaults,
            SelectionSource::LocalDefault,
            &known_set,
            &observed,
        );
        ignored.extend(rejected);
        if !errors.is_empty() {
            return Err(InvalidTaskContext { rejections: errors });
        }
        Ok(ValidatedTaskContext {
            known: known_set,
            observations: observed,
            selections,
            workspace,
            defaults,
            ignored,
            working_area: self.working_area.clone(),
        })
    }
}

// A conflicting preference is removed as a whole, never first/last-wins.
fn normalize(
    input: &[CheckoutSelection],
    source: SelectionSource,
    known: &BTreeSet<RepositoryId>,
    observations: &BTreeMap<ObservationId, CheckoutObservation>,
) -> (
    BTreeMap<RepositoryId, CheckoutSelection>,
    Vec<RejectedSelection>,
) {
    let mut accepted: BTreeMap<RepositoryId, CheckoutSelection> = BTreeMap::new();
    let mut conflicted = BTreeSet::new();
    let mut rejected = Vec::new();
    for selection in input {
        let reason = if !known.contains(selection.repository()) {
            Some(RejectionReason::UnknownRepository)
        } else if let Some(observation) = observations.get(selection.observation_id()) {
            if observation.repository() != selection.repository() {
                Some(RejectionReason::WrongRepository)
            } else if !observation.eligible() {
                Some(RejectionReason::IneligibleObservation)
            } else if conflicted.contains(selection.repository())
                || accepted
                    .get(selection.repository())
                    .is_some_and(|old| old != selection)
            {
                Some(RejectionReason::ConflictingSelection)
            } else {
                None
            }
        } else {
            Some(RejectionReason::UnknownObservation)
        };
        if let Some(reason) = reason {
            conflicted.insert(selection.repository().clone());
            accepted.remove(selection.repository());
            rejected.push(RejectedSelection {
                source: Some(source),
                selection: Some(selection.clone()),
                reason,
            });
        } else {
            accepted.insert(selection.repository().clone(), selection.clone());
        }
    }
    (accepted, rejected)
}

impl ValidatedTaskContext {
    pub fn working_area(&self) -> Option<&LocalPath> {
        self.working_area.as_ref()
    }
    pub fn observations(&self) -> &BTreeMap<ObservationId, CheckoutObservation> {
        &self.observations
    }
    pub fn ignored_preferences(&self) -> &[RejectedSelection] {
        &self.ignored
    }
    pub(crate) fn known(&self, repository: &RepositoryId) -> bool {
        self.known.contains(repository)
    }
    pub(crate) fn preferences(&self, repository: &RepositoryId) -> [Option<&CheckoutSelection>; 3] {
        [
            self.selections.get(repository),
            self.workspace.get(repository),
            self.defaults.get(repository),
        ]
    }
}
