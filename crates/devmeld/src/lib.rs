//! Thin application coordination for local context management.
mod annotation_args;
mod context;
mod declarations;
mod inspection;
mod instructions;
mod language;
mod render;
mod storage;

pub use inspection::inspect_in;
pub use storage::Plan;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn error(message: impl Into<String>) -> Box<dyn std::error::Error> {
    std::io::Error::other(message.into()).into()
}

/// Select a context for a human invocation without creating directories.
pub fn prepare_in(
    cwd: &std::path::Path,
    explicit: Option<&std::path::Path>,
    args: &[String],
) -> Result<Plan> {
    let root = context::select(cwd, explicit)?;
    if explicit.is_none()
        && matches!(args, [command, ..] if command == "init")
        && context::marker_exists(&root)?
    {
        return Err(error(
            "context marker already exists; refusing implicit reinitialization. After recovery, explicitly select --context PATH to retry init",
        ));
    }
    let args = context::operands(cwd, &root, args)?;
    prepare(&root, &args)
}

pub fn prepare(root: &std::path::Path, args: &[String]) -> Result<Plan> {
    storage::local_path(root)?;
    let root = storage::resolve(
        &std::env::current_dir()?,
        root.to_str()
            .ok_or_else(|| error("non-Unicode context path"))?,
    )?;
    let creating = matches!(args, [command, ..] if command == "init")
        || matches!(args, [command, action, _, ..] if matches!(command.as_str(), "resource" | "group") && action == "add");
    if root.try_exists()? {
        if !root.is_dir() {
            return Err(error("context root must be a directory"));
        }
    } else if !creating {
        return Err(error("context not initialized; add a resource or run init"));
    }
    if args == ["recover"] {
        let mut plan = Plan::recovery(root)?;
        // A pending v0 journal is independently recoverable, including interrupted init.
        if plan.is_empty() {
            if plan
                .capture(&plan.root().join(".devmeld/context.json"))?
                .is_some()
            {
                declarations::read_config(&mut plan)?;
            } else if plan.owned_paths().next().is_some() {
                return Err(error(
                    "missing configuration; ownership records left intact",
                ));
            }
        }
        return Ok(plan);
    }
    let existing = context::marker_exists(&root)?;
    let mut plan = Plan::new(root)?;
    if creating && !existing {
        plan.require_fresh_context();
    }
    match args {
        [command, options @ ..] if command == "init" => {
            let path = plan.root().join(".devmeld/context.json");
            if plan.capture(&path)?.is_some() {
                return Err(error("context already initialized; refusing adoption"));
            }
            if plan.owned_paths().next().is_some() {
                return Err(error(
                    "existing ownership records; refusing reinitialization",
                ));
            }
            let mut config = declarations::Config::default();
            let mut language_seen = false;
            for option in options.chunks(2) {
                match option {
                    [flag, value] if flag == "--entry" => {
                        config.publication.entries.push(declarations::Entry {
                            kind: "file".into(),
                            path: value.clone(),
                        })
                    }
                    [flag, value] if flag == "--instruction-entry" => {
                        config.publication.entries.push(declarations::Entry {
                            kind: "instructions".into(),
                            path: value.clone(),
                        });
                    }
                    [flag, value] if flag == "--output" => {
                        config.publication.directory = value.clone()
                    }
                    [flag, value] if flag == "--language" && !language_seen => {
                        config.publication.language = language::OutputLanguage::parse(value)?;
                        language_seen = true;
                    }
                    _ => {
                        return Err(error(
                            "init accepts --entry PATH, --instruction-entry PATH, --output PATH and one --language en|zh-CN",
                        ));
                    }
                }
            }
            declarations::validate_surfaces(&plan, &config)?;
            plan.set(path, Some(declarations::encode(&config)?))?;
        }
        [command, action, source, options @ ..] if command == "resource" && action == "add" => {
            let mut config = if existing {
                declarations::read_config(&mut plan)?
            } else {
                declarations::Config::default()
            };
            let mut address = None;
            let mut kind = None;
            let mut schema = None;
            let node = annotation_args::parse(options)?;
            for pair in node.remaining.chunks(2) {
                match pair {
                    [flag, value] if flag == "--as" && address.is_none() => {
                        address = Some(value.clone());
                    }
                    [flag, value]
                        if flag == "--kind"
                            && kind.is_none()
                            && matches!(value.as_str(), "document" | "description") =>
                    {
                        kind = Some(value.as_str());
                    }
                    [flag, value] if flag == "--schema" && schema.is_none() => {
                        schema = Some(value.clone());
                    }
                    _ => {
                        return Err(error(
                            "resource add SOURCE accepts --as PATH, --kind document|description and --schema PATH",
                        ));
                    }
                }
            }
            let address = match address {
                Some(address) => address,
                None => std::path::Path::new(source)
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .ok_or_else(|| error("cannot derive a resource address; use --as PATH"))?
                    .to_owned(),
            };
            let address = devmeld_resources::organization::OrganizationPath::new(address)?;
            let mut organization = config.organization()?;
            let id = config.allocate_identity()?;
            organization.register(
                devmeld_resources::ResourceId::new(id.clone())?,
                address.clone(),
            )?;
            node.apply_choices(&mut organization, &address)?;
            organization.set_annotations(&address, node.annotations.for_creation()?)?;
            let description = kind == Some("description");
            if schema.is_some() && !description {
                return Err(error("--schema requires --kind description"));
            }
            let registration = declarations::Registration {
                id,
                path: Some(address.as_str().to_owned()),
                document: (!description).then(|| source.clone()),
                description: description.then(|| source.clone()),
                attributes_schema: schema,
                annotations: Default::default(),
                inherit: None,
            };
            config.resources.push(registration);
            config.update_organization(&organization)?;
            declarations::load_resources(&mut plan, &config)?;
            config.resources.sort_by(|a, b| a.id.cmp(&b.id));
            plan.set(
                plan.root().join(".devmeld/context.json"),
                Some(declarations::encode(&config)?),
            )?;
        }
        [command, action, address, options @ ..]
            if command == "group"
                && (action == "add" || (action == "remove" && options.is_empty())) =>
        {
            let mut config = if !existing && action == "add" {
                declarations::Config::default()
            } else {
                declarations::read_config(&mut plan)?
            };
            let mut organization = config.organization()?;
            let address = devmeld_resources::organization::OrganizationPath::new(address.clone())?;
            if action == "add" {
                let node = annotation_args::parse(options)?;
                if !node.remaining.is_empty() {
                    return Err(error(
                        "unsupported group annotation option; see group add --help",
                    ));
                }
                organization.add_group(address.clone())?;
                node.apply_choices(&mut organization, &address)?;
                organization.set_annotations(&address, node.annotations.for_creation()?)?;
            } else {
                organization.remove_group(&address)?;
            }
            config.update_organization(&organization)?;
            plan.set(
                plan.root().join(".devmeld/context.json"),
                Some(declarations::encode(&config)?),
            )?;
        }
        [command, action, address, options @ ..]
            if matches!(command.as_str(), "resource" | "group") && action == "update" =>
        {
            if options.is_empty() {
                return Err(error(
                    "update requires annotation or inheritance options; see --help",
                ));
            }
            let node = annotation_args::parse(options)?;
            if !node.remaining.is_empty() {
                return Err(error(
                    "update accepts only context annotation and inheritance options, not source/identity changes",
                ));
            }
            let mut config = declarations::read_config(&mut plan)?;
            let mut organization = config.organization()?;
            let address = devmeld_resources::organization::OrganizationPath::new(address.clone())?;
            if command == "resource" {
                config.resource_id(address.as_str())?;
            } else if !organization.groups().any(|path| path == &address) {
                return Err(error("group not registered"));
            }
            let original = organization.clone();
            let annotations = organization
                .annotations(&address)
                .ok_or_else(|| error("organization node not registered"))?;
            let changed = node.annotations.apply(annotations)?;
            organization.set_annotations(&address, changed)?;
            node.apply_choices(&mut organization, &address)?;
            if organization != original {
                config.update_organization(&organization)?;
                plan.set(
                    plan.root().join(".devmeld/context.json"),
                    Some(declarations::encode(&config)?),
                )?;
            }
        }
        [command, action, key, value] if command == "config" && action == "set" => {
            let mut config = declarations::read_config(&mut plan)?;
            let changed = if key == "language" {
                let language = language::OutputLanguage::parse(value)?;
                let changed = config.publication.language != language;
                config.publication.language = language;
                changed
            } else {
                let value = match value.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => return Err(error("inheritance default must be true or false")),
                };
                let selected = match key.as_str() {
                    "defaults.inherit" => &mut config.defaults.inherit,
                    "defaults.propagate" => &mut config.defaults.propagate,
                    _ => {
                        return Err(error(
                            "config set accepts language, defaults.inherit or defaults.propagate",
                        ));
                    }
                };
                let changed = *selected != value;
                *selected = value;
                changed
            };
            if changed {
                plan.set(
                    plan.root().join(".devmeld/context.json"),
                    Some(declarations::encode(&config)?),
                )?;
            }
        }
        [command, action, from, to]
            if matches!(command.as_str(), "resource" | "group") && action == "move" =>
        {
            let mut config = declarations::read_config(&mut plan)?;
            let mut organization = config.organization()?;
            let from = devmeld_resources::organization::OrganizationPath::new(from.clone())?;
            let to = devmeld_resources::organization::OrganizationPath::new(to.clone())?;
            if command == "resource" {
                organization.move_resource(&from, to.clone())?;
            } else {
                organization.move_group(&from, to.clone())?;
            }
            if from != to {
                config.update_organization(&organization)?;
                plan.set(
                    plan.root().join(".devmeld/context.json"),
                    Some(declarations::encode(&config)?),
                )?;
            }
        }
        [command] if command == "sync" => {
            let config = declarations::read_config(&mut plan)?;
            prepare_publication(&mut plan, &config)?;
        }
        [command, action, address] if command == "resource" && action == "remove" => {
            let mut config = declarations::read_config(&mut plan)?;
            let id = config.resource_id(address)?;
            declarations::membership(&config)?.unregister(&id)?;
            config.resources.retain(|r| r.id != id.as_str());
            plan.set(
                plan.root().join(".devmeld/context.json"),
                Some(declarations::encode(&config)?),
            )?;
        }
        [command, action, path] if command == "entry" => {
            let mut config = declarations::read_config(&mut plan)?;
            let selected = storage::resolve(plan.root(), path)?;
            if action == "create" || action == "attach" {
                let kind = if action == "attach" {
                    "instructions"
                } else {
                    "file"
                };
                config.publication.entries.push(declarations::Entry {
                    kind: kind.into(),
                    path: path.clone(),
                });
            } else if action == "remove" {
                let mut found = false;
                let mut retained = Vec::new();
                for entry in config.publication.entries {
                    if storage::resolve(plan.root(), &entry.path)? == selected {
                        found = true;
                    } else {
                        retained.push(entry);
                    }
                }
                if !found {
                    return Err(error("entry not registered"));
                }
                config.publication.entries = retained;
            } else {
                return Err(error("entry expects attach, create or remove"));
            }
            declarations::validate_surfaces(&plan, &config)?;
            config
                .publication
                .entries
                .sort_by(|a, b| a.path.cmp(&b.path));
            plan.set(
                plan.root().join(".devmeld/context.json"),
                Some(declarations::encode(&config)?),
            )?;
        }
        [command, path] if command == "output" => {
            let mut config = declarations::read_config(&mut plan)?;
            config.publication.directory = path.clone();
            declarations::validate_surfaces(&plan, &config)?;
            plan.set(
                plan.root().join(".devmeld/context.json"),
                Some(declarations::encode(&config)?),
            )?;
        }
        [command, action, resource, tool] if command == "access" => {
            let mut config = declarations::read_config(&mut plan)?;
            let mut membership = declarations::membership(&config)?;
            let resource_id = config.resource_id(resource)?;
            let tool_id = config.resource_id(tool)?;
            match action.as_str() {
                "add" => {
                    membership.associate(resource_id.clone(), tool_id.clone())?;
                    config.access.push(declarations::Access {
                        resource: resource_id.as_str().into(),
                        tool: tool_id.as_str().into(),
                    });
                }
                "remove" => {
                    membership.dissociate(resource_id.clone(), tool_id.clone())?;
                    config.access.retain(|a| {
                        a.resource != resource_id.as_str() || a.tool != tool_id.as_str()
                    });
                }
                _ => return Err(error("access expects add or remove")),
            }
            config
                .access
                .sort_by(|a, b| (&a.resource, &a.tool).cmp(&(&b.resource, &b.tool)));
            plan.set(
                plan.root().join(".devmeld/context.json"),
                Some(declarations::encode(&config)?),
            )?;
        }
        _ => return Err(error("unsupported command or arguments; see --help")),
    }
    Ok(plan)
}

/// Build the same bounded publication comparison for sync and read-only status.
pub(crate) fn prepare_publication(plan: &mut Plan, config: &declarations::Config) -> Result<()> {
    let resources = declarations::load_resources(plan, config)?;
    let files = render::publication(plan, config, &resources)?;
    let mut instructions = std::collections::BTreeMap::new();
    for entry in config
        .publication
        .entries
        .iter()
        .filter(|e| e.kind == "instructions")
    {
        let path = storage::resolve(plan.root(), &entry.path)?;
        instructions.insert(
            path.clone(),
            render::instruction_entry(plan.root(), config, &path)?,
        );
    }
    let obsolete: Vec<_> = plan
        .owned_paths()
        .filter(|path| {
            path.as_path() != plan.root().join(".devmeld/context.json")
                && !files.contains_key(*path)
                && !instructions.contains_key(*path)
        })
        .cloned()
        .collect();
    plan.validate_targets(files.keys().chain(instructions.keys()).cloned())?;
    for (path, bytes) in files {
        plan.set(path, Some(bytes))?;
    }
    for (path, body) in instructions {
        plan.set_entry(path, Some(&body))?;
    }
    for path in obsolete {
        plan.withdraw(path)?;
    }
    Ok(())
}
