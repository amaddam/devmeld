use crate::{CatalogError, PortableLocator, SourceReference, WorkspaceId, text};
use devmeld_shared_kernel::{RepositoryId, ResourceId};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositoryRegistration {
    workspace: WorkspaceId,
    id: RepositoryId,
    canonical_key: String,
    display_name: String,
    aliases: Vec<String>,
    source: Option<PortableLocator>,
}
impl RepositoryRegistration {
    pub fn new(
        workspace: WorkspaceId,
        id: RepositoryId,
        key: impl Into<String>,
        name: impl Into<String>,
        aliases: Vec<String>,
        source: Option<PortableLocator>,
    ) -> Result<Self, CatalogError> {
        let value = Self {
            workspace,
            id,
            canonical_key: key.into(),
            display_name: name.into(),
            aliases,
            source,
        };
        text(&value.canonical_key, "canonical key")?;
        text(&value.display_name, "display name")?;
        let mut names = BTreeSet::from([value.canonical_key.as_str()]);
        for alias in &value.aliases {
            text(alias, "alias")?;
            if !names.insert(alias) {
                return Err(CatalogError::Duplicate("Repository name"));
            }
        }
        Ok(value)
    }
    pub fn with_labels(
        &self,
        name: impl Into<String>,
        aliases: Vec<String>,
    ) -> Result<Self, CatalogError> {
        Self::new(
            self.workspace.clone(),
            self.id.clone(),
            self.canonical_key.clone(),
            name,
            aliases,
            self.source.clone(),
        )
    }
    pub fn id(&self) -> &RepositoryId {
        &self.id
    }
    pub fn workspace(&self) -> &WorkspaceId {
        &self.workspace
    }
    pub fn canonical_key(&self) -> &str {
        &self.canonical_key
    }
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }
    pub fn declared_source(&self) -> Option<&PortableLocator> {
        self.source.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceRegistration {
    workspace: WorkspaceId,
    id: ResourceId,
    source: SourceReference,
    resource_type: String,
    locator: PortableLocator,
    eligible: bool,
}
impl ResourceRegistration {
    pub fn new(
        workspace: WorkspaceId,
        id: ResourceId,
        source: SourceReference,
        resource_type: impl Into<String>,
        locator: PortableLocator,
        eligible: bool,
    ) -> Result<Self, CatalogError> {
        let resource_type = resource_type.into();
        text(&resource_type, "Resource type")?;
        Ok(Self {
            workspace,
            id,
            source,
            resource_type,
            locator,
            eligible,
        })
    }
    pub fn workspace(&self) -> &WorkspaceId {
        &self.workspace
    }
    pub fn id(&self) -> &ResourceId {
        &self.id
    }
    pub fn source(&self) -> &SourceReference {
        &self.source
    }
    pub fn resource_type(&self) -> &str {
        &self.resource_type
    }
    pub fn locator(&self) -> &PortableLocator {
        &self.locator
    }
    pub fn eligible(&self) -> bool {
        self.eligible
    }
}
