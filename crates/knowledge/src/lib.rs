//! Provenance and query-time context applicability.

mod context_result;
mod provenance;
mod relations;
mod scope;

pub use context_result::{
    CheckoutBasis, CheckoutFactInput, CheckoutFacts, RelationResult, ResourceResult,
};
pub use provenance::{
    Evidence, EvidenceSupport, ResourceFact, ReviewStatus, SourceFact, SourceType, ValidityStatus,
};
pub use relations::{ObjectId, Relation, RelationIdentity, RelationKind, TargetAvailability};
pub use scope::{DimensionMatch, MatchState, Scope, ScopeDimension, ScopeInput, ScopeMatch};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KnowledgeError {
    InvalidText(&'static str),
    InvalidRange,
    MissingEvidence,
    MissingCheckout,
    InconsistentCheckout(&'static str),
}
impl std::fmt::Display for KnowledgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidText(field) => write!(f, "invalid {field}"),
            Self::InvalidRange => f.write_str("Evidence line range must be positive and ordered"),
            Self::MissingEvidence => {
                f.write_str("observed or inferred claim needs supporting Evidence")
            }
            Self::MissingCheckout => {
                f.write_str("query claims Checkout/revision without resolution facts")
            }
            Self::InconsistentCheckout(field) => {
                write!(f, "Checkout facts do not support query {field}")
            }
        }
    }
}
impl std::error::Error for KnowledgeError {}
pub(crate) fn text(value: &str, field: &'static str) -> Result<(), KnowledgeError> {
    if value.is_empty() || value.trim() != value || value.chars().any(char::is_control) {
        Err(KnowledgeError::InvalidText(field))
    } else {
        Ok(())
    }
}
