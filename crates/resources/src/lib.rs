//! Resource Organization: pure registration and access-association rules.
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidId(String),
    EmptyField(&'static str),
    Duplicate(String),
    Missing(String),
    Referenced(String),
    AssociationExists,
    AssociationMissing,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidId(id) => write!(
                f,
                "invalid resource ID {id:?}: use a lowercase portable slug (1..64 bytes)"
            ),
            Self::EmptyField(field) => write!(f, "empty resource {field}"),
            Self::Duplicate(id) => write!(f, "duplicate resource: {id}"),
            Self::Missing(id) => write!(f, "resource not registered: {id}"),
            Self::Referenced(id) => write!(
                f,
                "remove access associations explicitly before removing resource: {id}"
            ),
            Self::AssociationExists => f.write_str("access association already exists"),
            Self::AssociationMissing => f.write_str("access association does not exist"),
        }
    }
}
impl std::error::Error for Error {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceId(String);
impl ResourceId {
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 64
            || !value
                .bytes()
                .next()
                .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            || !value
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-' || c == b'_')
        {
            return Err(Error::InvalidId(value));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    id: ResourceId,
    source: PathBuf,
    title: String,
    summary: String,
    attributes: BTreeMap<String, String>,
    references: Vec<(String, PathBuf)>,
}
impl Resource {
    pub fn document(id: ResourceId, source: PathBuf) -> Result<Self, Error> {
        if source.as_os_str().is_empty() {
            return Err(Error::EmptyField("source"));
        }
        let title = id.as_str().to_owned();
        Ok(Self {
            id,
            source,
            title,
            summary: "Original document".into(),
            attributes: BTreeMap::new(),
            references: vec![],
        })
    }
    pub fn described(
        id: ResourceId,
        source: PathBuf,
        title: String,
        summary: String,
        attributes: BTreeMap<String, String>,
        references: Vec<(String, PathBuf)>,
    ) -> Result<Self, Error> {
        for (name, value) in [("title", title.as_str()), ("summary", summary.as_str())] {
            if value.trim().is_empty() {
                return Err(Error::EmptyField(name));
            }
        }
        if references
            .iter()
            .any(|(label, path)| label.trim().is_empty() || path.as_os_str().is_empty())
        {
            return Err(Error::EmptyField("reference"));
        }
        let mut resource = Self::document(id, source)?;
        resource.title = title;
        resource.summary = summary;
        resource.attributes = attributes;
        resource.references = references;
        Ok(resource)
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn summary(&self) -> &str {
        &self.summary
    }
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }
    pub fn references(&self) -> &[(String, PathBuf)] {
        &self.references
    }
    pub fn id(&self) -> &ResourceId {
        &self.id
    }
    pub fn source(&self) -> &std::path::Path {
        &self.source
    }
}

#[derive(Default)]
pub struct Resources {
    items: BTreeMap<ResourceId, Resource>,
}
impl Resources {
    pub fn add(&mut self, resource: Resource) -> Result<(), Error> {
        if self.items.contains_key(resource.id()) {
            return Err(Error::Duplicate(resource.id().as_str().into()));
        }
        self.items.insert(resource.id.clone(), resource);
        Ok(())
    }
    pub fn iter(&self) -> impl Iterator<Item = &Resource> {
        self.items.values()
    }
}

/// Membership can be maintained even when an authored source needs repair.
#[derive(Default)]
pub struct Membership {
    ids: BTreeSet<ResourceId>,
    access: BTreeSet<(ResourceId, ResourceId)>,
}
impl Membership {
    pub fn register(&mut self, id: ResourceId) -> Result<(), Error> {
        if !self.ids.insert(id.clone()) {
            return Err(Error::Duplicate(id.0));
        }
        Ok(())
    }
    pub fn associate(&mut self, resource: ResourceId, tool: ResourceId) -> Result<(), Error> {
        for id in [&resource, &tool] {
            if !self.ids.contains(id) {
                return Err(Error::Missing(id.0.clone()));
            }
        }
        if !self.access.insert((resource, tool)) {
            return Err(Error::AssociationExists);
        }
        Ok(())
    }
    pub fn dissociate(&mut self, resource: ResourceId, tool: ResourceId) -> Result<(), Error> {
        if !self.access.remove(&(resource, tool)) {
            return Err(Error::AssociationMissing);
        }
        Ok(())
    }
    pub fn unregister(&mut self, id: &ResourceId) -> Result<(), Error> {
        if self.access.iter().any(|(r, t)| r == id || t == id) {
            return Err(Error::Referenced(id.0.clone()));
        }
        if !self.ids.remove(id) {
            return Err(Error::Missing(id.0.clone()));
        }
        Ok(())
    }
}

/// Product policy wording, independent of rendering or an execution engine.
pub const ACCESS_GUIDANCE: &str = "These are declared associations, not exclusive selections or verified availability. Respect applicable explicit selection; do not silently substitute. Otherwise prefer an eligible existing option requiring no dependency, environment or installation change: project-managed first, then verified local/system options valid in the current task scope. Only then propose a change. Machine presence is not eligibility. Selection does not authorize installation, dependency changes, environment creation, system modification or execution beyond the task's authority.";
