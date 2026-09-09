use devmeld_resources::ResourceId;
use devmeld_resources::organization::{Organization, OrganizationPath};

fn path(value: &str) -> OrganizationPath {
    OrganizationPath::new(value).unwrap()
}

#[test]
fn creation_defaults_are_captured_for_new_nodes_and_never_retroactive() {
    use devmeld_resources::organization::InheritanceDefaults;
    let mut organization = Organization::default();
    organization.add_group(path("old")).unwrap();
    organization.set_defaults(InheritanceDefaults::new(true, false));
    organization
        .register(
            ResourceId::new("resource-1").unwrap(),
            path("new/nested/source"),
        )
        .unwrap();
    assert_eq!(organization.inherits(&path("old")), Some(false));
    assert_eq!(organization.propagates(&path("old")), Some(true));
    for node in ["new", "new/nested", "new/nested/source"] {
        assert_eq!(organization.inherits(&path(node)), Some(true));
    }
    assert_eq!(organization.propagates(&path("new/nested")), Some(false));
    organization.set_defaults(InheritanceDefaults::new(false, true));
    organization
        .set_inherit(&path("new/nested/source"), false)
        .unwrap();
    organization
        .move_group(&path("new/nested"), path("other/destination"))
        .unwrap();
    assert_eq!(organization.inherits(&path("other")), Some(false));
    assert_eq!(organization.propagates(&path("other")), Some(true));
    assert_eq!(
        organization.inherits(&path("other/destination")),
        Some(true)
    );
    assert_eq!(
        organization.propagates(&path("other/destination")),
        Some(false)
    );
    organization
        .move_resource(&path("other/destination/source"), path("moved/source"))
        .unwrap();
    assert_eq!(organization.inherits(&path("moved/source")), Some(false));
    assert_eq!(
        organization.defaults(),
        InheritanceDefaults::new(false, true)
    );
}

#[test]
fn inheritance_preserves_origins_overrides_fields_and_cuts_the_whole_ancestor_chain() {
    use devmeld_resources::organization::LocalAnnotations;
    let mut organization = Organization::default();
    organization
        .register(
            ResourceId::new("resource-1").unwrap(),
            path("team/db/service"),
        )
        .unwrap();
    organization
        .set_annotations(
            &path("team"),
            LocalAnnotations::new(
                Some("Team only".into()),
                vec!["shared".into(), "root".into()],
                vec![
                    ("owner".into(), "team".into()),
                    ("environment".into(), "prod".into()),
                ],
            )
            .unwrap(),
        )
        .unwrap();
    organization
        .set_annotations(
            &path("team/db"),
            LocalAnnotations::new(
                None,
                vec!["database".into(), "shared".into()],
                vec![("environment".into(), "test".into())],
            )
            .unwrap(),
        )
        .unwrap();
    organization
        .set_annotations(
            &path("team/db/service"),
            LocalAnnotations::new(
                Some("Service only".into()),
                vec!["shared".into()],
                vec![("environment".into(), "".into())],
            )
            .unwrap(),
        )
        .unwrap();
    organization.set_inherit(&path("team/db"), true).unwrap();
    organization
        .set_inherit(&path("team/db/service"), true)
        .unwrap();
    let original = organization.clone();
    let effective = organization
        .effective_annotations(&path("team/db/service"))
        .unwrap();
    assert_eq!(effective.fields()["environment"].value(), "");
    assert_eq!(
        effective.fields()["environment"].origin(),
        &path("team/db/service")
    );
    assert_eq!(effective.fields()["owner"].origin(), &path("team"));
    assert_eq!(
        effective.tags()["shared"]
            .iter()
            .cloned()
            .collect::<Vec<_>>(),
        vec![path("team"), path("team/db"), path("team/db/service")]
    );
    assert_eq!(organization, original);
    organization.set_inherit(&path("team/db"), false).unwrap();
    let cut = organization
        .effective_annotations(&path("team/db/service"))
        .unwrap();
    assert!(!cut.fields().contains_key("owner"));
    assert!(!cut.tags().contains_key("root"));
    assert!(cut.tags().contains_key("database"));
    organization.set_propagate(&path("team/db"), false).unwrap();
    assert!(
        !organization
            .effective_annotations(&path("team/db/service"))
            .unwrap()
            .tags()
            .contains_key("database")
    );
    organization.set_propagate(&path("team/db"), true).unwrap();
    organization
        .move_group(&path("team/db"), path("archive/db"))
        .unwrap();
    assert_eq!(organization.inherits(&path("archive/db")), Some(false));
    assert_eq!(
        organization.inherits(&path("archive/db/service")),
        Some(true)
    );
    let moved = organization
        .effective_annotations(&path("archive/db/service"))
        .unwrap();
    assert_eq!(
        moved.tags()["database"].iter().next(),
        Some(&path("archive/db"))
    );
    organization
        .set_annotations(
            &path("archive/db"),
            LocalAnnotations::new(None, vec!["new-tag".into()], vec![]).unwrap(),
        )
        .unwrap();
    assert!(
        organization
            .effective_annotations(&path("archive/db/service"))
            .unwrap()
            .tags()
            .contains_key("new-tag")
    );
    assert!(
        !organization
            .annotations(&path("archive/db/service"))
            .unwrap()
            .tags()
            .any(|tag| tag == "new-tag")
    );
}

#[test]
fn effective_annotations_require_both_parent_propagation_and_child_inheritance() {
    use devmeld_resources::organization::LocalAnnotations;
    let mut organization = Organization::default();
    let id = ResourceId::new("resource-1").unwrap();
    organization
        .register(id.clone(), path("db/service"))
        .unwrap();
    organization
        .set_annotations(
            &path("db"),
            LocalAnnotations::new(
                Some("parent description".into()),
                vec!["shared".into()],
                vec![("environment".into(), "test".into())],
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(organization.inherits(&path("db/service")), Some(false));
    assert_eq!(organization.propagates(&path("db")), Some(true));
    for (propagate, inherit, expected) in [
        (false, false, false),
        (false, true, false),
        (true, false, false),
        (true, true, true),
    ] {
        organization.set_propagate(&path("db"), propagate).unwrap();
        organization
            .set_inherit(&path("db/service"), inherit)
            .unwrap();
        let effective = organization
            .effective_annotations(&path("db/service"))
            .unwrap();
        assert_eq!(effective.tags().contains_key("shared"), expected);
        assert_eq!(effective.fields().contains_key("environment"), expected);
        if expected {
            let field = &effective.fields()["environment"];
            assert_eq!(field.value(), "test");
            assert_eq!(field.origin(), &path("db"));
        }
        assert!(
            organization
                .annotations(&path("db/service"))
                .unwrap()
                .is_empty()
        );
        assert_eq!(organization.resource_at(&path("db/service")), Some(&id));
    }
    let before = organization.clone();
    assert!(
        organization
            .set_propagate(&path("db/service"), true)
            .is_err()
    );
    assert!(organization.set_inherit(&path("missing"), true).is_err());
    assert!(
        organization
            .effective_annotations(&path("missing"))
            .is_err()
    );
    assert_eq!(organization, before);
}

#[test]
fn annotation_edits_preserve_omissions_and_reject_contradictions_in_the_domain() {
    use devmeld_resources::organization::{AnnotationEdit, LocalAnnotations};
    let original = LocalAnnotations::new(
        Some("group".into()),
        vec!["one".into(), "two".into()],
        vec![
            ("environment".into(), "test".into()),
            ("owner".into(), "team".into()),
        ],
    )
    .unwrap();
    let values = LocalAnnotations::new(
        None,
        vec!["three".into()],
        vec![("environment".into(), "production".into())],
    )
    .unwrap();
    let edit = AnnotationEdit::new(
        values.clone(),
        false,
        vec!["one".into()],
        vec!["owner".into()],
    )
    .unwrap();
    let changed = edit.apply(&original).unwrap();
    assert_eq!(changed.description(), Some("group"));
    assert_eq!(
        changed.tags().map(String::as_str).collect::<Vec<_>>(),
        vec!["three", "two"]
    );
    assert_eq!(
        changed.fields().get("environment").map(String::as_str),
        Some("production")
    );
    assert!(!changed.fields().contains_key("owner"));
    assert!(AnnotationEdit::new(values.clone(), false, vec!["three".into()], vec![]).is_err());
    assert!(AnnotationEdit::new(values, false, vec![], vec!["environment".into()]).is_err());
    assert!(AnnotationEdit::new(original.clone(), true, vec![], vec![]).is_err());
    let cleared = AnnotationEdit::new(LocalAnnotations::default(), true, vec![], vec![]).unwrap();
    assert_eq!(cleared.apply(&original).unwrap().description(), None);
    assert!(cleared.for_creation().is_err());
    assert_eq!(original.description(), Some("group"));
}

#[test]
fn local_annotations_keep_description_tags_and_extensible_fields_distinct() {
    use devmeld_resources::organization::LocalAnnotations;
    let annotations = LocalAnnotations::new(
        Some("团队 HTTP / ssh\nConnection notes".into()),
        vec!["backend".into(), "shared".into(), "backend".into()],
        vec![
            ("environment".into(), "test".into()),
            ("owner/team".into(), "平台".into()),
        ],
    )
    .unwrap();
    assert_eq!(
        annotations.description(),
        Some("团队 HTTP / ssh\nConnection notes")
    );
    assert_eq!(
        annotations.tags().map(String::as_str).collect::<Vec<_>>(),
        vec!["backend", "shared"]
    );
    assert_eq!(
        annotations.fields().get("owner/team").map(String::as_str),
        Some("平台")
    );
    assert!(!annotations.fields().contains_key("shared"));
    assert!(
        LocalAnnotations::new(
            None,
            vec![],
            vec![("x".into(), "a".into()), ("x".into(), "b".into())]
        )
        .is_err()
    );
    assert!(LocalAnnotations::new(None, vec!["".into()], vec![]).is_err());
    assert!(LocalAnnotations::new(None, vec![], vec![("bad=key".into(), "x".into())]).is_err());
    assert!(LocalAnnotations::new(Some("\u{1b}[2J".into()), vec![], vec![]).is_err());
    assert!(LocalAnnotations::default().is_empty());
}

#[test]
fn organization_addresses_are_separate_from_identity_and_allow_equal_leaf_names() {
    let mut organization = Organization::default();
    let first = ResourceId::new("resource-1").unwrap();
    let second = ResourceId::new("resource-2").unwrap();
    let created = organization
        .register(first.clone(), path("database/test/orders"))
        .unwrap();
    assert_eq!(created, vec![path("database"), path("database/test")]);
    organization
        .register(second.clone(), path("database/production/orders"))
        .unwrap();
    assert_eq!(
        organization.resource_at(&path("database/test/orders")),
        Some(&first)
    );
    assert_eq!(
        organization.resource_at(&path("database/production/orders")),
        Some(&second)
    );
    assert_eq!(
        organization.path_for(&first),
        Some(&path("database/test/orders"))
    );
}

#[test]
fn annotations_belong_to_nodes_and_follow_moves_without_implicit_inheritance() {
    use devmeld_resources::organization::LocalAnnotations;
    let mut organization = Organization::default();
    let id = ResourceId::new("resource-1").unwrap();
    organization
        .register(id.clone(), path("db/test/notes"))
        .unwrap();
    let group_info = LocalAnnotations::new(
        Some("database context".into()),
        vec!["backend".into()],
        vec![],
    )
    .unwrap();
    let resource_info =
        LocalAnnotations::new(None, vec![], vec![("environment".into(), "test".into())]).unwrap();
    organization
        .set_annotations(&path("db"), group_info.clone())
        .unwrap();
    organization
        .set_annotations(&path("db/test/notes"), resource_info.clone())
        .unwrap();
    assert!(
        organization
            .annotations(&path("db/test"))
            .unwrap()
            .is_empty()
    );
    let before = organization.clone();
    assert!(
        organization
            .set_annotations(&path("missing"), group_info.clone())
            .is_err()
    );
    assert_eq!(organization, before);
    organization
        .move_group(&path("db"), path("archive/db"))
        .unwrap();
    assert_eq!(
        organization.annotations(&path("archive/db")),
        Some(&group_info)
    );
    assert_eq!(
        organization.annotations(&path("archive/db/test/notes")),
        Some(&resource_info)
    );
    organization
        .move_resource(&path("archive/db/test/notes"), path("docs/notes"))
        .unwrap();
    assert_eq!(
        organization.annotations(&path("docs/notes")),
        Some(&resource_info)
    );
    assert_eq!(organization.resource_at(&path("docs/notes")), Some(&id));
    assert!(organization.annotations(&path("docs")).unwrap().is_empty());
    organization.remove_group(&path("archive/db/test")).unwrap();
    organization.remove_group(&path("archive/db")).unwrap();
    assert!(organization.annotations(&path("archive/db")).is_none());
}

#[test]
fn restoring_organization_rejects_missing_parents_and_collisions() {
    let id = ResourceId::new("resource-1").unwrap();
    assert!(Organization::from_parts(vec![], vec![(path("db/test"), id.clone())]).is_err());
    assert!(Organization::from_parts(vec![path("db/test")], vec![]).is_err());
    let original =
        Organization::from_parts(vec![path("db")], vec![(path("db/test"), id.clone())]).unwrap();
    let mut organization = original.clone();
    assert!(organization.register(id, path("new/other")).is_err());
    assert_eq!(organization, original);
    assert!(
        organization
            .register(
                ResourceId::new("resource-2").unwrap(),
                path("db/test/child")
            )
            .is_err()
    );
    assert_eq!(organization, original);
}

#[test]
fn organization_names_are_not_native_paths_or_dot_navigation() {
    for value in [
        "",
        "/db",
        "db/",
        "db//test",
        "db/../test",
        "./db",
        "C:/db",
        "db\\test",
        "db/ test",
        "db\ntest",
    ] {
        assert!(OrganizationPath::new(value).is_err(), "{value:?}");
    }
    assert!(OrganizationPath::new("知识/团队 notes").is_ok());
}

#[test]
fn moving_a_resource_preserves_identity_and_rejects_collisions_atomically() {
    let mut organization = Organization::default();
    let id = ResourceId::new("resource-1").unwrap();
    organization
        .register(id.clone(), path("database/test/orders"))
        .unwrap();
    organization
        .register(ResourceId::new("resource-2").unwrap(), path("tools/query"))
        .unwrap();
    organization
        .move_resource(&path("database/test/orders"), path("archive/orders"))
        .unwrap();
    assert_eq!(organization.resource_at(&path("archive/orders")), Some(&id));
    assert!(
        organization
            .resource_at(&path("database/test/orders"))
            .is_none()
    );
    assert!(organization.groups().any(|p| p == &path("database/test")));
    let before = organization.clone();
    for destination in [
        "tools",
        "tools/query",
        "tools/query/child",
        "archive/orders/child",
    ] {
        assert!(
            organization
                .move_resource(&path("archive/orders"), path(destination))
                .is_err()
        );
        assert_eq!(organization, before, "{destination}");
    }
    assert!(
        organization
            .move_resource(&path("missing"), path("new/place"))
            .is_err()
    );
    assert_eq!(organization, before);
    organization
        .move_resource(&path("archive/orders"), path("archive/orders"))
        .unwrap();
    assert_eq!(organization, before);
}

#[test]
fn groups_have_an_explicit_lifecycle_without_recursive_removal() {
    let mut organization = Organization::default();
    assert_eq!(
        organization.add_group(path("database/test")).unwrap(),
        vec![path("database"), path("database/test")]
    );
    let before = organization.clone();
    assert!(organization.remove_group(&path("database")).is_err());
    assert!(organization.add_group(path("database/test")).is_err());
    assert!(organization.remove_group(&path("missing")).is_err());
    assert_eq!(organization, before);
    organization.remove_group(&path("database/test")).unwrap();
    organization
        .register(
            ResourceId::new("resource-1").unwrap(),
            path("database/orders"),
        )
        .unwrap();
    let before = organization.clone();
    assert!(organization.remove_group(&path("database")).is_err());
    assert!(
        organization
            .add_group(path("database/orders/child"))
            .is_err()
    );
    assert!(organization.add_group(path("database/orders")).is_err());
    assert_eq!(organization, before);
    organization.add_group(path("empty")).unwrap();
    organization.remove_group(&path("empty")).unwrap();
    assert_eq!(organization, before);
}

#[test]
fn moving_a_group_readdresses_exactly_its_subtree_and_retains_all_identities() {
    let mut organization = Organization::default();
    let first = ResourceId::new("resource-1").unwrap();
    let second = ResourceId::new("resource-2").unwrap();
    organization
        .register(first.clone(), path("database/test/orders"))
        .unwrap();
    organization
        .register(second.clone(), path("database2/test/orders"))
        .unwrap();
    organization.add_group(path("database/empty")).unwrap();
    organization
        .move_group(&path("database"), path("archive/database"))
        .unwrap();
    assert_eq!(
        organization.path_for(&first),
        Some(&path("archive/database/test/orders"))
    );
    assert_eq!(
        organization.path_for(&second),
        Some(&path("database2/test/orders"))
    );
    assert!(
        organization
            .groups()
            .any(|p| p == &path("archive/database/empty"))
    );
    assert!(!organization.groups().any(|p| p == &path("database")));
    let before = organization.clone();
    for destination in [
        "archive",
        "database2",
        "database2/test/orders",
        "database2/test/orders/child",
        "archive/database/new/child",
    ] {
        assert!(
            organization
                .move_group(&path("archive/database"), path(destination))
                .is_err()
        );
        assert_eq!(organization, before, "{destination}");
    }
    assert!(
        organization
            .move_group(&path("missing"), path("new/place"))
            .is_err()
    );
    assert_eq!(organization, before);
    organization
        .move_group(&path("archive/database"), path("archive/database"))
        .unwrap();
    assert_eq!(organization, before);
}
