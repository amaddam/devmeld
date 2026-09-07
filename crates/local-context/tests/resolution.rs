use devmeld_local_context::*;
use devmeld_shared_kernel::RepositoryId;
use std::time::UNIX_EPOCH;

fn repo() -> RepositoryId {
    RepositoryId::new("repo").unwrap()
}
fn obs(id: &str, freshness: Freshness) -> CheckoutObservation {
    CheckoutObservation::new(ObservationInput {
        id: id.into(),
        repository: repo(),
        local_path: LocalPath::new(format!("/{id}"), PathDialect::Posix).unwrap(),
        branch: None,
        commit: None,
        working_tree: WorkingTree::Unknown("not inspected".into()),
        observed_at: UNIX_EPOCH,
        observer_revision: "fixture/1".into(),
        availability: Availability::Available,
        freshness,
    })
    .unwrap()
}
fn selection(id: &str) -> CheckoutSelection {
    CheckoutSelection::new(repo(), id).unwrap()
}

#[test]
fn all_four_resolution_bases_obey_precedence() {
    let observations = vec![obs("a", Freshness::Fresh), obs("b", Freshness::Fresh)];
    for (explicit, workspace, defaults, expected) in [
        (
            vec![selection("a")],
            vec![selection("b")],
            vec![selection("b")],
            ResolutionBasis::ExplicitTask,
        ),
        (
            vec![],
            vec![selection("a")],
            vec![selection("b")],
            ResolutionBasis::WorkspaceSelection,
        ),
        (
            vec![],
            vec![],
            vec![selection("a")],
            ResolutionBasis::LocalDefault,
        ),
    ] {
        let context = TaskContext {
            selections: explicit,
            working_area: None,
        }
        .validate(vec![repo()], observations.clone(), workspace, defaults)
        .unwrap();
        let Resolution::Resolved(result) = context.resolve(&repo()).unwrap() else {
            panic!("expected Resolved")
        };
        assert_eq!(result.selected().id(), "a");
        assert_eq!(result.basis(), expected);
        assert_eq!(result.considered().len(), 2);
    }
    let valid = TaskContext::default()
        .validate(
            vec![repo()],
            vec![obs("a", Freshness::Fresh)],
            vec![],
            vec![],
        )
        .unwrap();
    let Resolution::Resolved(result) = valid.resolve(&repo()).unwrap() else {
        panic!("expected Resolved")
    };
    assert_eq!(result.basis(), ResolutionBasis::SoleCandidate);
}

#[test]
fn ambiguity_is_order_independent_and_unavailable_is_explainable() {
    let candidates = vec![obs("b", Freshness::Fresh), obs("a", Freshness::Fresh)];
    let forward = TaskContext::default()
        .validate(vec![repo()], candidates.clone(), vec![], vec![])
        .unwrap()
        .resolve(&repo())
        .unwrap();
    let reverse = TaskContext::default()
        .validate(
            vec![repo()],
            candidates.into_iter().rev().collect(),
            vec![],
            vec![],
        )
        .unwrap()
        .resolve(&repo())
        .unwrap();
    assert_eq!(forward, reverse);
    assert!(matches!(forward, Resolution::Ambiguous(_)));
    let unavailable = TaskContext::default()
        .validate(
            vec![repo()],
            vec![obs("a", Freshness::Stale("old".into()))],
            vec![],
            vec![],
        )
        .unwrap()
        .resolve(&repo())
        .unwrap();
    let Resolution::Unavailable(result) = unavailable else {
        panic!("expected Unavailable")
    };
    assert_eq!(result.considered().len(), 1);
    assert!(!result.reason().is_empty());
}

#[test]
fn stale_explicit_selection_cannot_fall_back_but_stale_weak_preference_can() {
    let observations = vec![
        obs("stale", Freshness::Stale("old".into())),
        obs("fresh", Freshness::Fresh),
    ];
    assert!(
        TaskContext {
            selections: vec![selection("stale")],
            working_area: None
        }
        .validate(vec![repo()], observations.clone(), vec![], vec![])
        .is_err()
    );
    let valid = TaskContext::default()
        .validate(vec![repo()], observations, vec![selection("stale")], vec![])
        .unwrap();
    let Resolution::Resolved(result) = valid.resolve(&repo()).unwrap() else {
        panic!("expected Resolved")
    };
    assert_eq!(result.selected().id(), "fresh");
    assert_eq!(result.basis(), ResolutionBasis::SoleCandidate);
    assert!(!result.ignored_preferences().is_empty());
}
