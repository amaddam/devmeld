use devmeld_knowledge::*;
use devmeld_shared_kernel::{RepositoryId, ResourceId};
use std::time::UNIX_EPOCH;

fn repo() -> RepositoryId {
    RepositoryId::new("repo").unwrap()
}
fn source() -> SourceFact {
    SourceFact::new("vault", "r1", SourceType::Declared).unwrap()
}
fn scope() -> Scope {
    Scope::new(ScopeInput {
        repository: Some(repo()),
        ..Default::default()
    })
    .unwrap()
}
fn resource() -> ResourceFact {
    ResourceFact::new(
        ResourceId::new("note").unwrap(),
        source(),
        vec![],
        scope(),
        ReviewStatus::Accepted,
        ValidityStatus::Superseded,
    )
    .unwrap()
}
fn checkout(repository: RepositoryId, revision: &str) -> CheckoutFacts {
    CheckoutFacts::new(CheckoutFactInput {
        repository,
        observation: "o1".into(),
        branch: Some("main".into()),
        revision: Some(revision.into()),
        working_tree: "clean".into(),
        observed_at: UNIX_EPOCH,
        observer_revision: "git/1".into(),
        basis: CheckoutBasis::ExplicitTask,
    })
    .unwrap()
}

#[test]
fn repository_only_result_does_not_invent_checkout_requirement() {
    let fact = resource();
    let original = fact.clone();
    let result = fact
        .evaluate(scope(), None, Some("lexical match".into()))
        .unwrap();
    assert!(result.checkout().is_none());
    assert_eq!(result.fact(), &original);
    assert_eq!(result.fact().validity(), ValidityStatus::Superseded);
    assert_eq!(result.fact().review(), ReviewStatus::Accepted);
    assert_eq!(fact, original);
}

#[test]
fn checkout_query_requires_consistent_repository_revision_and_receipt() {
    let query = Scope::new(ScopeInput {
        repository: Some(repo()),
        checkout: Some("o1".into()),
        revision: Some("abc".into()),
        ..Default::default()
    })
    .unwrap();
    assert!(resource().evaluate(query.clone(), None, None).is_err());
    assert!(
        resource()
            .evaluate(query.clone(), Some(checkout(repo(), "wrong")), None)
            .is_err()
    );
    assert!(
        resource()
            .evaluate(
                query.clone(),
                Some(checkout(RepositoryId::new("other").unwrap(), "abc")),
                None
            )
            .is_err()
    );
    let result = resource()
        .evaluate(query, Some(checkout(repo(), "abc")), None)
        .unwrap();
    assert_eq!(result.checkout().unwrap().revision(), Some("abc"));
}

#[test]
fn relevance_cannot_hide_mismatch_or_unavailable_relation_target() {
    let target = ObjectId::Resource(ResourceId::new("target").unwrap());
    let relation = Relation::new(
        RelationIdentity::Explicit("rel".into()),
        ObjectId::Resource(ResourceId::new("note").unwrap()),
        RelationKind::References,
        target.clone(),
        source(),
        vec![],
        scope(),
        ReviewStatus::Accepted,
        ValidityStatus::Current,
    )
    .unwrap();
    let query = Scope::new(ScopeInput {
        repository: Some(RepositoryId::new("other").unwrap()),
        ..Default::default()
    })
    .unwrap();
    let result = relation
        .evaluate(
            query,
            None,
            TargetAvailability::Unavailable("source missing".into()),
            Some("high relevance".into()),
        )
        .unwrap();
    assert_eq!(result.scope_match().overall(), MatchState::Mismatch);
    assert_eq!(result.fact().target(), &target);
    assert!(matches!(
        result.target_availability(),
        TargetAvailability::Unavailable(_)
    ));
}

#[test]
fn every_source_review_validity_combination_survives_query_recomputation() {
    let mismatch = Scope::new(ScopeInput {
        repository: Some(RepositoryId::new("elsewhere").unwrap()),
        ..Default::default()
    })
    .unwrap();
    for source_type in [
        SourceType::Declared,
        SourceType::Observed,
        SourceType::Inferred,
    ] {
        for review in [
            ReviewStatus::Unreviewed,
            ReviewStatus::Accepted,
            ReviewStatus::Rejected,
        ] {
            for validity in [
                ValidityStatus::Current,
                ValidityStatus::Superseded,
                ValidityStatus::Invalid,
                ValidityStatus::Unknown,
            ] {
                let evidence = Evidence::new(
                    source(),
                    EvidenceSupport::Derivation("fixture/1 inputs: note.md@r1".into()),
                    None,
                )
                .unwrap();
                let fact = ResourceFact::new(
                    ResourceId::new("note").unwrap(),
                    SourceFact::new("producer", "1", source_type).unwrap(),
                    vec![evidence],
                    scope(),
                    review,
                    validity,
                )
                .unwrap();
                let original = fact.clone();
                let yes = fact.evaluate(scope(), None, None).unwrap();
                let no = fact
                    .evaluate(mismatch.clone(), None, Some("highest relevance".into()))
                    .unwrap();
                assert_eq!(yes.scope_match().overall(), MatchState::Match);
                assert_eq!(no.scope_match().overall(), MatchState::Mismatch);
                for result in [yes, no] {
                    assert_eq!(result.fact(), &original);
                    assert_eq!(result.fact().source().source_type(), source_type);
                    assert_eq!(result.fact().review(), review);
                    assert_eq!(result.fact().validity(), validity);
                }
                assert_eq!(fact, original);
            }
        }
    }
}

#[test]
fn relation_results_require_the_same_checkout_consistency_as_resources() {
    let relation = Relation::new(
        RelationIdentity::Explicit("r".into()),
        ObjectId::Repository(repo()),
        RelationKind::References,
        ObjectId::Resource(ResourceId::new("note").unwrap()),
        source(),
        vec![],
        scope(),
        ReviewStatus::Unreviewed,
        ValidityStatus::Unknown,
    )
    .unwrap();
    let query = Scope::new(ScopeInput {
        repository: Some(repo()),
        checkout: Some("o2".into()),
        ..Default::default()
    })
    .unwrap();
    assert!(
        relation
            .evaluate(query.clone(), None, TargetAvailability::Available, None)
            .is_err()
    );
    assert!(
        relation
            .evaluate(
                query,
                Some(checkout(repo(), "abc")),
                TargetAvailability::Available,
                None
            )
            .is_err()
    );
    assert!(
        relation
            .evaluate(scope(), None, TargetAvailability::Unknown(" ".into()), None)
            .is_err()
    );
    assert!(
        relation
            .evaluate(
                scope(),
                None,
                TargetAvailability::Available,
                Some(" ".into())
            )
            .is_err()
    );
}
