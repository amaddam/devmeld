use devmeld_local_context::*;
use devmeld_shared_kernel::RepositoryId;
use std::time::{Duration, UNIX_EPOCH};

fn repo() -> RepositoryId {
    RepositoryId::new("repo").unwrap()
}
fn observation(id: &str, repository: RepositoryId) -> CheckoutObservation {
    CheckoutObservation::new(ObservationInput {
        id: ObservationId::new(id).unwrap(),
        repository,
        local_path: LocalPath::new("C:/work/repo", PathDialect::Windows).unwrap(),
        branch: Some("main".into()),
        commit: Some("abc".into()),
        working_tree: WorkingTree::Dirty("edited file".into()),
        observed_at: UNIX_EPOCH + Duration::from_secs(42),
        observer_revision: "git-adapter/1".into(),
        availability: Availability::Available,
        freshness: Freshness::Fresh,
    })
    .unwrap()
}

#[test]
fn observation_preserves_full_provenance() {
    let observed = observation("o1", repo());
    assert_eq!(observed.branch(), Some("main"));
    assert_eq!(observed.commit(), Some("abc"));
    assert_eq!(observed.observed_at(), UNIX_EPOCH + Duration::from_secs(42));
    assert_eq!(observed.observer_revision(), "git-adapter/1");
    assert!(matches!(observed.working_tree(), WorkingTree::Dirty(_)));
    assert_eq!(observed.local_path().as_str(), "C:/work/repo");
}

#[test]
fn local_binding_transitions_are_local_checked_and_revisioned() {
    let registry = LocalBindingRegistry::new("machine", "developer", vec![repo()]).unwrap();
    let bound = registry
        .with_binding(
            LocalBinding::new(
                "b",
                repo(),
                LocalPath::new("/work/repo", PathDialect::Posix).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
    let selected = bound.with_default(repo(), "b").unwrap();
    let removed = selected.without_binding("b").unwrap();
    assert_eq!(registry.revision(), 0);
    assert_eq!(bound.revision(), 1);
    assert_eq!(selected.revision(), 2);
    assert_eq!(removed.revision(), 3);
    assert!(removed.default_bindings().is_empty());
    assert_eq!(selected.bindings().len(), 1);
    let unknown = LocalBinding::new(
        "x",
        RepositoryId::new("unknown").unwrap(),
        LocalPath::new("/x", PathDialect::Posix).unwrap(),
    )
    .unwrap();
    assert!(registry.with_binding(unknown).is_err());
}

#[test]
fn invalid_explicit_selections_keep_reason_and_never_resolve() {
    let observations = vec![
        observation("o1", repo()),
        observation("foreign", RepositoryId::new("other").unwrap()),
    ];
    for (selected, reason) in [
        ("missing", RejectionReason::UnknownObservation),
        ("foreign", RejectionReason::WrongRepository),
    ] {
        let raw = TaskContext {
            selections: vec![CheckoutSelection::new(
                repo(),
                ObservationId::new(selected).unwrap(),
            )],
            working_area: None,
        };
        let error = raw
            .validate(
                vec![repo(), RepositoryId::new("other").unwrap()],
                observations.clone(),
                vec![],
                vec![],
            )
            .unwrap_err();
        assert_eq!(
            error.rejections()[0]
                .selection()
                .unwrap()
                .observation_id()
                .as_str(),
            selected
        );
        assert_eq!(
            error.rejections()[0].source(),
            Some(SelectionSource::ExplicitTask)
        );
        assert!(!error.to_string().is_empty());
        assert_eq!(error.rejections()[0].reason(), &reason);
    }
    let conflict = TaskContext {
        selections: vec![
            CheckoutSelection::new(repo(), ObservationId::new("o1").unwrap()),
            CheckoutSelection::new(repo(), ObservationId::new("o2").unwrap()),
        ],
        working_area: None,
    };
    let error = conflict
        .validate(
            vec![repo()],
            vec![observation("o1", repo()), observation("o2", repo())],
            vec![],
            vec![],
        )
        .unwrap_err();
    assert_eq!(
        error.rejections()[0].reason(),
        &RejectionReason::ConflictingSelection
    );
}

#[test]
fn duplicate_snapshots_are_rejected_and_working_area_is_preserved() {
    let obs = observation("o1", repo());
    let error = TaskContext::default()
        .validate(vec![repo()], vec![obs.clone(), obs.clone()], vec![], vec![])
        .unwrap_err();
    assert_eq!(error.rejections().len(), 1);
    assert_eq!(error.rejections()[0].selection(), None);
    assert_eq!(error.rejections()[0].source(), None);
    assert_eq!(
        error.rejections()[0].reason(),
        &RejectionReason::DuplicateObservationIdentity(ObservationId::new("o1").unwrap()),
    );
    assert_eq!(error.to_string(), "duplicate observation identity: o1");
    let area = LocalPath::new("/work", PathDialect::Posix).unwrap();
    let raw = TaskContext {
        selections: vec![],
        working_area: Some(area.clone()),
    };
    let valid = raw
        .validate(vec![repo()], vec![obs], vec![], vec![])
        .unwrap();
    assert_eq!(valid.working_area(), Some(&area));
    assert!(
        valid
            .resolve(&RepositoryId::new("unknown").unwrap())
            .is_err()
    );
}

#[test]
fn observation_identity_is_validated_and_used_for_snapshot_lookup() {
    for invalid in ["", " ", " o1", "o1 ", "o\n1", "o\0"] {
        assert!(ObservationId::new(invalid).is_err());
    }
    let spelling = "\u{89c2}\u{5bdf}/o1";
    let id = ObservationId::new(spelling).unwrap();
    assert_eq!(id.as_str(), spelling);
    let observed = observation(spelling, repo());
    let valid = TaskContext {
        selections: vec![CheckoutSelection::new(repo(), id.clone())],
        working_area: None,
    }
    .validate(vec![repo()], vec![observed.clone()], vec![], vec![])
    .unwrap();
    assert_eq!(valid.observations().get(&id), Some(&observed));
    let Resolution::Resolved(result) = valid.resolve(&repo()).unwrap() else {
        panic!("expected Resolved")
    };
    assert_eq!(result.selected().id(), &id);
    assert_eq!(result.basis(), ResolutionBasis::ExplicitTask);
}

#[test]
fn rejection_reasons_distinguish_unknown_selection_from_invalid_snapshot() {
    let observed = observation("o1", repo());
    let error = TaskContext {
        selections: vec![CheckoutSelection::new(
            RepositoryId::new("other").unwrap(),
            ObservationId::new("o1").unwrap(),
        )],
        working_area: None,
    }
    .validate(vec![repo()], vec![observed.clone()], vec![], vec![])
    .unwrap_err();
    assert_eq!(
        error.rejections()[0].reason(),
        &RejectionReason::UnknownRepository
    );
    assert_eq!(
        error.rejections()[0].source(),
        Some(SelectionSource::ExplicitTask)
    );
    for (known, observations, expected) in [
        (
            vec![repo(), repo()],
            vec![],
            RejectionReason::DuplicateRepositoryIdentity,
        ),
        (
            vec![],
            vec![observed],
            RejectionReason::ObservationRepositoryAbsent,
        ),
    ] {
        let error = TaskContext::default()
            .validate(known, observations, vec![], vec![])
            .unwrap_err();
        assert_eq!(error.rejections().len(), 1);
        assert_eq!(error.rejections()[0].reason(), &expected);
        assert_eq!(error.rejections()[0].source(), None);
        assert_eq!(error.rejections()[0].selection(), None);
        assert!(!error.to_string().is_empty());
    }
}
