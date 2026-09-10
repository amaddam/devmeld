//! Convert terminal syntax into application requests; no filesystem access.
use super::{Operation, annotation_args};
use devmeld::{
    DefaultSetting, EntryKind, InitOptions, Mutation, NodeKind, OutputLanguage, Query,
    RegisterResource, ResourceSource, Result, error,
};
use devmeld_resources::organization::OrganizationPath;
use std::path::PathBuf;

pub(super) fn operation(args: &[String]) -> Result<Operation> {
    let args: Vec<_> = args.iter().map(String::as_str).collect();
    let kind = |name| {
        if name == "resource" {
            NodeKind::Resource
        } else {
            NodeKind::Group
        }
    };
    let path = |value: &str| -> Result<OrganizationPath> { Ok(OrganizationPath::new(value)?) };
    match args.as_slice() {
        ["resource" | "group", "list", scope @ ..] if scope.len() <= 1 => {
            return Ok(Operation::Query(Query::List {
                kind: kind(args[0]),
                scope: scope.first().map(|value| path(value)).transpose()?,
            }));
        }
        ["resource" | "group", "show", address] => {
            return Ok(Operation::Query(Query::Show {
                kind: kind(args[0]),
                path: path(address)?,
            }));
        }
        ["status"] => return Ok(Operation::Query(Query::PublicationStatus)),
        _ => {}
    }
    let request = match args.as_slice() {
        ["init", options @ ..] => {
            let mut result = InitOptions::default();
            for pair in options.chunks(2) {
                match pair {
                    ["--entry", file] => result.entries.push((EntryKind::File, file.into())),
                    ["--instruction-entry", file] => {
                        result.entries.push((EntryKind::Instructions, file.into()))
                    }
                    ["--output", directory] => result.output = Some(directory.into()),
                    ["--language", value] if result.language.is_none() => {
                        result.language = Some(language(value)?)
                    }
                    _ => {
                        return Err(error(
                            "init accepts --entry <ENTRY_FILE>, --instruction-entry <ENTRY_FILE>, --output <OUTPUT_DIR> and one --language <en|zh-CN>",
                        ));
                    }
                }
            }
            Mutation::Initialize(result)
        }
        ["resource", "add", source, options @ ..] => {
            let node = annotation_args::parse(&strings(options))?;
            let mut address = None;
            let mut source_kind = None;
            let mut schema = None;
            for pair in node.remaining.chunks(2) {
                match pair {
                    [flag, value] if flag == "--as" && address.is_none() => {
                        address = Some(path(value)?)
                    }
                    [flag, value]
                        if flag == "--kind"
                            && source_kind.is_none()
                            && matches!(value.as_str(), "document" | "description") =>
                    {
                        source_kind = Some(value.as_str())
                    }
                    [flag, value] if flag == "--schema" && schema.is_none() => {
                        schema = Some(PathBuf::from(value))
                    }
                    _ => {
                        return Err(error(
                            "resource add accepts --as <RESOURCE_PATH>, --kind <document|description> and --schema <SCHEMA_FILE>",
                        ));
                    }
                }
            }
            let source = if source_kind == Some("description") {
                ResourceSource::Description {
                    file: source.into(),
                    schema,
                }
            } else {
                if schema.is_some() {
                    return Err(error("--schema requires --kind description"));
                }
                ResourceSource::Document(source.into())
            };
            Mutation::RegisterResource(RegisterResource {
                source,
                address,
                edit: node.edit,
            })
        }
        ["group", "add", address, options @ ..] => {
            let node = annotation_args::parse(&strings(options))?;
            if !node.remaining.is_empty() {
                return Err(error(
                    "unsupported group annotation option; see group add --help",
                ));
            }
            Mutation::CreateGroup {
                path: path(address)?,
                edit: node.edit,
            }
        }
        ["resource" | "group", "update", address, options @ ..] => {
            if options.is_empty() {
                return Err(error(
                    "update requires annotation or inheritance options; see --help",
                ));
            }
            let node = annotation_args::parse(&strings(options))?;
            if !node.remaining.is_empty() {
                return Err(error(
                    "update accepts only context annotation and inheritance options, not source/identity changes",
                ));
            }
            Mutation::UpdateNode {
                kind: kind(args[0]),
                path: path(address)?,
                edit: node.edit,
            }
        }
        ["resource", "remove", address] => Mutation::RemoveResource(path(address)?),
        ["group", "remove", address] => Mutation::RemoveGroup(path(address)?),
        ["resource" | "group", "move", from, to] => Mutation::MoveNode {
            kind: kind(args[0]),
            from: path(from)?,
            to: path(to)?,
        },
        ["config", "set", "language", value] => Mutation::SetLanguage(language(value)?),
        ["config", "set", key, value] => {
            let setting = match *key {
                "defaults.inherit" => DefaultSetting::Inherit,
                "defaults.propagate" => DefaultSetting::Propagate,
                _ => {
                    return Err(error(
                        "config set accepts language, defaults.inherit or defaults.propagate",
                    ));
                }
            };
            let value = match *value {
                "true" => true,
                "false" => false,
                _ => return Err(error("inheritance default must be true or false")),
            };
            Mutation::SetDefault { setting, value }
        }
        ["entry", "attach", file] => Mutation::RegisterEntry {
            kind: EntryKind::Instructions,
            file: file.into(),
        },
        ["entry", "create", file] => Mutation::RegisterEntry {
            kind: EntryKind::File,
            file: file.into(),
        },
        ["entry", "remove", file] => Mutation::RemoveEntry(file.into()),
        ["output", directory] => Mutation::SetOutput(directory.into()),
        ["access", "add", resource, tool] => Mutation::Associate {
            resource: path(resource)?,
            tool: path(tool)?,
        },
        ["access", "remove", resource, tool] => Mutation::Dissociate {
            resource: path(resource)?,
            tool: path(tool)?,
        },
        ["sync"] => Mutation::Publish,
        ["recover"] => Mutation::Recover,
        _ => return Err(error("unsupported command or arguments; see --help")),
    };
    Ok(Operation::Mutate(request))
}
fn strings(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| (*s).into()).collect()
}
fn language(value: &str) -> Result<OutputLanguage> {
    match value {
        "en" => Ok(OutputLanguage::English),
        "zh-CN" => Ok(OutputLanguage::SimplifiedChinese),
        _ => Err(error(format!(
            "unsupported output language '{value}'; expected en or zh-CN"
        ))),
    }
}
