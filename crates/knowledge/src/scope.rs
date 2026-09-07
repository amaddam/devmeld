use crate::{KnowledgeError, text};
use devmeld_shared_kernel::RepositoryId;

#[derive(Clone, Debug, Default)]
pub struct ScopeInput {
    pub repository: Option<RepositoryId>,
    pub checkout: Option<String>,
    pub revision: Option<String>,
    pub environment: Option<String>,
    pub working_context: Option<String>,
    pub version_interval: Option<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scope {
    repository: Option<RepositoryId>,
    checkout: Option<String>,
    revision: Option<String>,
    environment: Option<String>,
    working_context: Option<String>,
    version_interval: Option<String>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScopeDimension {
    Repository,
    Checkout,
    Revision,
    Environment,
    WorkingContext,
    VersionInterval,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchState {
    Match,
    Mismatch,
    Unknown,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DimensionMatch {
    dimension: ScopeDimension,
    state: MatchState,
    compared: bool,
    explanation: String,
}
impl DimensionMatch {
    pub fn dimension(&self) -> ScopeDimension {
        self.dimension
    }
    pub fn state(&self) -> MatchState {
        self.state
    }
    /// Whether either Scope mentions this dimension. Both absent is still
    /// Unknown, but contributes no claim to the comparison summary.
    pub fn compared(&self) -> bool {
        self.compared
    }
    pub fn explanation(&self) -> &str {
        &self.explanation
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopeMatch {
    dimensions: Vec<DimensionMatch>,
    overall: MatchState,
}
impl ScopeMatch {
    pub fn dimensions(&self) -> &[DimensionMatch] {
        &self.dimensions
    }
    /// Summarizes mentioned dimensions only, not universal applicability.
    /// An entirely unspecified comparison remains Unknown.
    pub fn overall(&self) -> MatchState {
        self.overall
    }
}
impl Scope {
    pub fn new(input: ScopeInput) -> Result<Self, KnowledgeError> {
        for value in [
            &input.checkout,
            &input.revision,
            &input.environment,
            &input.working_context,
            &input.version_interval,
        ]
        .into_iter()
        .flatten()
        {
            text(value, "Scope dimension")?;
        }
        Ok(Self {
            repository: input.repository,
            checkout: input.checkout,
            revision: input.revision,
            environment: input.environment,
            working_context: input.working_context,
            version_interval: input.version_interval,
        })
    }
    pub fn repository(&self) -> Option<&RepositoryId> {
        self.repository.as_ref()
    }
    pub fn checkout(&self) -> Option<&str> {
        self.checkout.as_deref()
    }
    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }
    pub fn environment(&self) -> Option<&str> {
        self.environment.as_deref()
    }
    pub fn working_context(&self) -> Option<&str> {
        self.working_context.as_deref()
    }
    pub fn version_interval(&self) -> Option<&str> {
        self.version_interval.as_deref()
    }
    pub fn match_query(&self, query: &Scope) -> ScopeMatch {
        let pairs = [
            (
                ScopeDimension::Repository,
                self.repository().map(RepositoryId::as_str),
                query.repository().map(RepositoryId::as_str),
            ),
            (ScopeDimension::Checkout, self.checkout(), query.checkout()),
            (ScopeDimension::Revision, self.revision(), query.revision()),
            (
                ScopeDimension::Environment,
                self.environment(),
                query.environment(),
            ),
            (
                ScopeDimension::WorkingContext,
                self.working_context(),
                query.working_context(),
            ),
            (
                ScopeDimension::VersionInterval,
                self.version_interval(),
                query.version_interval(),
            ),
        ];
        let dimensions: Vec<_> = pairs
            .into_iter()
            .map(|(dimension, declared, requested)| {
                let (state, explanation) = if dimension == ScopeDimension::VersionInterval {
                    (
                        MatchState::Unknown,
                        "version-interval grammar and comparison semantics are not accepted"
                            .to_owned(),
                    )
                } else {
                    match (declared, requested) {
                        (Some(a), Some(b)) if a == b => {
                            (MatchState::Match, format!("exact value matches: {a}"))
                        }
                        (Some(a), Some(b)) => {
                            (MatchState::Mismatch, format!("declared {a}; query {b}"))
                        }
                        _ => (
                            MatchState::Unknown,
                            "dimension unspecified in declared or query Scope".to_owned(),
                        ),
                    }
                };
                DimensionMatch {
                    dimension,
                    state,
                    compared: declared.is_some() || requested.is_some(),
                    explanation,
                }
            })
            .collect();
        let overall = if dimensions.iter().any(|d| d.state == MatchState::Mismatch) {
            MatchState::Mismatch
        } else if !dimensions.iter().any(|d| d.compared)
            || dimensions
                .iter()
                .any(|d| d.compared && d.state == MatchState::Unknown)
        {
            MatchState::Unknown
        } else {
            MatchState::Match
        };
        ScopeMatch {
            dimensions,
            overall,
        }
    }
}
