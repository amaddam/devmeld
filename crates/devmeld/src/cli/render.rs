//! English terminal presentation over structured application results.
use devmeld::{
    EntryChange, EntryKind, EntryPublicationState, Inspection, InspectionView, NodeAnnotations,
    NodeKind, PlanPreview, ResourceSource,
};
use devmeld_resources::organization::OrganizationPath;

pub(crate) fn inspection(value: &Inspection) -> String {
    let mut report = format!("Context: {}\n", value.context_root.display());
    if let InspectionView::Publication(status) = &value.view {
        report.push_str(&format!("Configuration: saved\nRegistered resources: {}\nConfigured entries: {}\nPublication: {}\nNavigation: {}\n",
            status.resource_count, status.entries.len(), if status.pending {"pending"} else {"up to date"}, status.navigation.display()));
        for entry in &status.entries {
            let kind = match entry.kind {
                EntryKind::File => "file",
                EntryKind::Instructions => "instructions",
            };
            let state = match entry.state {
                EntryPublicationState::Unpublished => "not published",
                EntryPublicationState::PendingUpdate => "pending update",
                EntryPublicationState::Published => "published; matches current generated content",
            };
            report.push_str(&format!(
                "Entry: {} ({kind}) — {state}\n",
                entry.path.display()
            ));
        }
        for path in &status.pending_targets {
            report.push_str(&format!("Pending target: {}\n", path.display()));
        }
        report.push_str("Read-only comparison of expected generated content with owned files using current inputs; not a historical source-freshness receipt.\nClient consumption: unverified\n");
        return report;
    }
    report.push_str(&format!(
        "Creation defaults (new nodes only): inherit: {}, propagate: {}\n",
        value.defaults.inherit(),
        value.defaults.propagate()
    ));
    match &value.view {
        InspectionView::List { kind, paths } => append_list(
            &mut report,
            if *kind == NodeKind::Resource {
                "Resources"
            } else {
                "Groups"
            },
            paths,
        ),
        InspectionView::Group(group) => {
            report.push_str(&format!("Group: {}\n", group.path.as_str()));
            report.push_str(&annotations(&group.annotations));
            append_list(&mut report, "Child groups", &group.child_groups);
            append_list(&mut report, "Direct resources", &group.resources);
        }
        InspectionView::Resource(resource) => {
            let (kind, file, schema) = match &resource.source {
                ResourceSource::Document(file) => ("document", file, None),
                ResourceSource::Description { file, schema } => {
                    ("description", file, schema.as_ref())
                }
            };
            report.push_str(&format!(
                "Resource: {}\nIdentity: {}\nSource kind: {kind}\nSource: {file:?}\n",
                resource.address.as_str(),
                resource.id.as_str()
            ));
            if let Some(schema) = schema {
                report.push_str(&format!("Schema: {schema:?}\n"));
            }
            report.push_str(&annotations(&resource.annotations));
            append_list(&mut report, "Access guidance", &resource.access_guidance);
            append_list(&mut report, "Used by", &resource.used_by);
        }
        InspectionView::Publication(_) => unreachable!("handled above"),
    }
    report.push_str("Registration only; relative source/schema references use the shown context root. Source availability, publication and client consumption are not checked.\n");
    report
}
fn append_list(report: &mut String, heading: &str, values: &[OrganizationPath]) {
    report.push_str(&format!("{heading}:\n"));
    for value in values {
        report.push_str(&format!("- {}\n", value.as_str()));
    }
    if values.is_empty() {
        report.push_str("(none)\n");
    }
}
fn annotations(value: &NodeAnnotations) -> String {
    let local = &value.local;
    let mut result = String::new();
    if !local.is_empty() {
        result.push_str("Context annotations (local):\n");
        if let Some(description) = local.description() {
            result.push_str(&format!("- Description: {}\n", text(description)));
        }
        let tags = local.tags().map(|tag| text(tag)).collect::<Vec<_>>();
        if !tags.is_empty() {
            result.push_str(&format!("- Tags: {}\n", tags.join(", ")));
        }
        if !local.fields().is_empty() {
            result.push_str("- Fields:\n");
            for (key, value) in local.fields() {
                result.push_str(&format!("  - {}: {}\n", text(key), text(value)));
            }
        }
        result.push('\n');
    }
    result.push_str(&format!(
        "Saved inheritance choices: inherit: {}",
        value.inherit
    ));
    if let Some(propagate) = value.propagate {
        result.push_str(&format!(", propagate: {propagate}"));
    }
    result.push_str("\n\n");
    if value.inherit {
        result.push_str("Effective tags and fields:\n");
        if value.effective.is_empty() {
            result.push_str("(none; only enabled parent/child edges pass tags and fields)\n");
        }
        if !value.effective.tags().is_empty() {
            result.push_str("- Tags:\n");
            for (tag, origins) in value.effective.tags() {
                let origins = origins
                    .iter()
                    .map(|p| text(p.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ");
                result.push_str(&format!("  - {} (Origin: {origins})\n", text(tag)));
            }
        }
        if !value.effective.fields().is_empty() {
            result.push_str("- Fields:\n");
            for (key, field) in value.effective.fields() {
                result.push_str(&format!(
                    "  - {}: {} (Origin: {})\n",
                    text(key),
                    text(field.value()),
                    text(field.origin().as_str())
                ));
            }
        }
        result.push('\n');
    }
    result
}
// Keep existing report escaping; generated Markdown has its own presentation policy.
fn text(value: &str) -> String {
    value
        .chars()
        .flat_map(|c| match c {
            '&' => "&amp;".chars().collect::<Vec<_>>(),
            '<' => "&lt;".chars().collect(),
            '>' => "&gt;".chars().collect(),
            '\n' | '\r' => vec![' '],
            '\\' | '`' | '*' | '_' | '[' | ']' | '(' | ')' | '#' | '!' | '|' => vec!['\\', c],
            _ if c.is_control() => vec![' '],
            _ => vec![c],
        })
        .collect()
}
pub(crate) fn preview(value: PlanPreview<'_>) -> String {
    let bytes = |value: Option<&[u8]>| {
        value
            .map(|b| String::from_utf8_lossy(b).into_owned())
            .unwrap_or_else(|| "<absent>".into())
    };
    match value {
        PlanPreview::Recovery { committed, targets } => {
            let mut output = format!(
                "Recovery: {}\n",
                if committed {
                    "finish committed-operation cleanup"
                } else {
                    "restore unfinished operation"
                }
            );
            for target in targets {
                if committed {
                    output.push_str(&format!(
                        "\nLeave committed target unchanged: {}\n",
                        target.path.display()
                    ));
                } else {
                    output.push_str(&format!(
                        "\nTarget: {}\nRestore:\n{}\n",
                        target.path.display(),
                        bytes(target.before)
                    ));
                }
            }
            output
        }
        PlanPreview::Changes {
            configuration,
            targets,
        } => {
            let mut output = format!("{} changed target(s)\n", targets.len());
            if configuration {
                output.push_str("Registration/configuration only; run sync separately to publish entry changes.\n");
            }
            if !targets.is_empty() {
                output.push_str("Applying also records local ownership/recovery evidence under .devmeld/state.\n");
            }
            for target in targets {
                if let Some(entry) = target.entry {
                    let label = match entry {
                        EntryChange::Detach => "detach; retain host",
                        EntryChange::CreateHost => "attach; create host",
                        EntryChange::Update => "attach/update; preserve outside text",
                    };
                    output.push_str(&format!(
                        "\nInstruction insertion: {} ({label})\n",
                        target.path.display()
                    ));
                }
                output.push_str(&format!(
                    "\nTarget: {}\nBefore:\n{}\nAfter:\n{}\n",
                    target.path.display(),
                    bytes(target.before),
                    bytes(target.after)
                ));
            }
            output
        }
    }
}
