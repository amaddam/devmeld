use devmeld_knowledge::*;
use devmeld_shared_kernel::RepositoryId;

#[test]
fn exact_dimensions_match_but_missing_and_intervals_remain_unknown() {
    let scope = Scope::new(ScopeInput {
        repository: Some(RepositoryId::new("repo").unwrap()),
        checkout: Some("obs".into()),
        revision: Some("abc".into()),
        environment: Some("test".into()),
        working_context: Some("coding".into()),
        version_interval: Some(">=1".into()),
    })
    .unwrap();
    let matched = scope.match_query(&scope);
    assert_eq!(matched.overall(), MatchState::Unknown);
    assert_eq!(
        matched
            .dimensions()
            .iter()
            .filter(|d| d.state() == MatchState::Match)
            .count(),
        5
    );
    assert_eq!(
        matched.dimensions().last().unwrap().dimension(),
        ScopeDimension::VersionInterval
    );
    let unspecified = Scope::new(ScopeInput::default()).unwrap();
    assert_eq!(
        unspecified.match_query(&unspecified).overall(),
        MatchState::Unknown
    );
}

#[test]
fn mismatch_wins_over_unknown_without_mutating_scope() {
    let declared = Scope::new(ScopeInput {
        environment: Some("prod".into()),
        ..Default::default()
    })
    .unwrap();
    let query = Scope::new(ScopeInput {
        environment: Some("test".into()),
        ..Default::default()
    })
    .unwrap();
    let original = declared.clone();
    assert_eq!(declared.match_query(&query).overall(), MatchState::Mismatch);
    assert_eq!(declared, original);
    assert!(
        Scope::new(ScopeInput {
            environment: Some(" ".into()),
            ..Default::default()
        })
        .is_err()
    );
}

#[test]
fn summary_compares_only_mentioned_dimensions_without_claiming_universal_scope() {
    let repository = Scope::new(ScopeInput {
        repository: Some(RepositoryId::new("repo").unwrap()),
        ..Default::default()
    })
    .unwrap();
    let matched = repository.match_query(&repository);
    assert_eq!(matched.overall(), MatchState::Match);
    assert_eq!(matched.dimensions()[1].state(), MatchState::Unknown);
    assert!(!matched.dimensions()[1].compared());
    let unspecified = Scope::new(ScopeInput::default()).unwrap();
    assert_eq!(
        repository.match_query(&unspecified).overall(),
        MatchState::Unknown
    );
    assert_eq!(
        unspecified.match_query(&repository).overall(),
        MatchState::Unknown
    );
    assert_eq!(
        unspecified.match_query(&unspecified).overall(),
        MatchState::Unknown
    );
    let interval = Scope::new(ScopeInput {
        repository: repository.repository().cloned(),
        version_interval: Some(">=1".into()),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(
        interval.match_query(&interval).overall(),
        MatchState::Unknown
    );
    assert!(interval.match_query(&interval).dimensions()[5].compared());
}
