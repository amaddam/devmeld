use crate::{
    Evidence, KnowledgeError, ReviewStatus, Scope, SourceFact, SourceType, ValidityStatus, text,
};
use devmeld_shared_kernel::{RepositoryId, ResourceId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectId {
    Repository(RepositoryId),
    Resource(ResourceId),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationKind {
    References,
    DependsOn,
    Explains,
    Affects,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationIdentity {
    Explicit(String),
    Derived {
        value: String,
        method_revision: String,
        inputs: Vec<String>,
    },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TargetAvailability {
    Available,
    Unavailable(String),
    Unknown(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Relation {
    identity: RelationIdentity,
    from: ObjectId,
    kind: RelationKind,
    target: ObjectId,
    source: SourceFact,
    evidence: Vec<Evidence>,
    scope: Scope,
    review: ReviewStatus,
    validity: ValidityStatus,
}
impl Relation {
    pub fn new(
        identity: RelationIdentity,
        from: ObjectId,
        kind: RelationKind,
        target: ObjectId,
        source: SourceFact,
        evidence: Vec<Evidence>,
        scope: Scope,
        review: ReviewStatus,
        validity: ValidityStatus,
    ) -> Result<Self, KnowledgeError> {
        match &identity {
            RelationIdentity::Explicit(value) => text(value, "Relation identity")?,
            RelationIdentity::Derived {
                value,
                method_revision,
                inputs,
            } => {
                text(value, "derived Relation identity")?;
                text(method_revision, "identity derivation revision")?;
                if inputs.is_empty() {
                    return Err(KnowledgeError::InvalidText("identity derivation inputs"));
                }
                for input in inputs {
                    text(input, "identity derivation input")?;
                }
            }
        }
        if source.source_type() != SourceType::Declared && evidence.is_empty() {
            return Err(KnowledgeError::MissingEvidence);
        }
        Ok(Self {
            identity,
            from,
            kind,
            target,
            source,
            evidence,
            scope,
            review,
            validity,
        })
    }
    pub fn identity(&self) -> &RelationIdentity {
        &self.identity
    }
    pub fn source_object(&self) -> &ObjectId {
        &self.from
    }
    pub fn target(&self) -> &ObjectId {
        &self.target
    }
    pub fn kind(&self) -> RelationKind {
        self.kind
    }
    pub fn source(&self) -> &SourceFact {
        &self.source
    }
    pub fn evidence(&self) -> &[Evidence] {
        &self.evidence
    }
    pub fn scope(&self) -> &Scope {
        &self.scope
    }
    pub fn review(&self) -> ReviewStatus {
        self.review
    }
    pub fn validity(&self) -> ValidityStatus {
        self.validity
    }
}
