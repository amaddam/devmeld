//! Context Publication: pure managed-change rules.
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    Unchanged,
    Create,
    Replace,
    Remove,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Conflict {
    UnownedOrEdited,
    MissingOwned,
    SourceOverlap,
    InsertionConflict,
    OwnershipMode,
}
impl fmt::Display for Conflict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnownedOrEdited => "unowned or externally edited target",
            Self::MissingOwned => "managed target is missing",
            Self::SourceOverlap => "publication overlaps an input source",
            Self::InsertionConflict => {
                "instruction insertion evidence is missing, edited or ambiguous"
            }
            Self::OwnershipMode => "incompatible ownership mode; detach before changing entry kind",
        })
    }
}
impl std::error::Error for Conflict {}

/// Recognition comes from exact, context-bound insertion evidence, never host identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryEvidence {
    UnclaimedClean,
    MatchingInsertion,
    Conflict,
    WholeFile,
}

/// Shared-host permission is deliberately different from whole-file ownership.
pub fn authorize_entry(
    present: bool,
    evidence: EntryEvidence,
    detach: bool,
    overlap: bool,
) -> Result<(), Conflict> {
    if overlap {
        return Err(Conflict::SourceOverlap);
    }
    match evidence {
        EntryEvidence::WholeFile => Err(Conflict::OwnershipMode),
        EntryEvidence::Conflict => Err(Conflict::InsertionConflict),
        EntryEvidence::MatchingInsertion if !present => Err(Conflict::MissingOwned),
        EntryEvidence::UnclaimedClean if detach => Err(Conflict::InsertionConflict),
        _ => Ok(()),
    }
}

/// Ownership is evidence supplied by the adapter, not inferred from equal output.
pub fn classify(
    before: Option<&[u8]>,
    after: Option<&[u8]>,
    owned: bool,
    evidence_matches: bool,
    source_overlap: bool,
) -> Result<Change, Conflict> {
    if source_overlap {
        return Err(Conflict::SourceOverlap);
    }
    if before.is_some() && (!owned || !evidence_matches) {
        return Err(Conflict::UnownedOrEdited);
    }
    if before.is_none() && owned {
        return Err(Conflict::MissingOwned);
    }
    if before == after {
        return Ok(Change::Unchanged);
    }
    Ok(match (before, after) {
        (None, Some(_)) => Change::Create,
        (Some(_), None) => Change::Remove,
        _ => Change::Replace,
    })
}
