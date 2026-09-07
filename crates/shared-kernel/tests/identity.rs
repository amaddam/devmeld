use devmeld_shared_kernel::{RepositoryId, ResourceId};
use std::collections::HashSet;

#[test]
fn identities_preserve_opaque_unicode_without_normalization() {
    for text in ["repo-1", "项目/主仓", "é", "e\u{301}", "a b"] {
        assert_eq!(RepositoryId::new(text).unwrap().as_str(), text);
        assert_eq!(ResourceId::new(text).unwrap().as_str(), text);
    }
    assert_ne!(RepositoryId::new("é"), RepositoryId::new("e\u{301}"));
    assert_ne!(RepositoryId::new("A"), RepositoryId::new("a"));
}

#[test]
fn malformed_ids_are_rejected_for_both_kinds() {
    for text in [
        "", " ", " a", "a ", "\u{a0}a", "a\u{a0}", "a\n", "a\0b", "a\u{7f}", "a\u{85}b",
    ] {
        assert!(RepositoryId::new(text).is_err(), "{text:?}");
        assert!(ResourceId::new(text).is_err(), "{text:?}");
    }
}

#[test]
fn equality_and_hashing_use_identity_not_allocation() {
    let id = RepositoryId::new("repo").unwrap();
    let clone = id.clone();
    assert_eq!(id, clone);
    assert_eq!(HashSet::from([id.clone(), clone]).len(), 1);
    assert_eq!(id.to_string(), "repo");
    assert_eq!(ResourceId::new("res").unwrap().to_string(), "res");
}

#[test]
fn caller_changes_do_not_mutate_an_identity() {
    let mut input = String::from("original");
    let id = ResourceId::new(&input).unwrap();
    input.clear();
    assert_eq!(id.as_str(), "original");
}
