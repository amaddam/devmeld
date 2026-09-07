use crate::{CatalogError, WorkspaceId, text};
use devmeld_shared_kernel::{RepositoryId, ResourceId};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextProfile {
    workspace: WorkspaceId,
    id: String,
    name: String,
    repositories: BTreeSet<RepositoryId>,
    resources: BTreeSet<ResourceId>,
}
impl ContextProfile {
    pub fn new(
        workspace: WorkspaceId,
        id: impl Into<String>,
        name: impl Into<String>,
        repositories: Vec<RepositoryId>,
        resources: Vec<ResourceId>,
    ) -> Result<Self, CatalogError> {
        let id = id.into();
        let name = name.into();
        text(&id, "Profile identity")?;
        text(&name, "Profile name")?;
        let repo_set: BTreeSet<_> = repositories.iter().cloned().collect();
        let resource_set: BTreeSet<_> = resources.iter().cloned().collect();
        if repo_set.len() != repositories.len() || resource_set.len() != resources.len() {
            return Err(CatalogError::Duplicate("Profile selection"));
        }
        Ok(Self {
            workspace,
            id,
            name,
            repositories: repo_set,
            resources: resource_set,
        })
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn workspace(&self) -> &WorkspaceId {
        &self.workspace
    }
    pub fn repositories(&self) -> &BTreeSet<RepositoryId> {
        &self.repositories
    }
    pub fn resources(&self) -> &BTreeSet<ResourceId> {
        &self.resources
    }
}
