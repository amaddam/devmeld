//! Use the library as a non-terminal client: no CLI parser or formatted reports.
use devmeld::{
    ContextLocation, Mutation, NodeEdit, NodeKind, Query, RegisterResource, ResourceSource,
};
use devmeld_resources::organization::OrganizationPath;
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "devmeld-application-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        Self(root)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn typed_client_registers_inspects_and_publishes_without_command_strings() {
    let f = Fixture::new();
    fs::write(f.0.join("notes.md"), "# Team notes\nAuthored input.\n").unwrap();
    let location = ContextLocation::at(&f.0);
    let path = OrganizationPath::new("knowledge/notes").unwrap();
    let plan = devmeld::prepare(
        &location,
        Mutation::RegisterResource(RegisterResource {
            source: ResourceSource::Document("notes.md".into()),
            address: Some(path.clone()),
            edit: NodeEdit::default(),
        }),
    )
    .unwrap();
    assert!(!f.0.join(".devmeld").exists());
    match plan.preview() {
        devmeld::PlanPreview::Changes { targets, .. } => {
            assert_eq!(targets.len(), 1);
            assert_eq!(
                targets[0].path,
                plan.context_root().join(".devmeld/context.toml")
            );
            assert!(targets[0].before.is_none());
            assert!(targets[0].after.is_some());
        }
        _ => panic!("expected a prepared registration"),
    }
    plan.apply().unwrap();
    let result = devmeld::inspect(
        &location,
        Query::Show {
            kind: NodeKind::Resource,
            path,
        },
    )
    .unwrap();
    let devmeld::InspectionView::Resource(resource) = result.view else {
        panic!("expected resource facts")
    };
    assert_eq!(resource.address.as_str(), "knowledge/notes");
    assert_eq!(resource.id.as_str(), "resource-1");
    assert!(resource.access_guidance.is_empty());
    assert!(!f.0.join(".devmeld/output").exists());
    devmeld::prepare(&location, Mutation::Publish)
        .unwrap()
        .apply()
        .unwrap();
    let status = devmeld::inspect(&location, Query::PublicationStatus).unwrap();
    let devmeld::InspectionView::Publication(status) = status.view else {
        panic!("expected publication facts")
    };
    assert!(!status.pending);
    assert_eq!(status.resource_count, 1);
    assert!(status.pending_targets.is_empty());
    assert!(status.navigation.is_file());
    assert!(
        devmeld::prepare(&location, Mutation::Publish)
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        fs::read_to_string(f.0.join("notes.md")).unwrap(),
        "# Team notes\nAuthored input.\n"
    );
}

fn path(value: &str) -> OrganizationPath {
    OrganizationPath::new(value).unwrap()
}
fn save(location: &ContextLocation, request: Mutation) {
    devmeld::prepare(location, request)
        .unwrap()
        .apply()
        .unwrap();
}

#[test]
fn typed_queries_keep_inheritance_origins_and_associations_without_reading_sources() {
    use devmeld_resources::organization::{AnnotationEdit, LocalAnnotations};
    let f = Fixture::new();
    let location = ContextLocation::at(&f.0);
    let annotations = AnnotationEdit::new(
        LocalAnnotations::new(
            Some("Team rules".into()),
            vec!["backend".into()],
            vec![("environment".into(), "test".into())],
        )
        .unwrap(),
        false,
        vec![],
        vec![],
    )
    .unwrap();
    save(
        &location,
        Mutation::CreateGroup {
            path: path("team"),
            edit: NodeEdit {
                annotations: Some(annotations),
                propagate: Some(true),
                ..Default::default()
            },
        },
    );
    fs::write(f.0.join("source.md"), "ssh HTTP").unwrap();
    for address in ["team/notes", "tools/query"] {
        save(
            &location,
            Mutation::RegisterResource(RegisterResource {
                source: ResourceSource::Document("source.md".into()),
                address: Some(path(address)),
                edit: NodeEdit {
                    inherit: Some(true),
                    ..Default::default()
                },
            }),
        );
    }
    save(
        &location,
        Mutation::Associate {
            resource: path("team/notes"),
            tool: path("tools/query"),
        },
    );
    save(
        &location,
        Mutation::MoveNode {
            kind: NodeKind::Group,
            from: path("team"),
            to: path("archive/team"),
        },
    );
    fs::remove_file(f.0.join("source.md")).unwrap();
    let before = fs::read(f.0.join(".devmeld/context.toml")).unwrap();
    let result = devmeld::inspect(
        &location,
        Query::Show {
            kind: NodeKind::Resource,
            path: path("archive/team/notes"),
        },
    )
    .unwrap();
    let devmeld::InspectionView::Resource(resource) = result.view else {
        panic!("expected resource")
    };
    assert_eq!(resource.id.as_str(), "resource-1");
    assert_eq!(resource.access_guidance, [path("tools/query")]);
    assert!(resource.annotations.local.is_empty());
    assert!(resource.annotations.inherit);
    let field = &resource.annotations.effective.fields()["environment"];
    assert_eq!(field.value(), "test");
    assert_eq!(field.origin(), &path("archive/team"));
    assert!(resource.annotations.effective.tags()["backend"].contains(&path("archive/team")));
    let list = devmeld::inspect(
        &location,
        Query::List {
            kind: NodeKind::Group,
            scope: Some(path("archive")),
        },
    )
    .unwrap();
    let devmeld::InspectionView::List { paths, .. } = list.view else {
        panic!("expected list")
    };
    assert_eq!(paths, [path("archive/team")]);
    assert!(devmeld::inspect(&location, Query::PublicationStatus).is_err());
    assert_eq!(fs::read(f.0.join(".devmeld/context.toml")).unwrap(), before);
}

#[test]
fn nonterminal_clients_cannot_apply_stale_plans_even_when_publication_was_unchanged() {
    let f = Fixture::new();
    let location = ContextLocation::at(&f.0);
    fs::write(f.0.join("source.md"), "first").unwrap();
    let register = || {
        Mutation::RegisterResource(RegisterResource {
            source: ResourceSource::Document("source.md".into()),
            address: Some(path("notes")),
            edit: NodeEdit::default(),
        })
    };
    drop(devmeld::prepare(&location, register()).unwrap());
    assert!(!f.0.join(".devmeld").exists());
    let plan = devmeld::prepare(&location, register()).unwrap();
    fs::write(f.0.join("source.md"), "changed").unwrap();
    assert!(plan.apply().unwrap_err().to_string().contains("stale"));
    assert!(!f.0.join(".devmeld").exists());
    save(&location, register());
    save(&location, Mutation::Publish);
    let plan = devmeld::prepare(&location, Mutation::Publish).unwrap();
    assert!(plan.is_empty());
    let output = f.0.join(".devmeld/output/index.md");
    fs::write(&output, "external edit").unwrap();
    assert!(plan.apply().is_err());
    assert_eq!(fs::read(&output).unwrap(), b"external edit");
    assert!(devmeld::prepare(&location, Mutation::Publish).is_err());
}

#[test]
fn explicit_native_input_base_and_structured_entry_preview_are_independent_of_cli() {
    let f = Fixture::new();
    fs::create_dir(f.0.join("inputs")).unwrap();
    fs::write(f.0.join("inputs/notes.md"), "authored").unwrap();
    let host = f.0.join("inputs/AGENTS.md");
    fs::write(&host, "# Human rules\n").unwrap();
    let location = ContextLocation {
        base_directory: f.0.join("inputs"),
        directory: Some("../context".into()),
    };
    let plan = devmeld::prepare(
        &location,
        Mutation::RegisterResource(RegisterResource {
            source: ResourceSource::Document("notes.md".into()),
            address: Some(path("notes")),
            edit: NodeEdit::default(),
        }),
    )
    .unwrap();
    assert!(!f.0.join("context").exists());
    plan.apply().unwrap();
    save(
        &location,
        Mutation::RegisterEntry {
            kind: devmeld::EntryKind::Instructions,
            file: "AGENTS.md".into(),
        },
    );
    assert_eq!(fs::read(&host).unwrap(), b"# Human rules\n");
    let plan = devmeld::prepare(&location, Mutation::Publish).unwrap();
    let devmeld::PlanPreview::Changes { targets, .. } = plan.preview() else {
        panic!("expected changes")
    };
    let target = targets
        .iter()
        .find(|target| target.entry.is_some())
        .unwrap();
    assert_eq!(target.path, fs::canonicalize(&host).unwrap());
    assert_eq!(target.entry, Some(devmeld::EntryChange::Update));
    assert_eq!(target.before.unwrap(), b"# Human rules\n");
    assert!(target.after.unwrap().ends_with(b"# Human rules\n"));
    plan.apply().unwrap();
    save(&location, Mutation::RemoveEntry("AGENTS.md".into()));
    let plan = devmeld::prepare(&location, Mutation::Publish).unwrap();
    let devmeld::PlanPreview::Changes { targets, .. } = plan.preview() else {
        panic!("expected detachment")
    };
    assert!(
        targets
            .iter()
            .any(|target| target.entry == Some(devmeld::EntryChange::Detach))
    );
    plan.apply().unwrap();
    assert_eq!(fs::read(&host).unwrap(), b"# Human rules\n");
    assert_eq!(fs::read(f.0.join("inputs/notes.md")).unwrap(), b"authored");
}
