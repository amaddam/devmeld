use std::fmt;

/// The spelling is opaque: no case folding, trimming or Unicode normalization.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RepositoryId(String);

/// A Resource identity cannot be substituted for a Repository identity.
///
/// ```compile_fail,E0308
/// use devmeld_shared_kernel::{RepositoryId, ResourceId};
/// let resource = ResourceId::new("same-spelling").unwrap();
/// let repository: RepositoryId = resource;
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceId(String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityError {
    Empty,
    BoundaryWhitespace,
    ControlCharacter,
}

impl fmt::Display for IdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Empty => "identity must not be empty",
            Self::BoundaryWhitespace => "identity must not have boundary whitespace",
            Self::ControlCharacter => "identity must not contain control characters",
        })
    }
}

impl std::error::Error for IdentityError {}

// This helper stays private to the two identities whose validation is identical.
fn validate(value: &str) -> Result<(), IdentityError> {
    if value.is_empty() {
        return Err(IdentityError::Empty);
    }
    if value.trim() != value {
        return Err(IdentityError::BoundaryWhitespace);
    }
    if value.chars().any(char::is_control) {
        return Err(IdentityError::ControlCharacter);
    }
    Ok(())
}

impl RepositoryId {
    pub fn new(value: impl AsRef<str>) -> Result<Self, IdentityError> {
        let value = value.as_ref();
        validate(value)?;
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ResourceId {
    pub fn new(value: impl AsRef<str>) -> Result<Self, IdentityError> {
        let value = value.as_ref();
        validate(value)?;
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepositoryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
