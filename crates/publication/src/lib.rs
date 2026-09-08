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
}
impl fmt::Display for Conflict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnownedOrEdited => "unowned or externally edited target",
            Self::MissingOwned => "managed target is missing",
            Self::SourceOverlap => "publication overlaps an input source",
        })
    }
}
impl std::error::Error for Conflict {}

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
