use devmeld_catalog::{
    ContextProfile, PortableLocator, ProfileId, RepositoryRegistration, ResourceRegistration,
    SUPPORTED_SCHEMA_VERSION, SourceReference, Workspace, WorkspaceId,
};
use devmeld_shared_kernel::{RepositoryId, ResourceId};

fn source() -> SourceReference {
    SourceReference::new("vault", PortableLocator::new("vault").unwrap()).unwrap()
}
fn workspace() -> Workspace {
    Workspace::new(
        WorkspaceId::new("ws").unwrap(),
        "Project",
        SUPPORTED_SCHEMA_VERSION,
        source(),
        vec![],
    )
    .unwrap()
}
fn repo(id: &str, key: &str, aliases: &[&str]) -> RepositoryRegistration {
    RepositoryRegistration::new(
        WorkspaceId::new("ws").unwrap(),
        RepositoryId::new(id).unwrap(),
        key,
        "Repo",
        aliases.iter().map(|v| v.to_string()).collect(),
        None,
    )
    .unwrap()
}

#[test]
fn workspace_requires_supported_schema_and_distinct_sources() {
    assert!(
        Workspace::new(
            WorkspaceId::new("ws").unwrap(),
            "Project",
            2,
            source(),
            vec![]
        )
        .is_err()
    );
    assert!(
        Workspace::new(
            WorkspaceId::new("ws").unwrap(),
            "Project",
            SUPPORTED_SCHEMA_VERSION,
            source(),
            vec![source()]
        )
        .is_err()
    );
    assert_eq!(workspace().schema_version(), 1);
    assert_eq!(workspace().primary_vault(), &source());
}

#[test]
fn collisions_and_wrong_workspace_are_atomic_failures() {
    let original = workspace()
        .with_repository(repo("r1", "main", &["alias"]))
        .unwrap();
    assert!(original.with_repository(repo("r2", "alias", &[])).is_err());
    assert!(
        original
            .with_repository(repo("r2", "other", &["main"]))
            .is_err()
    );
    let foreign = RepositoryRegistration::new(
        WorkspaceId::new("other").unwrap(),
        RepositoryId::new("x").unwrap(),
        "x",
        "X",
        vec![],
        None,
    )
    .unwrap();
    assert!(original.with_repository(foreign).is_err());
    assert_eq!(original.repositories().len(), 1);
}

#[test]
fn profile_checks_known_eligible_members_and_blocks_referenced_removal() {
    let repo_id = RepositoryId::new("repo").unwrap();
    let res_id = ResourceId::new("note").unwrap();
    let profile = ContextProfile::new(
        WorkspaceId::new("ws").unwrap(),
        ProfileId::new("profile").unwrap(),
        "Coding",
        vec![repo_id.clone()],
        vec![res_id.clone()],
    )
    .unwrap();
    assert!(workspace().with_profile(profile.clone()).is_err());
    let resource = ResourceRegistration::new(
        WorkspaceId::new("ws").unwrap(),
        res_id.clone(),
        source(),
        "markdown",
        PortableLocator::new("note.md").unwrap(),
        true,
    )
    .unwrap();
    let ready = workspace()
        .with_repository(repo("repo", "repo", &[]))
        .unwrap()
        .with_resource(resource)
        .unwrap()
        .with_profile(profile)
        .unwrap();
    assert!(ready.without_repository(&repo_id).is_err());
    assert!(ready.without_resource(&res_id).is_err());
    let revised = ready
        .without_profile(&ProfileId::new("profile").unwrap())
        .unwrap();
    assert!(revised.without_repository(&repo_id).is_ok());
    assert!(revised.without_resource(&res_id).is_ok());
    assert_eq!(ready.profiles().len(), 1);
}

#[test]
fn resource_source_and_eligibility_cannot_bypass_workspace_validation() {
    let id = ResourceId::new("r").unwrap();
    let resource = ResourceRegistration::new(
        WorkspaceId::new("ws").unwrap(),
        id.clone(),
        source(),
        "note",
        PortableLocator::new("r.md").unwrap(),
        false,
    )
    .unwrap();
    let state = workspace().with_resource(resource).unwrap();
    let profile = ContextProfile::new(
        WorkspaceId::new("ws").unwrap(),
        ProfileId::new("p").unwrap(),
        "P",
        vec![],
        vec![id],
    )
    .unwrap();
    assert!(state.with_profile(profile).is_err());
    let foreign_source =
        SourceReference::new("unknown", PortableLocator::new("elsewhere").unwrap()).unwrap();
    let resource = ResourceRegistration::new(
        WorkspaceId::new("ws").unwrap(),
        ResourceId::new("x").unwrap(),
        foreign_source,
        "note",
        PortableLocator::new("x.md").unwrap(),
        true,
    )
    .unwrap();
    assert!(state.with_resource(resource).is_err());
}

#[test]
fn replacing_referenced_resource_cannot_invalidate_profile_or_source_ownership() {
    let id = ResourceId::new("note").unwrap();
    let make = |source, eligible| {
        ResourceRegistration::new(
            WorkspaceId::new("ws").unwrap(),
            id.clone(),
            source,
            "markdown",
            PortableLocator::new("note.md").unwrap(),
            eligible,
        )
        .unwrap()
    };
    let profile = ContextProfile::new(
        WorkspaceId::new("ws").unwrap(),
        ProfileId::new("p").unwrap(),
        "P",
        vec![],
        vec![id.clone()],
    )
    .unwrap();
    let original = workspace()
        .with_resource(make(source(), true))
        .unwrap()
        .with_profile(profile)
        .unwrap();
    assert!(original.with_resource(make(source(), false)).is_err());
    let redirected =
        SourceReference::new("vault", PortableLocator::new("different-vault").unwrap()).unwrap();
    assert!(original.with_resource(make(redirected, true)).is_err());
    let foreign = ContextProfile::new(
        WorkspaceId::new("foreign").unwrap(),
        ProfileId::new("p").unwrap(),
        "P",
        vec![],
        vec![],
    )
    .unwrap();
    assert!(original.with_profile(foreign).is_err());
    assert!(original.resources().get(&id).unwrap().eligible());
    assert_eq!(original.profiles().len(), 1);
}

#[test]
fn profile_identity_is_validated_and_survives_renaming() {
    for invalid in ["", " ", " p", "p ", "p\nq", "p\0"] {
        assert!(ProfileId::new(invalid).is_err());
    }
    let spelling = "\u{9879}\u{76ee}/profile";
    let id = ProfileId::new(spelling).unwrap();
    assert_eq!(id.as_str(), spelling);
    let make = |name| {
        ContextProfile::new(
            WorkspaceId::new("ws").unwrap(),
            id.clone(),
            name,
            vec![],
            vec![],
        )
        .unwrap()
    };
    let original = workspace().with_profile(make("Before")).unwrap();
    let renamed = original.with_profile(make("After")).unwrap();
    assert_eq!(renamed.profiles().len(), 1);
    assert_eq!(renamed.profiles()[&id].id(), &id);
    assert_eq!(renamed.profiles()[&id].name(), "After");
    assert_eq!(original.profiles()[&id].name(), "Before");
    assert!(renamed.without_profile(&id).unwrap().profiles().is_empty());
}
