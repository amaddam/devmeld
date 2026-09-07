use devmeld_knowledge::*;
use devmeld_shared_kernel::{RepositoryId, ResourceId};

fn source() -> SourceFact {
    SourceFact::new("vault", "rev1", SourceType::Declared).unwrap()
}
fn evidence() -> Evidence {
    Evidence::new(
        source(),
        EvidenceSupport::Locator {
            locator: "note.md".into(),
            lines: Some((2, 5)),
        },
        Some("sha256:abc".into()),
    )
    .unwrap()
}

#[test]
fn evidence_retains_revision_range_hash_without_implying_status() {
    let item = evidence();
    assert_eq!(item.source().revision(), "rev1");
    assert_eq!(item.content_hash(), Some("sha256:abc"));
    assert!(
        Evidence::new(
            source(),
            EvidenceSupport::Locator {
                locator: "note.md".into(),
                lines: Some((5, 2))
            },
            None
        )
        .is_err()
    );
    assert!(Evidence::new(source(), EvidenceSupport::Derivation(" ".into()), None).is_err());
}

#[test]
fn resource_keeps_independent_statuses_and_support() {
    let fact = ResourceFact::new(
        ResourceId::new("note").unwrap(),
        source(),
        vec![evidence()],
        Scope::new(ScopeInput::default()).unwrap(),
        ReviewStatus::Accepted,
        ValidityStatus::Invalid,
    )
    .unwrap();
    assert_eq!(fact.review(), ReviewStatus::Accepted);
    assert_eq!(fact.validity(), ValidityStatus::Invalid);
    assert_eq!(fact.evidence().len(), 1);
    let inferred = SourceFact::new("extractor", "rev1", SourceType::Inferred).unwrap();
    assert!(
        ResourceFact::new(
            ResourceId::new("x").unwrap(),
            inferred,
            vec![],
            Scope::new(ScopeInput::default()).unwrap(),
            ReviewStatus::Unreviewed,
            ValidityStatus::Unknown
        )
        .is_err()
    );
}

#[test]
fn relation_is_typed_directed_and_requires_support_for_nondeclared_claims() {
    let from = ObjectId::Resource(ResourceId::new("note").unwrap());
    let to = ObjectId::Repository(RepositoryId::new("repo").unwrap());
    let make = |from, to, evidence| {
        Relation::new(
            RelationIdentity::Explicit("rel".into()),
            from,
            RelationKind::Explains,
            to,
            SourceFact::new("extractor", "rev1", SourceType::Inferred).unwrap(),
            evidence,
            Scope::new(ScopeInput::default()).unwrap(),
            ReviewStatus::Accepted,
            ValidityStatus::Current,
        )
    };
    assert!(make(from.clone(), to.clone(), vec![]).is_err());
    let forward = make(from.clone(), to.clone(), vec![evidence()]).unwrap();
    let reverse = make(to.clone(), from.clone(), vec![evidence()]).unwrap();
    assert_ne!(forward, reverse);
    assert_eq!(forward.target(), &to);
    assert_eq!(forward.source_object(), &from);
}
