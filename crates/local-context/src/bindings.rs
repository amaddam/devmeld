use crate::{LocalContextError, text};
use devmeld_shared_kernel::RepositoryId;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathDialect {
    Windows,
    Posix,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalPath {
    value: String,
    dialect: PathDialect,
}
impl LocalPath {
    pub fn new(value: impl Into<String>, dialect: PathDialect) -> Result<Self, LocalContextError> {
        let value = value.into();
        text(&value, "local path")?;
        let bytes = value.as_bytes();
        let absolute = match dialect {
            PathDialect::Posix => value.starts_with('/') && !value.starts_with("//"),
            PathDialect::Windows => {
                (bytes.len() >= 3
                    && bytes[0].is_ascii_alphabetic()
                    && bytes[1] == b':'
                    && matches!(bytes[2], b'/' | b'\\'))
                    || (value.starts_with("\\\\")
                        && value[2..].split('\\').filter(|s| !s.is_empty()).count() >= 2)
            }
        };
        if !absolute {
            return Err(LocalContextError::InvalidPath);
        }
        Ok(Self { value, dialect })
    }
    pub fn as_str(&self) -> &str {
        &self.value
    }
    pub fn dialect(&self) -> PathDialect {
        self.dialect
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalBinding {
    id: String,
    repository: RepositoryId,
    path: LocalPath,
}
impl LocalBinding {
    pub fn new(
        id: impl Into<String>,
        repository: RepositoryId,
        path: LocalPath,
    ) -> Result<Self, LocalContextError> {
        let id = id.into();
        text(&id, "binding identity")?;
        Ok(Self {
            id,
            repository,
            path,
        })
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn repository(&self) -> &RepositoryId {
        &self.repository
    }
    pub fn path(&self) -> &LocalPath {
        &self.path
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalBindingRegistry {
    machine: String,
    developer: String,
    known_repositories: BTreeSet<RepositoryId>,
    bindings: BTreeMap<String, LocalBinding>,
    workspace_selections: BTreeMap<RepositoryId, String>,
    defaults: BTreeMap<RepositoryId, String>,
    revision: u64,
}
impl LocalBindingRegistry {
    pub fn new(
        machine: impl Into<String>,
        developer: impl Into<String>,
        known: Vec<RepositoryId>,
    ) -> Result<Self, LocalContextError> {
        let machine = machine.into();
        let developer = developer.into();
        text(&machine, "machine identity")?;
        text(&developer, "developer identity")?;
        let known_repositories: BTreeSet<_> = known.iter().cloned().collect();
        if known_repositories.len() != known.len() {
            return Err(LocalContextError::ConflictingIdentity);
        }
        Ok(Self {
            machine,
            developer,
            known_repositories,
            bindings: BTreeMap::new(),
            workspace_selections: BTreeMap::new(),
            defaults: BTreeMap::new(),
            revision: 0,
        })
    }
    fn advance(&mut self) -> Result<(), LocalContextError> {
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or(LocalContextError::RevisionOverflow)?;
        Ok(())
    }
    pub fn with_binding(&self, binding: LocalBinding) -> Result<Self, LocalContextError> {
        if !self.known_repositories.contains(binding.repository()) {
            return Err(LocalContextError::UnknownRepository);
        }
        if self
            .bindings
            .get(binding.id())
            .is_some_and(|old| old.repository() != binding.repository())
        {
            return Err(LocalContextError::ConflictingIdentity);
        }
        let mut next = self.clone();
        next.bindings.insert(binding.id().to_owned(), binding);
        next.advance()?;
        Ok(next)
    }
    fn validate_preference(
        &self,
        repository: &RepositoryId,
        binding: &str,
    ) -> Result<(), LocalContextError> {
        if !self.known_repositories.contains(repository) {
            return Err(LocalContextError::UnknownRepository);
        }
        if self
            .bindings
            .get(binding)
            .ok_or(LocalContextError::UnknownBinding)?
            .repository()
            != repository
        {
            return Err(LocalContextError::WrongRepository);
        }
        Ok(())
    }
    pub fn with_default(
        &self,
        repository: RepositoryId,
        binding: &str,
    ) -> Result<Self, LocalContextError> {
        self.validate_preference(&repository, binding)?;
        let mut next = self.clone();
        next.defaults.insert(repository, binding.into());
        next.advance()?;
        Ok(next)
    }
    pub fn with_workspace_selection(
        &self,
        repository: RepositoryId,
        binding: &str,
    ) -> Result<Self, LocalContextError> {
        self.validate_preference(&repository, binding)?;
        let mut next = self.clone();
        next.workspace_selections.insert(repository, binding.into());
        next.advance()?;
        Ok(next)
    }
    pub fn without_binding(&self, id: &str) -> Result<Self, LocalContextError> {
        let mut next = self.clone();
        next.bindings
            .remove(id)
            .ok_or(LocalContextError::UnknownBinding)?;
        next.defaults.retain(|_, binding| binding != id);
        next.workspace_selections.retain(|_, binding| binding != id);
        next.advance()?;
        Ok(next)
    }
    pub fn revision(&self) -> u64 {
        self.revision
    }
    pub fn machine(&self) -> &str {
        &self.machine
    }
    pub fn developer(&self) -> &str {
        &self.developer
    }
    pub fn bindings(&self) -> &BTreeMap<String, LocalBinding> {
        &self.bindings
    }
    pub fn default_bindings(&self) -> &BTreeMap<RepositoryId, String> {
        &self.defaults
    }
    pub fn workspace_selections(&self) -> &BTreeMap<RepositoryId, String> {
        &self.workspace_selections
    }
}
