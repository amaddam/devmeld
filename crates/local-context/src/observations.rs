use crate::{LocalContextError, LocalPath, text};
use devmeld_shared_kernel::RepositoryId;
use std::time::SystemTime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Availability {
    Available,
    Unavailable(String),
    Unknown(String),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Freshness {
    Fresh,
    Stale(String),
    Unknown(String),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkingTree {
    Clean,
    Dirty(String),
    Unknown(String),
}

/// Untrusted construction input. The validated observation owns its snapshot.
#[derive(Clone, Debug)]
pub struct ObservationInput {
    pub id: String,
    pub repository: RepositoryId,
    pub local_path: LocalPath,
    pub branch: Option<String>,
    pub commit: Option<String>,
    pub working_tree: WorkingTree,
    pub observed_at: SystemTime,
    pub observer_revision: String,
    pub availability: Availability,
    pub freshness: Freshness,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckoutObservation {
    id: String,
    repository: RepositoryId,
    local_path: LocalPath,
    branch: Option<String>,
    commit: Option<String>,
    working_tree: WorkingTree,
    observed_at: SystemTime,
    observer_revision: String,
    availability: Availability,
    freshness: Freshness,
}
impl CheckoutObservation {
    pub fn new(input: ObservationInput) -> Result<Self, LocalContextError> {
        text(&input.id, "observation identity")?;
        text(&input.observer_revision, "observer revision")?;
        for value in [&input.branch, &input.commit].into_iter().flatten() {
            text(value, "observed revision")?;
        }
        match &input.working_tree {
            WorkingTree::Clean => {}
            WorkingTree::Dirty(reason) | WorkingTree::Unknown(reason) => {
                text(reason, "working-tree explanation")?
            }
        }
        match &input.availability {
            Availability::Available => {}
            Availability::Unavailable(reason) | Availability::Unknown(reason) => {
                text(reason, "availability explanation")?
            }
        }
        match &input.freshness {
            Freshness::Fresh => {}
            Freshness::Stale(reason) | Freshness::Unknown(reason) => {
                text(reason, "freshness explanation")?
            }
        }
        Ok(Self {
            id: input.id,
            repository: input.repository,
            local_path: input.local_path,
            branch: input.branch,
            commit: input.commit,
            working_tree: input.working_tree,
            observed_at: input.observed_at,
            observer_revision: input.observer_revision,
            availability: input.availability,
            freshness: input.freshness,
        })
    }
    pub fn eligible(&self) -> bool {
        self.availability == Availability::Available && self.freshness == Freshness::Fresh
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn repository(&self) -> &RepositoryId {
        &self.repository
    }
    pub fn local_path(&self) -> &LocalPath {
        &self.local_path
    }
    pub fn branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }
    pub fn commit(&self) -> Option<&str> {
        self.commit.as_deref()
    }
    pub fn working_tree(&self) -> &WorkingTree {
        &self.working_tree
    }
    pub fn observed_at(&self) -> SystemTime {
        self.observed_at
    }
    pub fn observer_revision(&self) -> &str {
        &self.observer_revision
    }
    pub fn availability(&self) -> &Availability {
        &self.availability
    }
    pub fn freshness(&self) -> &Freshness {
        &self.freshness
    }
}
