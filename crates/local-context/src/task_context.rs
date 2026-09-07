use crate::{CheckoutObservation, LocalContextError, LocalPath, text};
use devmeld_shared_kernel::RepositoryId;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckoutSelection {
    repository: RepositoryId,
    observation_id: String,
}
impl CheckoutSelection {
    pub fn new(
        repository: RepositoryId,
        observation_id: impl Into<String>,
    ) -> Result<Self, LocalContextError> {
        let observation_id = observation_id.into();
        text(&observation_id, "selected observation identity")?;
        Ok(Self {
            repository,
            observation_id,
        })
    }
    pub fn repository(&self) -> &RepositoryId {
        &self.repository
    }
    pub fn observation_id(&self) -> &str {
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
    selection: Option<CheckoutSelection>,
    reason: String,
}
impl RejectedSelection {
    pub fn selection(&self) -> Option<&CheckoutSelection> {
        self.selection.as_ref()
    }
    pub fn reason(&self) -> &str {
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
            f.write_str(rejection.reason())?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidTaskContext {}

#[derive(Clone, Debug)]
pub struct ValidatedTaskContext {
    known: BTreeSet<RepositoryId>,
    observations: BTreeMap<String, CheckoutObservation>,
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
                selection: None,
                reason: "duplicate catalog Repository identity".into(),
            });
        }
        let mut observed = BTreeMap::new();
        for observation in observations {
            if !known_set.contains(observation.repository()) {
                errors.push(RejectedSelection {
                    selection: None,
                    reason: "observation Repository absent from catalog snapshot".into(),
                });
            }
            let id = observation.id().to_owned();
            if observed.insert(id.clone(), observation).is_some() {
                errors.push(RejectedSelection {
                    selection: None,
                    reason: format!("duplicate observation identity: {id}"),
                });
            }
        }
        let (selections, rejected) = normalize(&self.selections, &known_set, &observed);
        errors.extend(rejected);
        let (workspace, mut ignored) = normalize(&workspace, &known_set, &observed);
        let (defaults, rejected) = normalize(&defaults, &known_set, &observed);
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
    known: &BTreeSet<RepositoryId>,
    observations: &BTreeMap<String, CheckoutObservation>,
) -> (
    BTreeMap<RepositoryId, CheckoutSelection>,
    Vec<RejectedSelection>,
) {
    let mut accepted: BTreeMap<RepositoryId, CheckoutSelection> = BTreeMap::new();
    let mut conflicted = BTreeSet::new();
    let mut rejected = Vec::new();
    for selection in input {
        let reason = if !known.contains(selection.repository()) {
            Some("selected Repository is unknown")
        } else if let Some(observation) = observations.get(selection.observation_id()) {
            if observation.repository() != selection.repository() {
                Some("selected observation belongs to another Repository")
            } else if !observation.eligible() {
                Some("selected observation is unavailable, stale or unverified")
            } else if conflicted.contains(selection.repository())
                || accepted
                    .get(selection.repository())
                    .is_some_and(|old| old != selection)
            {
                Some("conflicting selections for Repository")
            } else {
                None
            }
        } else {
            Some("selected observation is unknown")
        };
        if let Some(reason) = reason {
            conflicted.insert(selection.repository().clone());
            accepted.remove(selection.repository());
            rejected.push(RejectedSelection {
                selection: Some(selection.clone()),
                reason: reason.into(),
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
    pub fn observations(&self) -> &BTreeMap<String, CheckoutObservation> {
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
