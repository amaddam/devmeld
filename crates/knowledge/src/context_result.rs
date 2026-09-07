use crate::{KnowledgeError, Relation, ResourceFact, Scope, ScopeMatch, TargetAvailability, text};
use devmeld_shared_kernel::RepositoryId;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckoutBasis {
    ExplicitTask,
    WorkspaceSelection,
    LocalDefault,
    SoleCandidate,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkingTreeState {
    Clean,
    Dirty(String),
    Unknown(String),
}
#[derive(Clone, Debug)]
pub struct CheckoutFactInput {
    pub repository: RepositoryId,
    pub observation: String,
    pub branch: Option<String>,
    pub revision: Option<String>,
    pub working_tree: WorkingTreeState,
    pub observed_at: SystemTime,
    pub observer_revision: String,
    pub basis: CheckoutBasis,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckoutFacts {
    repository: RepositoryId,
    observation: String,
    branch: Option<String>,
    revision: Option<String>,
    working_tree: WorkingTreeState,
    observed_at: SystemTime,
    observer_revision: String,
    basis: CheckoutBasis,
}
impl CheckoutFacts {
    pub fn new(input: CheckoutFactInput) -> Result<Self, KnowledgeError> {
        text(&input.observation, "Checkout observation")?;
        match &input.working_tree {
            WorkingTreeState::Clean => {}
            WorkingTreeState::Dirty(reason) | WorkingTreeState::Unknown(reason) => {
                text(reason, "working-tree explanation")?;
            }
        }
        text(&input.observer_revision, "observer revision")?;
        for value in [&input.branch, &input.revision].into_iter().flatten() {
            text(value, "Checkout revision")?;
        }
        Ok(Self {
            repository: input.repository,
            observation: input.observation,
            branch: input.branch,
            revision: input.revision,
            working_tree: input.working_tree,
            observed_at: input.observed_at,
            observer_revision: input.observer_revision,
            basis: input.basis,
        })
    }
    pub fn repository(&self) -> &RepositoryId {
        &self.repository
    }
    pub fn observation(&self) -> &str {
        &self.observation
    }
    pub fn branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }
    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }
    pub fn working_tree(&self) -> &WorkingTreeState {
        &self.working_tree
    }
    pub fn observed_at(&self) -> SystemTime {
        self.observed_at
    }
    pub fn observer_revision(&self) -> &str {
        &self.observer_revision
    }
    pub fn basis(&self) -> CheckoutBasis {
        self.basis
    }
}
fn validate_checkout(
    query: &Scope,
    checkout: Option<&CheckoutFacts>,
) -> Result<(), KnowledgeError> {
    let Some(checkout) = checkout else {
        return if query.checkout().is_some() || query.revision().is_some() {
            Err(KnowledgeError::MissingCheckout)
        } else {
            Ok(())
        };
    };
    if query.repository() != Some(checkout.repository()) {
        return Err(KnowledgeError::InconsistentCheckout("Repository"));
    }
    if query
        .checkout()
        .is_some_and(|value| value != checkout.observation())
    {
        return Err(KnowledgeError::InconsistentCheckout("observation"));
    }
    if query.revision().is_some() && query.revision() != checkout.revision() {
        return Err(KnowledgeError::InconsistentCheckout("revision"));
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceResult {
    fact: ResourceFact,
    query: Scope,
    scope_match: ScopeMatch,
    checkout: Option<CheckoutFacts>,
    relevance: Option<String>,
}
impl ResourceResult {
    pub fn fact(&self) -> &ResourceFact {
        &self.fact
    }
    pub fn query(&self) -> &Scope {
        &self.query
    }
    pub fn scope_match(&self) -> &ScopeMatch {
        &self.scope_match
    }
    pub fn checkout(&self) -> Option<&CheckoutFacts> {
        self.checkout.as_ref()
    }
    pub fn relevance(&self) -> Option<&str> {
        self.relevance.as_deref()
    }
}
impl ResourceFact {
    pub fn evaluate(
        &self,
        query: Scope,
        checkout: Option<CheckoutFacts>,
        relevance: Option<String>,
    ) -> Result<ResourceResult, KnowledgeError> {
        validate_checkout(&query, checkout.as_ref())?;
        if let Some(relevance) = &relevance {
            text(relevance, "relevance explanation")?;
        }
        Ok(ResourceResult {
            fact: self.clone(),
            scope_match: self.scope().match_query(&query),
            query,
            checkout,
            relevance,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationResult {
    fact: Relation,
    query: Scope,
    scope_match: ScopeMatch,
    checkout: Option<CheckoutFacts>,
    target_availability: TargetAvailability,
    relevance: Option<String>,
}
impl RelationResult {
    pub fn fact(&self) -> &Relation {
        &self.fact
    }
    pub fn query(&self) -> &Scope {
        &self.query
    }
    pub fn scope_match(&self) -> &ScopeMatch {
        &self.scope_match
    }
    pub fn checkout(&self) -> Option<&CheckoutFacts> {
        self.checkout.as_ref()
    }
    pub fn target_availability(&self) -> &TargetAvailability {
        &self.target_availability
    }
    pub fn relevance(&self) -> Option<&str> {
        self.relevance.as_deref()
    }
}
impl Relation {
    pub fn evaluate(
        &self,
        query: Scope,
        checkout: Option<CheckoutFacts>,
        target_availability: TargetAvailability,
        relevance: Option<String>,
    ) -> Result<RelationResult, KnowledgeError> {
        validate_checkout(&query, checkout.as_ref())?;
        match &target_availability {
            TargetAvailability::Available => {}
            TargetAvailability::Unavailable(reason) | TargetAvailability::Unknown(reason) => {
                text(reason, "target availability explanation")?
            }
        }
        if let Some(relevance) = &relevance {
            text(relevance, "relevance explanation")?;
        }
        Ok(RelationResult {
            fact: self.clone(),
            scope_match: self.scope().match_query(&query),
            query,
            checkout,
            target_availability,
            relevance,
        })
    }
}
