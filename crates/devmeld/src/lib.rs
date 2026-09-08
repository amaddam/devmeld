//! Thin application coordination for explicitly selected local contexts.
mod declarations;
mod render;
mod storage;

pub use storage::Plan;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn error(message: impl Into<String>) -> Box<dyn std::error::Error> {
    std::io::Error::other(message.into()).into()
}

pub fn prepare(root: &std::path::Path, args: &[String]) -> Result<Plan> {
    let root = root.canonicalize()?;
    if !root.is_dir() {
        return Err(error("context root must be an existing directory"));
    }
    if args == ["recover"] {
        return Plan::recovery(root);
    }
    let mut plan = Plan::new(root)?;
    match args {
        [command, options @ ..] if command == "init" => {
            let path = plan.root().join(".devmeld/context.json");
            if plan.capture(&path)?.is_some() {
                return Err(error("context already initialized; refusing adoption"));
            }
            let mut config = declarations::Config::default();
            for option in options.chunks(2) {
                match option {
                    [flag, value] if flag == "--entry" => {
                        config.publication.entries.push(declarations::Entry {
                            kind: "file".into(),
                            path: value.clone(),
                        })
                    }
                    [flag, value] if flag == "--output" => {
                        config.publication.directory = value.clone()
                    }
                    _ => return Err(error("init accepts --entry PATH and --output PATH")),
                }
            }
            declarations::validate_surfaces(&plan, &config)?;
            plan.set(path, Some(declarations::encode(&config)?))?;
        }
        [command, action, id, options @ ..] if command == "resource" && action == "add" => {
            let mut config = declarations::read_config(&mut plan)?;
            let mut registration = declarations::Registration {
                id: id.clone(),
                document: None,
                description: None,
                attributes_schema: None,
            };
            for pair in options.chunks(2) {
                match pair {
                    [flag, value] if flag == "--document" && registration.document.is_none() => {
                        registration.document = Some(value.clone())
                    }
                    [flag, value]
                        if flag == "--description" && registration.description.is_none() =>
                    {
                        registration.description = Some(value.clone())
                    }
                    [flag, value]
                        if flag == "--schema" && registration.attributes_schema.is_none() =>
                    {
                        registration.attributes_schema = Some(value.clone())
                    }
                    _ => {
                        return Err(error(
                            "resource add expects --document PATH OR --description PATH [--schema PATH]",
                        ));
                    }
                }
            }
            config.resources.push(registration);
            declarations::load_resources(&mut plan, &config)?;
            config.resources.sort_by(|a, b| a.id.cmp(&b.id));
            plan.set(
                plan.root().join(".devmeld/context.json"),
                Some(declarations::encode(&config)?),
            )?;
        }
        [command] if command == "sync" => {
            let config = declarations::read_config(&mut plan)?;
            let resources = declarations::load_resources(&mut plan, &config)?;
            let files = render::publication(&mut plan, &config, &resources)?;
            let obsolete: Vec<_> = plan
                .owned_paths()
                .filter(|path| {
                    path.as_path() != plan.root().join(".devmeld/context.json")
                        && !files.contains_key(*path)
                })
                .cloned()
                .collect();
            for (path, bytes) in files {
                plan.set(path, Some(bytes))?;
            }
            for path in obsolete {
                plan.set(path, None)?;
            }
        }
        [command, action, id] if command == "resource" && action == "remove" => {
            let mut config = declarations::read_config(&mut plan)?;
            if config
                .access
                .iter()
                .any(|a| &a.resource == id || &a.tool == id)
            {
                return Err(error(
                    "remove access associations explicitly before removing this resource",
                ));
            }
            let count = config.resources.len();
            declarations::membership(&config)?
                .unregister(&devmeld_resources::ResourceId::new(id.clone())?)?;
            config.resources.retain(|r| &r.id != id);
            if config.resources.len() == count {
                return Err(error("resource not registered"));
            }
            plan.set(
                plan.root().join(".devmeld/context.json"),
                Some(declarations::encode(&config)?),
            )?;
        }
        [command, action, path] if command == "entry" => {
            let mut config = declarations::read_config(&mut plan)?;
            let selected = storage::resolve(plan.root(), path)?;
            if action == "add" {
                config.publication.entries.push(declarations::Entry {
                    kind: "file".into(),
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
                return Err(error("entry expects add or remove"));
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
            let resource_id = devmeld_resources::ResourceId::new(resource.clone())?;
            let tool_id = devmeld_resources::ResourceId::new(tool.clone())?;
            match action.as_str() {
                "add" => {
                    membership.associate(resource_id, tool_id)?;
                    config.access.push(declarations::Access {
                        resource: resource.clone(),
                        tool: tool.clone(),
                    });
                }
                "remove" => {
                    membership.dissociate(resource_id, tool_id)?;
                    config
                        .access
                        .retain(|a| &a.resource != resource || &a.tool != tool);
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
