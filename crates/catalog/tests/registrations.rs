use devmeld_catalog::{
    PortableLocator, RepositoryRegistration, ResourceRegistration, SourceReference, WorkspaceId,
};
use devmeld_shared_kernel::{RepositoryId, ResourceId};

#[test]
fn portable_locators_accept_supported_relative_and_remote_forms() {
    for value in [
        "vault",
        "docs/设计.md",
        "https://example.org/team/repo.git",
        "ssh://example.org/repo",
    ] {
        assert_eq!(PortableLocator::new(value).unwrap().as_str(), value);
    }
}

#[test]
fn local_credentials_traversal_and_ambiguous_encoding_are_rejected() {
    for value in [
        "",
        " vault",
        "/etc/passwd",
        "C:/repo",
        "C:repo",
        "\\\\host\\repo",
        "~/vault",
        "%7e/vault",
        "%7Euser/vault",
        "file:///repo",
        "../secret",
        "a/../b",
        "a\\..\\b",
        "https://user:pass@host/repo",
        "https://host/repo?token=secret",
        "a/%2e%2e/b",
        "a/%252e%252e/b",
        "a/%00",
        "a/%ZZ",
        "https://host/%2f../x",
        "https://",
        "//host/repo",
        "https://host/repo#secret",
    ] {
        assert!(PortableLocator::new(value).is_err(), "accepted {value:?}");
    }
}

#[test]
fn labels_and_aliases_can_change_without_changing_repository_identity() {
    let ws = WorkspaceId::new("ws").unwrap();
    let id = RepositoryId::new("repo").unwrap();
    let old =
        RepositoryRegistration::new(ws, id.clone(), "main", "Main", vec!["legacy".into()], None)
            .unwrap();
    let new = old.with_labels("Renamed", vec!["new".into()]).unwrap();
    assert_eq!(new.id(), &id);
    assert_eq!(old.display_name(), "Main");
    assert_eq!(new.aliases(), &["new"]);
    assert!(old.with_labels(" ", vec![]).is_err());
    assert!(old.with_labels("Valid", vec!["main".into()]).is_err());
}

#[test]
fn resource_keeps_declared_source_type_locator_and_eligibility() {
    let source = SourceReference::new("vault", PortableLocator::new("vault").unwrap()).unwrap();
    let resource = ResourceRegistration::new(
        WorkspaceId::new("ws").unwrap(),
        ResourceId::new("note").unwrap(),
        source.clone(),
        "markdown",
        PortableLocator::new("notes/design.md").unwrap(),
        false,
    )
    .unwrap();
    assert_eq!(resource.source(), &source);
    assert_eq!(resource.resource_type(), "markdown");
    assert!(!resource.eligible());
}
