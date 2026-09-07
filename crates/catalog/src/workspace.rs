use crate::{
    CatalogError, ContextProfile, RepositoryRegistration, ResourceRegistration, SourceReference,
    text,
};
use devmeld_shared_kernel::{RepositoryId, ResourceId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorkspaceId(String);
impl WorkspaceId {
    pub fn new(value: impl Into<String>) -> Result<Self, CatalogError> {
        let value = value.into();
        text(&value, "Workspace identity")?;
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Workspace {
    id: WorkspaceId,
    name: String,
    schema_version: u32,
    primary_vault: SourceReference,
    additional_sources: Vec<SourceReference>,
    repositories: BTreeMap<RepositoryId, RepositoryRegistration>,
    resources: BTreeMap<ResourceId, ResourceRegistration>,
    profiles: BTreeMap<String, ContextProfile>,
}
impl Workspace {
    pub fn new(
        id: WorkspaceId,
        name: impl Into<String>,
        schema_version: u32,
        primary_vault: SourceReference,
        additional_sources: Vec<SourceReference>,
    ) -> Result<Self, CatalogError> {
        let value = Self {
            id,
            name: name.into(),
            schema_version,
            primary_vault,
            additional_sources,
            repositories: BTreeMap::new(),
            resources: BTreeMap::new(),
            profiles: BTreeMap::new(),
        };
        value.validate()?;
        Ok(value)
    }
    fn validate(&self) -> Result<(), CatalogError> {
        text(&self.name, "Workspace name")?;
        if self.schema_version != 1 {
            return Err(CatalogError::UnsupportedSchema(self.schema_version));
        }
        let mut sources = BTreeMap::from([(self.primary_vault.id(), &self.primary_vault)]);
        for source in &self.additional_sources {
            if sources.insert(source.id(), source).is_some() {
                return Err(CatalogError::Duplicate("source identity"));
            }
        }
        let mut keys = BTreeSet::new();
        for repository in self.repositories.values() {
            if repository.workspace() != &self.id {
                return Err(CatalogError::WrongWorkspace);
            }
            for key in std::iter::once(repository.canonical_key())
                .chain(repository.aliases().iter().map(String::as_str))
            {
                if !keys.insert(key) {
                    return Err(CatalogError::Duplicate("Repository key/alias"));
                }
            }
        }
        for resource in self.resources.values() {
            if resource.workspace() != &self.id {
                return Err(CatalogError::WrongWorkspace);
            }
            if sources.get(resource.source().id()).copied() != Some(resource.source()) {
                return Err(CatalogError::UnknownReference(
                    "Resource source revision of reference",
                ));
            }
        }
        for profile in self.profiles.values() {
            if profile.workspace() != &self.id {
                return Err(CatalogError::WrongWorkspace);
            }
            for id in profile.repositories() {
                if !self.repositories.contains_key(id) {
                    return Err(CatalogError::UnknownReference("Profile Repository"));
                }
            }
            for id in profile.resources() {
                let resource = self
                    .resources
                    .get(id)
                    .ok_or(CatalogError::UnknownReference("Profile Resource"))?;
                if !resource.eligible() {
                    return Err(CatalogError::IneligibleResource);
                }
            }
        }
        Ok(())
    }
    // Each transition validates a candidate before exposing it. The original
    // remains a usable immutable snapshot, including after any failure.
    pub fn with_repository(&self, value: RepositoryRegistration) -> Result<Self, CatalogError> {
        let mut next = self.clone();
        next.repositories.insert(value.id().clone(), value);
        next.validate()?;
        Ok(next)
    }
    pub fn with_resource(&self, value: ResourceRegistration) -> Result<Self, CatalogError> {
        let mut next = self.clone();
        next.resources.insert(value.id().clone(), value);
        next.validate()?;
        Ok(next)
    }
    pub fn with_profile(&self, value: ContextProfile) -> Result<Self, CatalogError> {
        let mut next = self.clone();
        next.profiles.insert(value.id().to_owned(), value);
        next.validate()?;
        Ok(next)
    }
    pub fn without_repository(&self, id: &RepositoryId) -> Result<Self, CatalogError> {
        if self
            .profiles
            .values()
            .any(|p| p.repositories().contains(id))
        {
            return Err(CatalogError::Referenced("Repository"));
        }
        let mut next = self.clone();
        next.repositories
            .remove(id)
            .ok_or(CatalogError::UnknownReference("Repository"))?;
        Ok(next)
    }
    pub fn without_resource(&self, id: &ResourceId) -> Result<Self, CatalogError> {
        if self.profiles.values().any(|p| p.resources().contains(id)) {
            return Err(CatalogError::Referenced("Resource"));
        }
        let mut next = self.clone();
        next.resources
            .remove(id)
            .ok_or(CatalogError::UnknownReference("Resource"))?;
        Ok(next)
    }
    pub fn without_profile(&self, id: &str) -> Result<Self, CatalogError> {
        let mut next = self.clone();
        next.profiles
            .remove(id)
            .ok_or(CatalogError::UnknownReference("Profile"))?;
        Ok(next)
    }
    pub fn id(&self) -> &WorkspaceId {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
    pub fn primary_vault(&self) -> &SourceReference {
        &self.primary_vault
    }
    pub fn additional_sources(&self) -> &[SourceReference] {
        &self.additional_sources
    }
    pub fn repositories(&self) -> &BTreeMap<RepositoryId, RepositoryRegistration> {
        &self.repositories
    }
    pub fn resources(&self) -> &BTreeMap<ResourceId, ResourceRegistration> {
        &self.resources
    }
    pub fn profiles(&self) -> &BTreeMap<String, ContextProfile> {
        &self.profiles
    }
}
