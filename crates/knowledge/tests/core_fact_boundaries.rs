use devmeld_catalog::{
    PortableLocator, RepositoryRegistration, SourceReference, Workspace, WorkspaceId,
};
use devmeld_knowledge::{
    CheckoutBasis, CheckoutFactInput, CheckoutFacts, ResourceFact, ReviewStatus, Scope, ScopeInput,
    SourceFact, SourceType, ValidityStatus,
};
use devmeld_local_context::{
    Availability, CheckoutObservation, Freshness, LocalPath, ObservationInput, PathDialect,
    Resolution, TaskContext, WorkingTree,
};
use devmeld_shared_kernel::{RepositoryId, ResourceId};
use std::time::UNIX_EPOCH;

#[test]
fn independently_supplied_facts_produce_identical_knowledge_results() {
    let repository = RepositoryId::new("repo").unwrap();
    let ws_id = WorkspaceId::new("ws").unwrap();
    let source = SourceReference::new("vault", PortableLocator::new("vault").unwrap()).unwrap();
    let catalog = Workspace::new(ws_id.clone(), "Project", 1, source, vec![])
        .unwrap()
        .with_repository(
            RepositoryRegistration::new(
                ws_id,
                repository.clone(),
                "repo",
                "Repository",
                vec![],
                None,
            )
            .unwrap(),
        )
        .unwrap();
    let original_catalog = catalog.clone();
    let observation = CheckoutObservation::new(ObservationInput {
        id: "o1".into(),
        repository: repository.clone(),
        local_path: LocalPath::new("/repo", PathDialect::Posix).unwrap(),
        branch: Some("main".into()),
        commit: Some("abc".into()),
        working_tree: WorkingTree::Clean,
        observed_at: UNIX_EPOCH,
        observer_revision: "fixture/1".into(),
        availability: Availability::Available,
        freshness: Freshness::Fresh,
    })
    .unwrap();
    let validated = TaskContext::default()
        .validate(
            catalog.repositories().keys().cloned().collect(),
            vec![observation.clone()],
            vec![],
            vec![],
        )
        .unwrap();
    let Resolution::Resolved(resolved) = validated.resolve(&repository).unwrap() else {
        panic!("expected Resolved")
    };
    let selected = resolved.selected();
    // Consumer mapping is explicit. It is not a dependency of production Knowledge.
    let mapped = CheckoutFacts::new(CheckoutFactInput {
        repository: resolved.repository().clone(),
        observation: selected.id().into(),
        branch: selected.branch().map(str::to_owned),
        revision: selected.commit().map(str::to_owned),
        working_tree: match selected.working_tree() {
            WorkingTree::Clean => "clean".into(),
            WorkingTree::Dirty(reason) => format!("dirty: {reason}"),
            WorkingTree::Unknown(reason) => format!("unknown: {reason}"),
        },
        observed_at: selected.observed_at(),
        observer_revision: selected.observer_revision().into(),
        basis: match resolved.basis() {
            devmeld_local_context::ResolutionBasis::ExplicitTask => CheckoutBasis::ExplicitTask,
            devmeld_local_context::ResolutionBasis::WorkspaceSelection => {
                CheckoutBasis::WorkspaceSelection
            }
            devmeld_local_context::ResolutionBasis::LocalDefault => CheckoutBasis::LocalDefault,
            devmeld_local_context::ResolutionBasis::SoleCandidate => CheckoutBasis::SoleCandidate,
        },
    })
    .unwrap();
    // Independent builder: it does not call the mapping or read its result.
    let independent = CheckoutFacts::new(CheckoutFactInput {
        repository: RepositoryId::new("repo").unwrap(),
        observation: "o1".into(),
        branch: Some("main".into()),
        revision: Some("abc".into()),
        working_tree: "clean".into(),
        observed_at: UNIX_EPOCH,
        observer_revision: "fixture/1".into(),
        basis: CheckoutBasis::SoleCandidate,
    })
    .unwrap();
    let scope = Scope::new(ScopeInput {
        repository: Some(repository),
        checkout: Some("o1".into()),
        revision: Some("abc".into()),
        ..Default::default()
    })
    .unwrap();
    let resource = ResourceFact::new(
        ResourceId::new("note").unwrap(),
        SourceFact::new("vault", "v1", SourceType::Declared).unwrap(),
        vec![],
        scope.clone(),
        ReviewStatus::Accepted,
        ValidityStatus::Current,
    )
    .unwrap();
    assert_eq!(
        resource
            .evaluate(scope.clone(), Some(mapped), None)
            .unwrap(),
        resource.evaluate(scope, Some(independent), None).unwrap()
    );
    assert_eq!(catalog, original_catalog);
    assert_eq!(selected, &observation);
}
