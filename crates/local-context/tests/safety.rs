use devmeld_local_context::*;
use devmeld_shared_kernel::RepositoryId;
use std::time::UNIX_EPOCH;

fn repo() -> RepositoryId {
    RepositoryId::new("repo").unwrap()
}
fn input() -> ObservationInput {
    ObservationInput {
        id: "o1".into(),
        repository: repo(),
        local_path: LocalPath::new("/repo", PathDialect::Posix).unwrap(),
        branch: Some("main".into()),
        commit: Some("abc".into()),
        working_tree: WorkingTree::Clean,
        observed_at: UNIX_EPOCH,
        observer_revision: "fixture/1".into(),
        availability: Availability::Available,
        freshness: Freshness::Fresh,
    }
}

#[test]
fn changed_caller_inputs_cannot_replace_validated_snapshot() {
    let mut original = input();
    let old = CheckoutObservation::new(original.clone()).unwrap();
    let mut raw = TaskContext {
        selections: vec![CheckoutSelection::new(repo(), "o1").unwrap()],
        working_area: None,
    };
    let valid = raw
        .validate(vec![repo()], vec![old.clone()], vec![], vec![])
        .unwrap();
    original.freshness = Freshness::Stale("later inspection".into());
    original.commit = Some("changed".into());
    let changed = CheckoutObservation::new(original).unwrap();
    assert!(
        raw.validate(vec![repo()], vec![changed], vec![], vec![])
            .is_err()
    );
    raw.selections.clear();
    let Resolution::Resolved(result) = valid.resolve(&repo()).unwrap() else {
        panic!("expected Resolved")
    };
    assert_eq!(result.selected(), &old);
    assert_eq!(result.basis(), ResolutionBasis::ExplicitTask);
}

#[test]
fn conflicting_weak_preferences_never_become_order_based_winners() {
    let a = CheckoutObservation::new(input()).unwrap();
    let mut other = input();
    other.id = "o2".into();
    let b = CheckoutObservation::new(other).unwrap();
    for names in [["o1", "o2"], ["o2", "o1"]] {
        let preferences = names
            .into_iter()
            .map(|s| CheckoutSelection::new(repo(), s).unwrap())
            .collect();
        let valid = TaskContext::default()
            .validate(
                vec![repo()],
                vec![a.clone(), b.clone()],
                preferences,
                vec![],
            )
            .unwrap();
        assert!(matches!(
            valid.resolve(&repo()).unwrap(),
            Resolution::Ambiguous(_)
        ));
    }
}

#[test]
fn unknown_availability_is_not_a_success_default() {
    let mut facts = input();
    facts.availability = Availability::Unknown("not inspected".into());
    let observed = CheckoutObservation::new(facts).unwrap();
    let valid = TaskContext::default()
        .validate(vec![repo()], vec![observed], vec![], vec![])
        .unwrap();
    assert!(matches!(
        valid.resolve(&repo()).unwrap(),
        Resolution::Unavailable(_)
    ));
}

#[test]
fn all_three_candidate_permutations_preserve_ambiguity_and_explicit_choice() {
    let observations: Vec<_> = ["a", "b", "c"]
        .into_iter()
        .map(|id| {
            let mut facts = input();
            facts.id = id.into();
            CheckoutObservation::new(facts).unwrap()
        })
        .collect();
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let supplied = order.map(|index| observations[index].clone()).to_vec();
        let valid = TaskContext::default()
            .validate(vec![repo()], supplied.clone(), vec![], vec![])
            .unwrap();
        assert!(matches!(
            valid.resolve(&repo()).unwrap(),
            Resolution::Ambiguous(_)
        ));
        let selection = CheckoutSelection::new(repo(), "b").unwrap();
        let raw = TaskContext {
            selections: vec![selection.clone(), selection],
            working_area: None,
        };
        let explicit = raw
            .validate(vec![repo()], supplied, vec![], vec![])
            .unwrap();
        let Resolution::Resolved(result) = explicit.resolve(&repo()).unwrap() else {
            panic!("expected explicit choice")
        };
        assert_eq!(result.selected().id(), "b");
        assert_eq!(result.basis(), ResolutionBasis::ExplicitTask);
    }
}

#[test]
fn missing_observation_is_unavailable_but_unknown_repository_is_invalid() {
    let valid = TaskContext::default()
        .validate(vec![repo()], vec![], vec![], vec![])
        .unwrap();
    assert!(matches!(
        valid.resolve(&repo()).unwrap(),
        Resolution::Unavailable(_)
    ));
    assert!(
        valid
            .resolve(&RepositoryId::new("unknown").unwrap())
            .is_err()
    );
    for freshness in [
        Freshness::Fresh,
        Freshness::Stale("old".into()),
        Freshness::Unknown("not checked".into()),
    ] {
        let mut facts = input();
        facts.availability = Availability::Unavailable("missing directory".into());
        facts.freshness = freshness;
        let raw = TaskContext {
            selections: vec![CheckoutSelection::new(repo(), "o1").unwrap()],
            working_area: None,
        };
        assert!(
            raw.validate(
                vec![repo()],
                vec![CheckoutObservation::new(facts).unwrap()],
                vec![],
                vec![]
            )
            .is_err()
        );
    }
}
