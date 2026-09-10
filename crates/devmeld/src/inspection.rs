//! Read-only presentation: registrations alone, or current publication comparison.
use crate::{Plan, Result, context, declarations, error};
use devmeld_resources::organization::OrganizationPath;
use std::path::Path;

pub fn inspect_in(cwd: &Path, explicit: Option<&Path>, args: &[String]) -> Result<Option<String>> {
    if args == ["status"] {
        return publication_status(cwd, explicit)
            .map(Some)
            .map_err(|cause| error(format!("publication status blocked/unverified: {cause}")));
    }
    let command: Vec<_> = args.iter().map(String::as_str).collect();
    if !matches!(
        command.as_slice(),
        ["resource" | "group", "show", _]
            | ["resource" | "group", "list"]
            | ["resource" | "group", "list", _]
    ) {
        return Ok(None);
    }
    let root = context::select(cwd, explicit)?;
    let mut snapshot = Plan::new(root.clone())?;
    let config = declarations::read_config(&mut snapshot)?;
    let organization = config.organization()?;
    let mut report = format!("Context: {}\n", root.display());
    report.push_str(&format!(
        "Creation defaults (new nodes only): inherit: {}, propagate: {}\n",
        config.defaults.inherit, config.defaults.propagate
    ));
    match command.as_slice() {
        ["resource", "show", address] => report.push_str(&resource_report(&config, address)?),
        [kind, "list", scope @ ..] => {
            let scope = scope
                .first()
                .map(|value| OrganizationPath::new(*value))
                .transpose()?;
            if let Some(scope) = &scope {
                require_group(&organization, scope)?;
            }
            let included = |path: &OrganizationPath| {
                scope
                    .as_ref()
                    .is_none_or(|scope| path.is_descendant_of(scope))
            };
            if *kind == "resource" {
                append_list(
                    &mut report,
                    "Resources",
                    organization
                        .resources()
                        .filter(|(path, _)| included(path))
                        .map(|(path, _)| path.as_str()),
                );
            } else {
                append_list(
                    &mut report,
                    "Groups",
                    organization
                        .groups()
                        .filter(|path| included(path))
                        .map(OrganizationPath::as_str),
                );
            }
        }
        ["group", "show", address] => {
            let group = OrganizationPath::new(*address)?;
            require_group(&organization, &group)?;
            report.push_str(&format!("Group: {address}\n"));
            report.push_str(&crate::render::node_annotations(
                &organization,
                &group,
                crate::language::OutputLanguage::English.messages(),
            )?);
            append_list(
                &mut report,
                "Child groups",
                organization
                    .groups()
                    .filter(|path| path.parent().as_ref() == Some(&group))
                    .map(OrganizationPath::as_str),
            );
            append_list(
                &mut report,
                "Direct resources",
                organization
                    .resources()
                    .filter(|(path, _)| path.parent().as_ref() == Some(&group))
                    .map(|(path, _)| path.as_str()),
            );
        }
        _ => return Err(error("unsupported inspection command")),
    }
    report.push_str("Registration only; relative source/schema references use the shown context root. Source availability, publication and client consumption are not checked.\n");
    Ok(Some(report))
}

fn publication_status(cwd: &Path, explicit: Option<&Path>) -> Result<String> {
    let root = context::select(cwd, explicit)?;
    let mut plan = Plan::new(root.clone())?;
    let config = declarations::read_config(&mut plan)?;
    crate::prepare_publication(&mut plan, &config)?;
    plan.recheck()?;
    let mut report = format!(
        "Context: {}\nConfiguration: saved\nRegistered resources: {}\nConfigured entries: {}\nPublication: {}\n",
        root.display(),
        config.resources.len(),
        config.publication.entries.len(),
        if plan.is_empty() {
            "up to date"
        } else {
            "pending"
        }
    );
    report.push_str(&format!(
        "Navigation: {}\n",
        crate::storage::resolve(&root, &config.publication.directory)?
            .join("index.md")
            .display()
    ));
    for entry in &config.publication.entries {
        let path = crate::storage::resolve(&root, &entry.path)?;
        let owned = plan.owned_paths().any(|p| p == &path);
        let changed = plan.changed_paths().any(|p| p == &path);
        let state = match (owned, changed) {
            (false, _) => "not published",
            (true, true) => "pending update",
            (true, false) => "published; matches current generated content",
        };
        report.push_str(&format!(
            "Entry: {} ({}) — {state}\n",
            path.display(),
            entry.kind
        ));
    }
    for path in plan.changed_paths() {
        report.push_str(&format!("Pending target: {}\n", path.display()));
    }
    report.push_str("Read-only comparison of expected generated content with owned files using current inputs; not a historical source-freshness receipt.\nClient consumption: unverified\n");
    Ok(report)
}

fn require_group(
    organization: &devmeld_resources::organization::Organization,
    group: &OrganizationPath,
) -> Result<()> {
    if organization.groups().any(|path| path == group) {
        Ok(())
    } else {
        Err(error(format!("group not registered: {}", group.as_str())))
    }
}

fn append_list<'a>(report: &mut String, heading: &str, values: impl Iterator<Item = &'a str>) {
    report.push_str(&format!("{heading}:\n"));
    let mut empty = true;
    for value in values {
        report.push_str(&format!("- {value}\n"));
        empty = false;
    }
    if empty {
        report.push_str("(none)\n");
    }
}

fn resource_report(config: &declarations::Config, address: &str) -> Result<String> {
    let id = config.resource_id(address)?;
    let registration = config
        .resources
        .iter()
        .find(|r| r.id == id.as_str())
        .ok_or_else(|| error("missing resource registration"))?;
    let (kind, source) = match (&registration.document, &registration.description) {
        (Some(source), None) => ("document", source),
        (None, Some(source)) => ("description", source),
        _ => return Err(error("expected exactly one source reference")),
    };
    let mut report = format!(
        "Resource: {}\nIdentity: {}\nSource kind: {kind}\nSource: {source:?}\n",
        address,
        id.as_str()
    );
    if let Some(schema) = &registration.attributes_schema {
        report.push_str(&format!("Schema: {schema:?}\n"));
    }
    let organization = config.organization()?;
    report.push_str(&crate::render::node_annotations(
        &organization,
        &OrganizationPath::new(address)?,
        crate::language::OutputLanguage::English.messages(),
    )?);
    for (label, incoming) in [("Access guidance", false), ("Used by", true)] {
        let mut paths = std::collections::BTreeSet::new();
        for association in &config.access {
            let (owner, other) = if incoming {
                (&association.tool, &association.resource)
            } else {
                (&association.resource, &association.tool)
            };
            if owner == id.as_str() {
                let other_id = devmeld_resources::ResourceId::new(other.clone())?;
                let path = organization
                    .path_for(&other_id)
                    .ok_or_else(|| error("association refers to an unregistered resource"))?;
                paths.insert(path.as_str());
            }
        }
        append_list(&mut report, label, paths.into_iter());
    }
    Ok(report)
}
