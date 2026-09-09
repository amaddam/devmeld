use crate::{Result, error, language::OutputLanguage, storage::Plan};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    pub format_version: u32,
    pub resources: Vec<Registration>,
    pub access: Vec<Access>,
    pub publication: Publication,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Registration {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_schema: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Access {
    pub resource: String,
    pub tool: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Publication {
    pub directory: String,
    pub entries: Vec<Entry>,
    #[serde(default, skip_serializing_if = "OutputLanguage::is_default")]
    pub language: OutputLanguage,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Entry {
    pub kind: String,
    pub path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            format_version: 0,
            resources: vec![],
            access: vec![],
            publication: Publication {
                directory: ".devmeld/output".into(),
                entries: vec![],
                language: OutputLanguage::default(),
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Description {
    title: String,
    summary: String,
    #[serde(default)]
    attributes: std::collections::BTreeMap<String, String>,
    #[serde(default)]
    references: Vec<Reference>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Reference {
    label: String,
    path: String,
}
pub(crate) fn encode(value: &impl Serialize) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub(crate) fn read_config(plan: &mut Plan) -> Result<Config> {
    let path = plan.root().join(".devmeld/context.json");
    let bytes = plan
        .capture(&path)?
        .ok_or_else(|| error("context not initialized"))?;
    let config: Config =
        serde_json::from_slice(&bytes).map_err(|e| error(format!("{}: {e}", path.display())))?;
    if config.format_version != 0 {
        return Err(error("unsupported configuration format_version"));
    }
    validate_surfaces(plan, &config)?;
    membership(&config)?;
    plan.require_owned_config()?;
    Ok(config)
}

pub(crate) fn membership(config: &Config) -> Result<devmeld_resources::Membership> {
    let mut membership = devmeld_resources::Membership::default();
    for resource in &config.resources {
        membership.register(devmeld_resources::ResourceId::new(resource.id.clone())?)?;
    }
    for access in &config.access {
        membership.associate(
            devmeld_resources::ResourceId::new(access.resource.clone())?,
            devmeld_resources::ResourceId::new(access.tool.clone())?,
        )?;
    }
    Ok(membership)
}

pub(crate) fn validate_surfaces(plan: &Plan, config: &Config) -> Result<()> {
    use crate::storage::{overlaps, resolve};
    let directory = resolve(plan.root(), &config.publication.directory)?;
    let reserved = [
        plan.root().join(".devmeld/context.json"),
        plan.root().join(".devmeld/state"),
    ];
    if reserved.iter().any(|p| overlaps(p, &directory)) {
        return Err(error("output overlaps reserved configuration/state"));
    }
    let mut entries: Vec<std::path::PathBuf> = Vec::new();
    for entry in &config.publication.entries {
        if !matches!(entry.kind.as_str(), "file" | "instructions") {
            return Err(error("unsupported entry kind"));
        }
        let path = resolve(plan.root(), &entry.path)?;
        plan.check_entry_kind(&path, &entry.kind)?;
        if overlaps(&path, &directory)
            || reserved.iter().any(|p| overlaps(p, &path))
            || entries.iter().any(|p| overlaps(p, &path))
        {
            return Err(error(
                "entry overlaps output, state, configuration or another entry",
            ));
        }
        entries.push(path);
    }
    Ok(())
}

pub(crate) fn load_resources(
    plan: &mut Plan,
    config: &Config,
) -> Result<devmeld_resources::Resources> {
    let mut resources = devmeld_resources::Resources::default();
    for registration in &config.resources {
        let id = devmeld_resources::ResourceId::new(registration.id.clone())?;
        let source = match (
            &registration.document,
            &registration.description,
            &registration.attributes_schema,
        ) {
            (Some(document), None, None) => document,
            (None, Some(description), _) => description,
            _ => {
                return Err(error(
                    "expected a document OR a description with optional schema",
                ));
            }
        };
        let path = super::storage::resolve(plan.root(), source)?;
        let bytes = plan.source(&path)?;
        let resource = if registration.description.is_some() {
            let description: Description = serde_json::from_slice(&bytes)
                .map_err(|e| error(format!("{}: {e}", path.display())))?;
            if let Some(schema_path) = &registration.attributes_schema {
                let schema_path = super::storage::resolve(plan.root(), schema_path)?;
                let schema: serde_json::Value = serde_json::from_slice(&plan.source(&schema_path)?)
                    .map_err(|e| error(format!("{}: {e}", schema_path.display())))?;
                schema_controls(&schema)?;
                let validator = jsonschema::options()
                    .with_draft(jsonschema::Draft::Draft202012)
                    .offline()
                    .build(&schema)
                    .map_err(|e| error(format!("{}: {e}", schema_path.display())))?;
                let attributes = serde_json::to_value(&description.attributes)?;
                validator.validate(&attributes).map_err(|e| {
                    error(format!(
                        "{} attributes {}: {e}",
                        path.display(),
                        e.instance_path()
                    ))
                })?;
            }
            let parent = path
                .parent()
                .ok_or_else(|| error("description has no parent"))?;
            let mut references = Vec::new();
            for reference in description.references {
                let target = super::storage::resolve(parent, &reference.path)?;
                plan.source(&target)?;
                references.push((reference.label, target));
            }
            devmeld_resources::Resource::described(
                id,
                path,
                description.title,
                description.summary,
                description.attributes,
                references,
            )?
        } else {
            devmeld_resources::Resource::document(id, path)?
        };
        resources.add(resource)?;
    }
    Ok(resources)
}

fn schema_controls(schema: &serde_json::Value) -> Result<()> {
    let Some(object) = schema.as_object() else {
        return if schema.is_boolean() {
            Ok(())
        } else {
            Err(error("schema must be an object or boolean"))
        };
    };
    if let Some(dialect) = object.get("$schema") {
        if dialect != "https://json-schema.org/draft/2020-12/schema" {
            return Err(error("only JSON Schema Draft 2020-12 is supported"));
        }
    }
    for keyword in ["$ref", "$dynamicRef"] {
        if let Some(reference) = object.get(keyword) {
            if !reference.as_str().is_some_and(|r| r.starts_with('#')) {
                return Err(error(
                    "only local fragment schema references are supported; external retrieval is disabled",
                ));
            }
        }
    }
    if let Some(vocabularies) = object.get("$vocabulary") {
        let vocabularies = vocabularies
            .as_object()
            .ok_or_else(|| error("$vocabulary must be an object"))?;
        for (name, required) in vocabularies {
            let known = [
                "core",
                "applicator",
                "unevaluated",
                "validation",
                "meta-data",
                "format-annotation",
                "content",
            ]
            .iter()
            .any(|suffix| name == &format!("https://json-schema.org/draft/2020-12/vocab/{suffix}"));
            if !required.is_boolean() || (required == true && !known) {
                return Err(error(format!(
                    "unsupported required schema vocabulary: {name}"
                )));
            }
        }
    }
    let single = [
        "items",
        "contains",
        "additionalProperties",
        "unevaluatedProperties",
        "unevaluatedItems",
        "propertyNames",
        "not",
        "if",
        "then",
        "else",
        "contentSchema",
    ];
    let maps = [
        "$defs",
        "properties",
        "patternProperties",
        "dependentSchemas",
    ];
    let arrays = ["prefixItems", "allOf", "anyOf", "oneOf"];
    let leaf = [
        "$schema",
        "$id",
        "$anchor",
        "$dynamicAnchor",
        "$ref",
        "$dynamicRef",
        "$vocabulary",
        "$comment",
        "type",
        "enum",
        "const",
        "multipleOf",
        "maximum",
        "exclusiveMaximum",
        "minimum",
        "exclusiveMinimum",
        "maxLength",
        "minLength",
        "pattern",
        "maxItems",
        "minItems",
        "uniqueItems",
        "maxContains",
        "minContains",
        "maxProperties",
        "minProperties",
        "required",
        "dependentRequired",
        "title",
        "description",
        "default",
        "deprecated",
        "readOnly",
        "writeOnly",
        "examples",
        "format",
        "contentEncoding",
        "contentMediaType",
    ];
    for (keyword, value) in object {
        if single.contains(&keyword.as_str()) {
            schema_controls(value)?;
        } else if maps.contains(&keyword.as_str()) {
            for child in value
                .as_object()
                .ok_or_else(|| error(format!("{keyword} must be an object")))?
                .values()
            {
                schema_controls(child)?;
            }
        } else if arrays.contains(&keyword.as_str()) {
            for child in value
                .as_array()
                .ok_or_else(|| error(format!("{keyword} must be an array")))?
            {
                schema_controls(child)?;
            }
        } else if !leaf.contains(&keyword.as_str()) {
            return Err(error(format!("unsupported schema keyword: {keyword}")));
        }
    }
    Ok(())
}
