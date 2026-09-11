//! Filesystem spelling for published group pages and cards, not domain identity.
use crate::{Result, error};
use devmeld_resources::organization::{Organization, OrganizationPath};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

pub(crate) struct Pages {
    pub resources: BTreeMap<String, PathBuf>,
    pub groups: BTreeMap<OrganizationPath, PathBuf>,
}

pub(crate) fn pages(directory: &Path, organization: &Organization) -> Result<Pages> {
    let mut pages = Pages {
        resources: BTreeMap::new(),
        groups: BTreeMap::new(),
    };
    let mut prefixes = BTreeMap::new();
    for (address, identity) in organization.resources() {
        let page = claim_page(
            directory,
            address.as_str(),
            &format!("resource {}", address.as_str()),
            &mut prefixes,
        )?;
        pages.resources.insert(identity.as_str().to_owned(), page);
    }
    for group in organization.groups() {
        let path = format!("{}/{}", group.as_str(), group.leaf());
        let page = claim_page(
            directory,
            &path,
            &format!("group {}", group.as_str()),
            &mut prefixes,
        )?;
        pages.groups.insert(group.clone(), page);
    }
    Ok(pages)
}

fn claim_page(
    directory: &Path,
    address: &str,
    owner: &str,
    prefixes: &mut BTreeMap<String, (String, bool, String)>,
) -> Result<PathBuf> {
    let mut page = directory.join("resources");
    let mut relative = String::new();
    let mut segments = address.split('/').peekable();
    while let Some(segment) = segments.next() {
        let mut name = portable_segment(segment);
        let is_file = segments.peek().is_none();
        if is_file {
            name.push_str(".md");
        }
        if !relative.is_empty() {
            relative.push('/');
        }
        relative.push_str(&name);
        // Group documents and resource cards share a filesystem namespace.
        // Validate ancestors too; never merge case-folded names on any host.
        let key = relative.to_lowercase();
        if let Some((spelling, was_file, previous_owner)) = prefixes.get(&key) {
            if spelling != &relative || *was_file || is_file {
                return Err(error(format!(
                    "published path collision between '{previous_owner}' and '{owner}' at resources/{relative}; choose distinct logical addresses"
                )));
            }
        } else {
            prefixes.insert(key, (relative.clone(), is_file, owner.to_owned()));
        }
        page.push(name);
    }
    Ok(page)
}

fn portable_segment(segment: &str) -> String {
    let stem = segment
        .split('.')
        .next()
        .unwrap_or(segment)
        .trim_end_matches(' ')
        .to_ascii_uppercase();
    let device = matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || ["COM", "LPT"].iter().any(|prefix| {
            stem.strip_prefix(prefix).is_some_and(|suffix| {
                matches!(
                    suffix,
                    "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
                )
            })
        });
    let mut name = String::new();
    for (offset, character) in segment.char_indices() {
        // Escape the escape character too, so authored %3F differs from a ?.
        // Apply Windows spelling on every host; logical names remain unchanged.
        if matches!(
            character,
            '%' | '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
        ) || (character == '.' && offset + 1 == segment.len())
            || (device && offset == 0)
        {
            name.push_str(&format!("%{:02X}", character as u32));
        } else {
            name.push(character);
        }
    }
    name
}
