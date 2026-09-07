use crate::{CheckoutObservation, LocalContextError, RejectedSelection, ValidatedTaskContext};
use devmeld_shared_kernel::RepositoryId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolutionBasis {
    ExplicitTask,
    WorkspaceSelection,
    LocalDefault,
    SoleCandidate,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Resolution {
    Resolved(ResolvedCheckout),
    Ambiguous(UnresolvedCheckout),
    Unavailable(UnresolvedCheckout),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedCheckout {
    repository: RepositoryId,
    selected: CheckoutObservation,
    basis: ResolutionBasis,
    considered: Vec<CheckoutObservation>,
    ignored: Vec<RejectedSelection>,
}
impl ResolvedCheckout {
    pub fn repository(&self) -> &RepositoryId {
        &self.repository
    }
    pub fn selected(&self) -> &CheckoutObservation {
        &self.selected
    }
    pub fn basis(&self) -> ResolutionBasis {
        self.basis
    }
    pub fn considered(&self) -> &[CheckoutObservation] {
        &self.considered
    }
    pub fn ignored_preferences(&self) -> &[RejectedSelection] {
        &self.ignored
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnresolvedCheckout {
    repository: RepositoryId,
    considered: Vec<CheckoutObservation>,
    reason: String,
    ignored: Vec<RejectedSelection>,
}
impl UnresolvedCheckout {
    pub fn repository(&self) -> &RepositoryId {
        &self.repository
    }
    pub fn considered(&self) -> &[CheckoutObservation] {
        &self.considered
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
    pub fn ignored_preferences(&self) -> &[RejectedSelection] {
        &self.ignored
    }
}
impl ValidatedTaskContext {
    pub fn resolve(&self, repository: &RepositoryId) -> Result<Resolution, LocalContextError> {
        if !self.known(repository) {
            return Err(LocalContextError::UnknownRepository);
        }
        let considered: Vec<_> = self
            .observations()
            .values()
            .filter(|o| o.repository() == repository)
            .cloned()
            .collect();
        let ignored: Vec<_> = self
            .ignored_preferences()
            .iter()
            .filter(|r| r.selection().is_some_and(|s| s.repository() == repository))
            .cloned()
            .collect();
        let eligible: Vec<_> = considered.iter().filter(|o| o.eligible()).collect();
        let bases = [
            ResolutionBasis::ExplicitTask,
            ResolutionBasis::WorkspaceSelection,
            ResolutionBasis::LocalDefault,
        ];
        for (preference, basis) in self.preferences(repository).into_iter().zip(bases) {
            if let Some(selection) = preference {
                if let Some(selected) = eligible
                    .iter()
                    .find(|o| o.id() == selection.observation_id())
                {
                    return Ok(Resolution::Resolved(ResolvedCheckout {
                        repository: repository.clone(),
                        selected: (*selected).clone(),
                        basis,
                        considered,
                        ignored,
                    }));
                }
            }
        }
        if eligible.len() == 1 {
            return Ok(Resolution::Resolved(ResolvedCheckout {
                repository: repository.clone(),
                selected: eligible[0].clone(),
                basis: ResolutionBasis::SoleCandidate,
                considered,
                ignored,
            }));
        }
        let absent = eligible.is_empty();
        let result = UnresolvedCheckout {
            repository: repository.clone(),
            considered,
            ignored,
            reason: if absent {
                "no available and fresh observation"
            } else {
                "multiple eligible observations without a decisive selection"
            }
            .into(),
        };
        Ok(if absent {
            Resolution::Unavailable(result)
        } else {
            Resolution::Ambiguous(result)
        })
    }
}
