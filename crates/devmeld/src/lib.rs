//! Reusable local-context application boundary. Terminal syntax lives in the binary.
mod application;
mod context;
mod declarations;
mod inspection;
mod instructions;
mod language;
mod markdown;
mod page_paths;
mod records;
mod render;
mod request;
mod storage;

pub use application::prepare;
pub use inspection::*;
pub use language::OutputLanguage;
pub use request::*;
#[cfg(test)]
extern crate self as devmeld;
#[cfg(test)]
#[path = "../tests/support/mod.rs"]
mod test_support;
pub use storage::{EntryChange, Plan, PlanPreview, RecoveryTarget, TargetChange};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn error(message: impl Into<String>) -> Box<dyn std::error::Error> {
    std::io::Error::other(message.into()).into()
}
