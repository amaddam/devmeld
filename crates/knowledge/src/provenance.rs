use crate::{KnowledgeError, Scope, text};
use devmeld_shared_kernel::ResourceId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceType {
    Declared,
    Observed,
    Inferred,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReviewStatus {
    Unreviewed,
    Accepted,
    Rejected,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidityStatus {
    Current,
    Superseded,
    Invalid,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceFact {
    identity: String,
    revision: String,
    source_type: SourceType,
}
impl SourceFact {
    pub fn new(
        identity: impl Into<String>,
        revision: impl Into<String>,
        source_type: SourceType,
    ) -> Result<Self, KnowledgeError> {
        let identity = identity.into();
        let revision = revision.into();
        text(&identity, "source identity")?;
        text(&revision, "source revision")?;
        Ok(Self {
            identity,
            revision,
            source_type,
        })
    }
    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub fn revision(&self) -> &str {
        &self.revision
    }
    pub fn source_type(&self) -> SourceType {
        self.source_type
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceSupport {
    Locator {
        locator: String,
        lines: Option<(u32, u32)>,
    },
    Derivation(String),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evidence {
    source: SourceFact,
    support: EvidenceSupport,
    content_hash: Option<String>,
}
impl Evidence {
    pub fn new(
        source: SourceFact,
        support: EvidenceSupport,
        content_hash: Option<String>,
    ) -> Result<Self, KnowledgeError> {
        match &support {
            EvidenceSupport::Locator { locator, lines } => {
                text(locator, "Evidence locator")?;
                if lines.is_some_and(|(start, end)| start == 0 || start > end) {
                    return Err(KnowledgeError::InvalidRange);
                }
            }
            EvidenceSupport::Derivation(description) => text(description, "Evidence derivation")?,
        }
        if let Some(hash) = &content_hash {
            text(hash, "content hash")?;
        }
        Ok(Self {
            source,
            support,
            content_hash,
        })
    }
    pub fn source(&self) -> &SourceFact {
        &self.source
    }
    pub fn support(&self) -> &EvidenceSupport {
        &self.support
    }
    pub fn content_hash(&self) -> Option<&str> {
        self.content_hash.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceFact {
    id: ResourceId,
    source: SourceFact,
    evidence: Vec<Evidence>,
    scope: Scope,
    review: ReviewStatus,
    validity: ValidityStatus,
}
impl ResourceFact {
    pub fn new(
        id: ResourceId,
        source: SourceFact,
        evidence: Vec<Evidence>,
        scope: Scope,
        review: ReviewStatus,
        validity: ValidityStatus,
    ) -> Result<Self, KnowledgeError> {
        if source.source_type() != SourceType::Declared && evidence.is_empty() {
            return Err(KnowledgeError::MissingEvidence);
        }
        Ok(Self {
            id,
            source,
            evidence,
            scope,
            review,
            validity,
        })
    }
    pub fn id(&self) -> &ResourceId {
        &self.id
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
