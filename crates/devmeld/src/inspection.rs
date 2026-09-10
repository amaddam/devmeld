//! Structured, read-only application views. No terminal formatting or argument parsing.
use crate::{
    ContextLocation, EntryKind, NodeKind, Plan, Query, ResourceSource, Result, context,
    declarations, error,
};
use devmeld_resources::{
    ResourceId,
    organization::{
        EffectiveAnnotations, InheritanceDefaults, LocalAnnotations, Organization, OrganizationPath,
    },
};
use std::path::PathBuf;

/// A query result, not a source of truth or a public wire protocol.
/// Registration queries do not open source files or claim that a client consumed them.
pub struct Inspection {
    pub context_root: PathBuf,
    pub defaults: InheritanceDefaults,
    pub view: InspectionView,
}
pub enum InspectionView {
    List {
        kind: NodeKind,
        paths: Vec<OrganizationPath>,
    },
    Group(GroupDetails),
    Resource(ResourceDetails),
    Publication(PublicationStatus),
}
pub struct NodeAnnotations {
    pub local: LocalAnnotations,
    pub inherit: bool,
    pub propagate: Option<bool>,
    pub effective: EffectiveAnnotations,
}
pub struct ResourceDetails {
    pub id: ResourceId,
    pub address: OrganizationPath,
    /// Declared references, relative to Inspection.context_root when not absolute.
    pub source: ResourceSource,
    pub annotations: NodeAnnotations,
    pub access_guidance: Vec<OrganizationPath>,
    pub used_by: Vec<OrganizationPath>,
}
pub struct GroupDetails {
    pub path: OrganizationPath,
    pub annotations: NodeAnnotations,
    pub child_groups: Vec<OrganizationPath>,
    pub resources: Vec<OrganizationPath>,
}
/// Current expected-content comparison, not a historical source-freshness receipt.
/// This does not verify Agent Client consumption, even when no targets are pending.
pub struct PublicationStatus {
    pub resource_count: usize,
    pub entries: Vec<EntryStatus>,
    pub pending: bool,
    pub navigation: PathBuf,
    pub pending_targets: Vec<PathBuf>,
}
pub struct EntryStatus {
    pub path: PathBuf,
    pub kind: EntryKind,
    pub state: EntryPublicationState,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryPublicationState {
    Unpublished,
    PendingUpdate,
    Published,
}

pub fn inspect(location: &ContextLocation, query: Query) -> Result<Inspection> {
    let status = matches!(query, Query::PublicationStatus);
    inspect_query(location, query).map_err(|cause| {
        if status {
            error(format!("publication status blocked/unverified: {cause}"))
        } else {
            cause
        }
    })
}
fn inspect_query(location: &ContextLocation, query: Query) -> Result<Inspection> {
    let root = context::select(&location.base_directory, location.directory.as_deref())?;
    let mut snapshot = Plan::new(root.clone())?;
    let config = declarations::read_config(&mut snapshot)?;
    let organization = config.organization()?;
    let view = match query {
        Query::List { kind, scope } => {
            if let Some(scope) = &scope {
                require_group(&organization, scope)?;
            }
            let included = |path: &OrganizationPath| {
                scope
                    .as_ref()
                    .is_none_or(|scope| path.is_descendant_of(scope))
            };
            let paths = match kind {
                NodeKind::Resource => organization
                    .resources()
                    .filter(|(path, _)| included(path))
                    .map(|(path, _)| path.clone())
                    .collect(),
                NodeKind::Group => organization
                    .groups()
                    .filter(|path| included(path))
                    .cloned()
                    .collect(),
            };
            InspectionView::List { kind, paths }
        }
        Query::Show {
            kind: NodeKind::Resource,
            path,
        } => InspectionView::Resource(resource_details(&config, &organization, path)?),
        Query::Show {
            kind: NodeKind::Group,
            path,
        } => {
            require_group(&organization, &path)?;
            InspectionView::Group(GroupDetails {
                annotations: node_annotations(&organization, &path)?,
                child_groups: organization
                    .groups()
                    .filter(|child| child.parent().as_ref() == Some(&path))
                    .cloned()
                    .collect(),
                resources: organization
                    .resources()
                    .filter(|(child, _)| child.parent().as_ref() == Some(&path))
                    .map(|(child, _)| child.clone())
                    .collect(),
                path,
            })
        }
        Query::PublicationStatus => {
            InspectionView::Publication(publication_status(&mut snapshot, &config)?)
        }
    };
    snapshot.recheck()?;
    Ok(Inspection {
        context_root: root,
        defaults: InheritanceDefaults::new(config.defaults.inherit, config.defaults.propagate),
        view,
    })
}
fn publication_status(plan: &mut Plan, config: &declarations::Config) -> Result<PublicationStatus> {
    crate::application::prepare_publication(plan, config)?;
    let entries = config
        .publication
        .entries
        .iter()
        .map(|entry| {
            let path = crate::storage::resolve(plan.root(), &entry.path)?;
            let owned = plan.owned_paths().any(|p| p == &path);
            let changed = plan.changed_paths().any(|p| p == &path);
            let state = match (owned, changed) {
                (false, _) => EntryPublicationState::Unpublished,
                (true, true) => EntryPublicationState::PendingUpdate,
                (true, false) => EntryPublicationState::Published,
            };
            let kind = match entry.kind.as_str() {
                "file" => EntryKind::File,
                "instructions" => EntryKind::Instructions,
                _ => return Err(error("unsupported entry kind")),
            };
            Ok(EntryStatus { path, kind, state })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(PublicationStatus {
        resource_count: config.resources.len(),
        entries,
        pending: !plan.is_empty(),
        navigation: crate::storage::resolve(plan.root(), &config.publication.directory)?
            .join("index.md"),
        pending_targets: plan.changed_paths().cloned().collect(),
    })
}
fn require_group(organization: &Organization, group: &OrganizationPath) -> Result<()> {
    if organization.groups().any(|path| path == group) {
        Ok(())
    } else {
        Err(error(format!("group not registered: {}", group.as_str())))
    }
}
fn node_annotations(
    organization: &Organization,
    path: &OrganizationPath,
) -> Result<NodeAnnotations> {
    Ok(NodeAnnotations {
        local: organization
            .annotations(path)
            .ok_or_else(|| error("missing node annotations"))?
            .clone(),
        inherit: organization
            .inherits(path)
            .ok_or_else(|| error("missing inheritance choice"))?,
        propagate: organization.propagates(path),
        effective: organization.effective_annotations(path)?,
    })
}
fn resource_details(
    config: &declarations::Config,
    organization: &Organization,
    address: OrganizationPath,
) -> Result<ResourceDetails> {
    let id = config.resource_id(address.as_str())?;
    let registration = config
        .resources
        .iter()
        .find(|r| r.id == id.as_str())
        .ok_or_else(|| error("missing resource registration"))?;
    let source = match (&registration.document, &registration.description) {
        (Some(file), None) => ResourceSource::Document(file.into()),
        (None, Some(file)) => ResourceSource::Description {
            file: file.into(),
            schema: registration.attributes_schema.as_ref().map(PathBuf::from),
        },
        _ => return Err(error("expected exactly one source reference")),
    };
    let associations = |incoming| -> Result<Vec<OrganizationPath>> {
        let mut paths = std::collections::BTreeSet::new();
        for association in &config.access {
            let (owner, other) = if incoming {
                (&association.tool, &association.resource)
            } else {
                (&association.resource, &association.tool)
            };
            if owner == id.as_str() {
                let other_id = ResourceId::new(other.clone())?;
                paths.insert(
                    organization
                        .path_for(&other_id)
                        .ok_or_else(|| error("association refers to an unregistered resource"))?
                        .clone(),
                );
            }
        }
        Ok(paths.into_iter().collect())
    };
    Ok(ResourceDetails {
        access_guidance: associations(false)?,
        used_by: associations(true)?,
        annotations: node_annotations(organization, &address)?,
        id,
        address,
        source,
    })
}
