//! Local observations, task validation, and explicit Checkout resolution.

mod bindings;
mod observations;
mod resolution;
mod task_context;

pub use bindings::{LocalBinding, LocalBindingRegistry, LocalPath, PathDialect};
pub use observations::{
    Availability, CheckoutObservation, Freshness, ObservationInput, WorkingTree,
};
pub use resolution::{Resolution, ResolutionBasis, ResolvedCheckout, UnresolvedCheckout};
pub use task_context::{
    CheckoutSelection, InvalidTaskContext, RejectedSelection, TaskContext, ValidatedTaskContext,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalContextError {
    InvalidText(&'static str),
    InvalidPath,
    UnknownRepository,
    UnknownBinding,
    WrongRepository,
    ConflictingIdentity,
    RevisionOverflow,
}
impl std::fmt::Display for LocalContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidText(field) => write!(f, "invalid {field}"),
            Self::InvalidPath => f.write_str("path is not absolute in the declared local dialect"),
            Self::UnknownRepository => {
                f.write_str("Repository is absent from supplied catalog identities")
            }
            Self::UnknownBinding => f.write_str("local binding is unknown"),
            Self::WrongRepository => {
                f.write_str("binding or preference belongs to another Repository")
            }
            Self::ConflictingIdentity => {
                f.write_str("identity is duplicated or would be reassigned")
            }
            Self::RevisionOverflow => f.write_str("registry revision cannot advance"),
        }
    }
}
impl std::error::Error for LocalContextError {}
pub(crate) fn text(value: &str, field: &'static str) -> Result<(), LocalContextError> {
    if value.is_empty() || value.trim() != value || value.chars().any(char::is_control) {
        Err(LocalContextError::InvalidText(field))
    } else {
        Ok(())
    }
}
