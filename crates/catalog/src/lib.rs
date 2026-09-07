//! Portable project identity and composition.

mod profiles;
mod references;
mod registrations;
mod workspace;

pub use profiles::ContextProfile;
pub use references::{PortableLocator, SourceReference};
pub use registrations::{RepositoryRegistration, ResourceRegistration};
pub use workspace::{Workspace, WorkspaceId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CatalogError {
    InvalidText(&'static str),
    InvalidLocator(&'static str),
    UnsupportedSchema(u32),
    Duplicate(&'static str),
    WrongWorkspace,
    UnknownReference(&'static str),
    IneligibleResource,
    Referenced(&'static str),
}

impl std::fmt::Display for CatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidText(field) => write!(
                f,
                "invalid {field}: empty, boundary whitespace or control character"
            ),
            Self::InvalidLocator(reason) => {
                write!(f, "non-portable or unsupported locator: {reason}")
            }
            Self::UnsupportedSchema(version) => {
                write!(f, "unsupported catalog schema version {version}")
            }
            Self::Duplicate(field) => write!(f, "duplicate or conflicting {field}"),
            Self::WrongWorkspace => f.write_str("registration belongs to another Workspace"),
            Self::UnknownReference(field) => write!(f, "unknown {field}"),
            Self::IneligibleResource => f.write_str("profile selects an ineligible Resource"),
            Self::Referenced(field) => {
                write!(f, "{field} is still referenced by a Context Profile")
            }
        }
    }
}
impl std::error::Error for CatalogError {}

pub(crate) fn text(value: &str, field: &'static str) -> Result<(), CatalogError> {
    if value.is_empty() || value.trim() != value || value.chars().any(char::is_control) {
        Err(CatalogError::InvalidText(field))
    } else {
        Ok(())
    }
}
