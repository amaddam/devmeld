//! Application inputs: product operations, never command-line tokens.
use crate::{OutputLanguage, Result};
use devmeld_resources::organization::{AnnotationEdit, Organization, OrganizationPath};
use std::path::PathBuf;

/// Relative native inputs use this absolute base, independently of context selection.
/// An omitted directory discovers the nearest marker; an explicit directory does not.
pub struct ContextLocation {
    pub base_directory: PathBuf,
    pub directory: Option<PathBuf>,
}
impl ContextLocation {
    /// Select an absolute context directory and also use it as the native-input base.
    pub fn at(directory: impl Into<PathBuf>) -> Self {
        let directory = directory.into();
        Self {
            base_directory: directory.clone(),
            directory: Some(directory),
        }
    }
    /// Discover from an absolute input base; never read the process working directory.
    pub fn discover_from(directory: impl Into<PathBuf>) -> Self {
        Self {
            base_directory: directory.into(),
            directory: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Resource,
    Group,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Instructions,
}
impl EntryKind {
    pub(crate) fn stored(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Instructions => "instructions",
        }
    }
}

pub enum ResourceSource {
    Document(PathBuf),
    Description {
        file: PathBuf,
        schema: Option<PathBuf>,
    },
}

/// Omission preserves an existing choice; creation resolves saved defaults.
#[derive(Default)]
pub struct NodeEdit {
    pub annotations: Option<AnnotationEdit>,
    pub inherit: Option<bool>,
    pub propagate: Option<bool>,
}
impl NodeEdit {
    pub(crate) fn apply_choices(
        &self,
        organization: &mut Organization,
        path: &OrganizationPath,
    ) -> Result<()> {
        if let Some(value) = self.inherit {
            organization.set_inherit(path, value)?;
        }
        if let Some(value) = self.propagate {
            organization.set_propagate(path, value)?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct InitOptions {
    pub output: Option<PathBuf>,
    pub entries: Vec<(EntryKind, PathBuf)>,
    pub language: Option<OutputLanguage>,
}

pub struct RegisterResource {
    pub source: ResourceSource,
    pub address: Option<OrganizationPath>,
    pub edit: NodeEdit,
}

#[derive(Clone, Copy)]
pub enum DefaultSetting {
    Inherit,
    Propagate,
}

/// Prepare a local use case without writing. The caller may inspect or discard the plan;
/// only Plan::apply writes, and it always rechecks captured inputs and ownership.
pub enum Mutation {
    Initialize(InitOptions),
    RegisterResource(RegisterResource),
    CreateGroup {
        path: OrganizationPath,
        edit: NodeEdit,
    },
    UpdateNode {
        kind: NodeKind,
        path: OrganizationPath,
        edit: NodeEdit,
    },
    RemoveResource(OrganizationPath),
    RemoveGroup(OrganizationPath),
    MoveNode {
        kind: NodeKind,
        from: OrganizationPath,
        to: OrganizationPath,
    },
    SetLanguage(OutputLanguage),
    SetDefault {
        setting: DefaultSetting,
        value: bool,
    },
    RegisterEntry {
        kind: EntryKind,
        file: PathBuf,
    },
    RemoveEntry(PathBuf),
    SetOutput(PathBuf),
    Associate {
        resource: OrganizationPath,
        tool: OrganizationPath,
    },
    Dissociate {
        resource: OrganizationPath,
        tool: OrganizationPath,
    },
    Publish,
    Recover,
}

pub enum Query {
    List {
        kind: NodeKind,
        scope: Option<OrganizationPath>,
    },
    Show {
        kind: NodeKind,
        path: OrganizationPath,
    },
    PublicationStatus,
}
