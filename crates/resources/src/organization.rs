//! Logical organization is independent of resource identity and filesystem paths.
use crate::ResourceId;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidPath(String),
    OccupiedPath(String),
    DuplicateIdentity(String),
    MissingGroup(String),
    MissingResource(String),
    MissingNode(String),
    NonemptyGroup(String),
    SelfDescendantMove(String),
    InvalidAnnotation(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAnnotation(reason) => write!(f, "invalid context annotation: {reason}"),
            Self::InvalidPath(path) => write!(
                f,
                "invalid organization path {path:?}: use nonempty /-separated logical names, not filesystem paths or dot segments"
            ),
            Self::OccupiedPath(path) => write!(f, "organization path already occupied: {path}"),
            Self::DuplicateIdentity(id) => write!(f, "resource identity already registered: {id}"),
            Self::MissingGroup(path) => write!(f, "group is not registered: {path}"),
            Self::MissingResource(path) => write!(f, "resource is not registered: {path}"),
            Self::MissingNode(path) => write!(f, "organization node is not registered: {path}"),
            Self::NonemptyGroup(path) => write!(
                f,
                "group is not empty: {path}; move or remove its members explicitly"
            ),
            Self::SelfDescendantMove(path) => {
                write!(f, "cannot move a group into its own descendant: {path}")
            }
        }
    }
}
impl std::error::Error for Error {}

/// Explicit organization information, never source attributes or permissions.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LocalAnnotations {
    description: Option<String>,
    tags: BTreeSet<String>,
    fields: BTreeMap<String, String>,
}

impl LocalAnnotations {
    pub fn new(
        description: Option<String>,
        tags: Vec<String>,
        fields: Vec<(String, String)>,
    ) -> Result<Self, Error> {
        let text_valid = |value: &str| {
            !value
                .chars()
                .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
        };
        let name_valid = |value: &str| {
            !value.is_empty() && value.trim() == value && !value.chars().any(char::is_control)
        };
        if description
            .as_ref()
            .is_some_and(|value| value.trim().is_empty() || !text_valid(value))
        {
            return Err(Error::InvalidAnnotation(
                "description must be nonblank text without control codes".into(),
            ));
        }
        if tags.iter().any(|tag| !name_valid(tag)) {
            return Err(Error::InvalidAnnotation(
                "tags must be nonblank names without control codes or surrounding whitespace"
                    .into(),
            ));
        }
        let mut named = BTreeMap::new();
        for (key, value) in fields {
            if !name_valid(&key) || key.contains('=') || !text_valid(&value) {
                return Err(Error::InvalidAnnotation("field keys must be nonblank names without =; values must be text without control codes".into()));
            }
            if named.insert(key.clone(), value).is_some() {
                return Err(Error::InvalidAnnotation(format!("duplicate field {key:?}")));
            }
        }
        Ok(Self {
            description,
            tags: tags.into_iter().collect(),
            fields: named,
        })
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn tags(&self) -> impl Iterator<Item = &String> {
        self.tags.iter()
    }
    pub fn fields(&self) -> &BTreeMap<String, String> {
        &self.fields
    }
    pub fn is_empty(&self) -> bool {
        self.description.is_none() && self.tags.is_empty() && self.fields.is_empty()
    }
}

/// A local annotation change; omitted information is preserved on update.
pub struct AnnotationEdit {
    values: LocalAnnotations,
    clear_description: bool,
    remove_tags: BTreeSet<String>,
    remove_fields: BTreeSet<String>,
}

impl AnnotationEdit {
    pub fn new(
        values: LocalAnnotations,
        clear_description: bool,
        remove_tags: Vec<String>,
        remove_fields: Vec<String>,
    ) -> Result<Self, Error> {
        LocalAnnotations::new(
            None,
            remove_tags.clone(),
            remove_fields
                .iter()
                .map(|key| (key.clone(), String::new()))
                .collect(),
        )?;
        let remove_tags: BTreeSet<_> = remove_tags.into_iter().collect();
        let remove_fields: BTreeSet<_> = remove_fields.into_iter().collect();
        if clear_description && values.description().is_some() {
            return Err(Error::InvalidAnnotation(
                "cannot set and clear description together".into(),
            ));
        }
        if values.tags().any(|tag| remove_tags.contains(tag))
            || values
                .fields()
                .keys()
                .any(|key| remove_fields.contains(key))
        {
            return Err(Error::InvalidAnnotation(
                "cannot set and remove the same annotation in one change".into(),
            ));
        }
        Ok(Self {
            values,
            clear_description,
            remove_tags,
            remove_fields,
        })
    }

    pub fn for_creation(self) -> Result<LocalAnnotations, Error> {
        if self.clear_description || !self.remove_tags.is_empty() || !self.remove_fields.is_empty()
        {
            return Err(Error::InvalidAnnotation(
                "annotation removal requires an existing node".into(),
            ));
        }
        Ok(self.values)
    }

    pub fn apply(&self, original: &LocalAnnotations) -> Result<LocalAnnotations, Error> {
        let description = if self.clear_description {
            None
        } else {
            self.values
                .description()
                .or(original.description())
                .map(str::to_owned)
        };
        let tags = original
            .tags()
            .filter(|tag| !self.remove_tags.contains(*tag))
            .chain(self.values.tags())
            .cloned()
            .collect();
        let mut fields = original.fields().clone();
        for key in &self.remove_fields {
            fields.remove(key);
        }
        fields.extend(self.values.fields().clone());
        LocalAnnotations::new(description, tags, fields.into_iter().collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrganizationPath(String);

impl OrganizationPath {
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.split('/').any(|part| {
            part.is_empty()
                || matches!(part, "." | "..")
                || part.trim() != part
                || part
                    .chars()
                    .any(|c| c.is_control() || matches!(c, '\\' | ':'))
        }) {
            return Err(Error::InvalidPath(value));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn parent(&self) -> Option<Self> {
        self.0
            .rsplit_once('/')
            .map(|(parent, _)| Self(parent.into()))
    }

    pub fn leaf(&self) -> &str {
        self.0.rsplit('/').next().unwrap_or(&self.0)
    }

    pub fn is_descendant_of(&self, parent: &Self) -> bool {
        self.0
            .strip_prefix(&parent.0)
            .is_some_and(|suffix| suffix.starts_with('/'))
    }

    fn relocated(&self, from: &Self, to: &Self) -> Self {
        if self == from {
            return to.clone();
        }
        if self.is_descendant_of(from) {
            // The validated prefix ends at a logical segment boundary.
            Self(format!("{}{}", to.0, &self.0[from.0.len()..]))
        } else {
            self.clone()
        }
    }

    fn ancestors(&self) -> Vec<Self> {
        let mut result = Vec::new();
        let mut parent = self.parent();
        while let Some(path) = parent {
            parent = path.parent();
            result.push(path);
        }
        result.reverse();
        result
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Organization {
    defaults: InheritanceDefaults,
    groups: BTreeMap<OrganizationPath, OrganizedGroup>,
    resources: BTreeMap<OrganizationPath, OrganizedResource>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrganizedResource {
    id: ResourceId,
    annotations: LocalAnnotations,
    inherit: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrganizedGroup {
    annotations: LocalAnnotations,
    inherit: bool,
    propagate: bool,
}

impl Default for OrganizedGroup {
    fn default() -> Self {
        Self::new(InheritanceDefaults::default())
    }
}

impl OrganizedGroup {
    fn new(defaults: InheritanceDefaults) -> Self {
        Self {
            annotations: LocalAnnotations::default(),
            inherit: defaults.inherit,
            propagate: defaults.propagate,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InheritanceDefaults {
    inherit: bool,
    propagate: bool,
}

impl Default for InheritanceDefaults {
    fn default() -> Self {
        Self::new(false, true)
    }
}

impl InheritanceDefaults {
    pub fn new(inherit: bool, propagate: bool) -> Self {
        Self { inherit, propagate }
    }
    pub fn inherit(self) -> bool {
        self.inherit
    }
    pub fn propagate(self) -> bool {
        self.propagate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldOrigin {
    value: String,
    origin: OrganizationPath,
}

impl FieldOrigin {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn origin(&self) -> &OrganizationPath {
        &self.origin
    }
}

/// A computed view, never copied into local declarations. Tags retain every
/// contributing origin; a local field replaces the inherited value and origin.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EffectiveAnnotations {
    tags: BTreeMap<String, BTreeSet<OrganizationPath>>,
    fields: BTreeMap<String, FieldOrigin>,
}

impl EffectiveAnnotations {
    pub fn tags(&self) -> &BTreeMap<String, BTreeSet<OrganizationPath>> {
        &self.tags
    }
    pub fn fields(&self) -> &BTreeMap<String, FieldOrigin> {
        &self.fields
    }
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty() && self.fields.is_empty()
    }
}

impl Organization {
    /// Restore explicit declarations without silently creating missing parents.
    pub fn from_parts(
        groups: Vec<OrganizationPath>,
        resources: Vec<(OrganizationPath, ResourceId)>,
    ) -> Result<Self, Error> {
        let mut result = Self::default();
        for group in groups {
            if result
                .groups
                .insert(group.clone(), OrganizedGroup::default())
                .is_some()
            {
                return Err(Error::OccupiedPath(group.0));
            }
        }
        for path in result
            .groups
            .keys()
            .chain(resources.iter().map(|(path, _)| path))
        {
            if let Some(parent) = path.parent() {
                if !result.groups.contains_key(&parent) {
                    return Err(Error::MissingGroup(parent.0));
                }
            }
        }
        for (path, id) in resources {
            result.register(id, path)?;
        }
        Ok(result)
    }

    pub fn register(
        &mut self,
        id: ResourceId,
        path: OrganizationPath,
    ) -> Result<Vec<OrganizationPath>, Error> {
        if self.groups.contains_key(&path) || self.resources.contains_key(&path) {
            return Err(Error::OccupiedPath(path.0));
        }
        if self.resources.values().any(|existing| existing.id == id) {
            return Err(Error::DuplicateIdentity(id.as_str().into()));
        }
        let created = self.parents_to_create(&path)?;
        self.groups.extend(
            created
                .iter()
                .cloned()
                .map(|path| (path, OrganizedGroup::new(self.defaults))),
        );
        self.resources.insert(
            path,
            OrganizedResource {
                id,
                annotations: LocalAnnotations::default(),
                inherit: self.defaults.inherit,
            },
        );
        Ok(created)
    }

    fn parents_to_create(&self, path: &OrganizationPath) -> Result<Vec<OrganizationPath>, Error> {
        let parents = path.ancestors();
        for parent in &parents {
            if self.resources.contains_key(parent) {
                return Err(Error::OccupiedPath(parent.0.clone()));
            }
        }
        Ok(parents
            .into_iter()
            .filter(|parent| !self.groups.contains_key(parent))
            .collect())
    }

    pub fn add_group(&mut self, path: OrganizationPath) -> Result<Vec<OrganizationPath>, Error> {
        if self.groups.contains_key(&path) || self.resources.contains_key(&path) {
            return Err(Error::OccupiedPath(path.0));
        }
        let mut created = self.parents_to_create(&path)?;
        created.push(path);
        self.groups.extend(
            created
                .iter()
                .cloned()
                .map(|path| (path, OrganizedGroup::new(self.defaults))),
        );
        Ok(created)
    }

    pub fn remove_group(&mut self, path: &OrganizationPath) -> Result<(), Error> {
        if !self.groups.contains_key(path) {
            return Err(Error::MissingGroup(path.0.clone()));
        }
        if self
            .groups
            .keys()
            .chain(self.resources.keys())
            .any(|child| child.is_descendant_of(path))
        {
            return Err(Error::NonemptyGroup(path.0.clone()));
        }
        self.groups.remove(path);
        Ok(())
    }

    pub fn move_group(
        &mut self,
        from: &OrganizationPath,
        to: OrganizationPath,
    ) -> Result<(), Error> {
        if !self.groups.contains_key(from) {
            return Err(Error::MissingGroup(from.0.clone()));
        }
        if from == &to {
            return Ok(());
        }
        if to.is_descendant_of(from) {
            return Err(Error::SelfDescendantMove(to.0));
        }
        if self.groups.contains_key(&to) || self.resources.contains_key(&to) {
            return Err(Error::OccupiedPath(to.0));
        }
        let parents = self.parents_to_create(&to)?;
        let groups = self
            .groups
            .iter()
            .map(|(path, group)| (path.relocated(from, &to), group.clone()))
            .chain(
                parents
                    .into_iter()
                    .map(|path| (path, OrganizedGroup::new(self.defaults))),
            )
            .collect();
        let resources = self
            .resources
            .iter()
            .map(|(path, resource)| (path.relocated(from, &to), resource.clone()))
            .collect();
        // An absent destination cannot have descendants in this validated tree.
        // Readdress whole nodes so local metadata and saved choices move together.
        let next = Self {
            groups,
            resources,
            defaults: self.defaults,
        };
        *self = next;
        Ok(())
    }

    pub fn resource_at(&self, path: &OrganizationPath) -> Option<&ResourceId> {
        self.resources.get(path).map(|resource| &resource.id)
    }

    pub fn move_resource(
        &mut self,
        from: &OrganizationPath,
        to: OrganizationPath,
    ) -> Result<(), Error> {
        let resource = self
            .resources
            .get(from)
            .cloned()
            .ok_or_else(|| Error::MissingResource(from.0.clone()))?;
        if from == &to {
            return Ok(());
        }
        for parent in to.ancestors() {
            if self.resources.contains_key(&parent) {
                return Err(Error::OccupiedPath(parent.0));
            }
        }
        let mut next = self.clone();
        next.resources.remove(from);
        next.register(resource.id.clone(), to.clone())?;
        next.resources.insert(to, resource);
        *self = next;
        Ok(())
    }

    pub fn path_for(&self, id: &ResourceId) -> Option<&OrganizationPath> {
        self.resources
            .iter()
            .find_map(|(path, value)| (&value.id == id).then_some(path))
    }

    pub fn groups(&self) -> impl Iterator<Item = &OrganizationPath> {
        self.groups.keys()
    }

    pub fn resources(&self) -> impl Iterator<Item = (&OrganizationPath, &ResourceId)> {
        self.resources
            .iter()
            .map(|(path, resource)| (path, &resource.id))
    }

    pub fn annotations(&self, path: &OrganizationPath) -> Option<&LocalAnnotations> {
        self.groups
            .get(path)
            .map(|group| &group.annotations)
            .or_else(|| {
                self.resources
                    .get(path)
                    .map(|resource| &resource.annotations)
            })
    }

    pub fn set_annotations(
        &mut self,
        path: &OrganizationPath,
        annotations: LocalAnnotations,
    ) -> Result<(), Error> {
        if let Some(group) = self.groups.get_mut(path) {
            group.annotations = annotations;
        } else if let Some(resource) = self.resources.get_mut(path) {
            resource.annotations = annotations;
        } else {
            return Err(Error::MissingNode(path.0.clone()));
        }
        Ok(())
    }

    pub fn inherits(&self, path: &OrganizationPath) -> Option<bool> {
        self.groups
            .get(path)
            .map(|group| group.inherit)
            .or_else(|| self.resources.get(path).map(|resource| resource.inherit))
    }

    pub fn defaults(&self) -> InheritanceDefaults {
        self.defaults
    }

    /// Creation-time policy only; existing node choices are never rewritten.
    pub fn set_defaults(&mut self, defaults: InheritanceDefaults) {
        self.defaults = defaults;
    }

    pub fn propagates(&self, path: &OrganizationPath) -> Option<bool> {
        self.groups.get(path).map(|group| group.propagate)
    }

    pub fn set_inherit(&mut self, path: &OrganizationPath, inherit: bool) -> Result<(), Error> {
        if let Some(group) = self.groups.get_mut(path) {
            group.inherit = inherit;
        } else if let Some(resource) = self.resources.get_mut(path) {
            resource.inherit = inherit;
        } else {
            return Err(Error::MissingNode(path.0.clone()));
        }
        Ok(())
    }

    pub fn set_propagate(&mut self, path: &OrganizationPath, propagate: bool) -> Result<(), Error> {
        self.groups
            .get_mut(path)
            .ok_or_else(|| Error::MissingGroup(path.0.clone()))?
            .propagate = propagate;
        Ok(())
    }

    pub fn effective_annotations(
        &self,
        path: &OrganizationPath,
    ) -> Result<EffectiveAnnotations, Error> {
        let mut result = EffectiveAnnotations::default();
        let mut parent_propagates = false;
        for node in path
            .ancestors()
            .into_iter()
            .chain(std::iter::once(path.clone()))
        {
            let local = self
                .annotations(&node)
                .ok_or_else(|| Error::MissingNode(node.0.clone()))?;
            if !parent_propagates || self.inherits(&node) != Some(true) {
                result = EffectiveAnnotations::default();
            }
            for tag in local.tags() {
                result
                    .tags
                    .entry(tag.clone())
                    .or_default()
                    .insert(node.clone());
            }
            for (key, value) in local.fields() {
                result.fields.insert(
                    key.clone(),
                    FieldOrigin {
                        value: value.clone(),
                        origin: node.clone(),
                    },
                );
            }
            parent_propagates = self.propagates(&node).unwrap_or(false);
        }
        Ok(result)
    }
}
