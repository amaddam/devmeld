//! Typed use-case coordination; external syntax and presentation belong to adapters.
use crate::{Plan, Result, context, declarations, error, render, request::*, storage};
use devmeld_resources::organization::OrganizationPath;

pub fn prepare(location: &ContextLocation, request: Mutation) -> Result<Plan> {
    let root = context::select(&location.base_directory, location.directory.as_deref())?;
    if location.directory.is_none()
        && matches!(request, Mutation::Initialize(_))
        && context::marker_exists(&root)?
    {
        return Err(error(
            "context marker already exists; refusing implicit reinitialization. After recovery, explicitly select the context to retry initialization",
        ));
    }
    let creating = matches!(
        request,
        Mutation::Initialize(_) | Mutation::RegisterResource(_) | Mutation::CreateGroup { .. }
    );
    if root.try_exists()? {
        if !root.is_dir() {
            return Err(error("context root must be a directory"));
        }
    } else if !creating {
        return Err(error(
            "context not initialized; add a resource or initialize it",
        ));
    }
    if matches!(request, Mutation::Recover) {
        let mut plan = Plan::recovery(root)?;
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
    let reference =
        |path: &std::path::Path| context::reference(&location.base_directory, plan.root(), path);
    match request {
        Mutation::Initialize(options) => {
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
            if let Some(output) = options.output {
                config.publication.directory =
                    context::reference(&location.base_directory, plan.root(), &output)?;
            }
            if let Some(language) = options.language {
                config.publication.language = language;
            }
            for (kind, file) in options.entries {
                config.publication.entries.push(declarations::Entry {
                    kind: kind.stored().into(),
                    path: context::reference(&location.base_directory, plan.root(), &file)?,
                });
            }
            declarations::validate_surfaces(&plan, &config)?;
            plan.set(path, Some(declarations::encode(&config)?))?;
        }
        Mutation::RegisterResource(request) => {
            let mut config = if existing {
                declarations::read_config(&mut plan)?
            } else {
                declarations::Config::default()
            };
            let (source, schema, description) = match request.source {
                ResourceSource::Document(file) => (
                    context::reference(&location.base_directory, plan.root(), &file)?,
                    None,
                    false,
                ),
                ResourceSource::Description { file, schema } => (
                    context::reference(&location.base_directory, plan.root(), &file)?,
                    schema
                        .map(|file| {
                            context::reference(&location.base_directory, plan.root(), &file)
                        })
                        .transpose()?,
                    true,
                ),
            };
            let address = match request.address {
                Some(address) => address,
                None => OrganizationPath::new(std::path::Path::new(&source).file_stem().and_then(|name| name.to_str()).ok_or_else(|| error("cannot derive a resource address; provide an explicit logical address"))?)?,
            };
            let mut organization = config.organization()?;
            let id = config.allocate_identity()?;
            organization.register(
                devmeld_resources::ResourceId::new(id.clone())?,
                address.clone(),
            )?;
            request.edit.apply_choices(&mut organization, &address)?;
            let annotations = request
                .edit
                .annotations
                .map(|edit| edit.for_creation())
                .transpose()?
                .unwrap_or_default();
            organization.set_annotations(&address, annotations)?;
            config.resources.push(declarations::Registration {
                id,
                path: Some(address.as_str().to_owned()),
                document: (!description).then(|| source.clone()),
                description: description.then_some(source),
                attributes_schema: schema,
                annotations: Default::default(),
                inherit: None,
            });
            config.update_organization(&organization)?;
            declarations::load_resources(&mut plan, &config)?;
            config.resources.sort_by(|a, b| a.id.cmp(&b.id));
            save_config(&mut plan, &config)?;
        }
        Mutation::CreateGroup { path, edit } => {
            let mut config = if existing {
                declarations::read_config(&mut plan)?
            } else {
                declarations::Config::default()
            };
            let mut organization = config.organization()?;
            organization.add_group(path.clone())?;
            edit.apply_choices(&mut organization, &path)?;
            let annotations = edit
                .annotations
                .map(|edit| edit.for_creation())
                .transpose()?
                .unwrap_or_default();
            organization.set_annotations(&path, annotations)?;
            config.update_organization(&organization)?;
            save_config(&mut plan, &config)?;
        }
        Mutation::RemoveGroup(path) => {
            let mut config = declarations::read_config(&mut plan)?;
            let mut organization = config.organization()?;
            organization.remove_group(&path)?;
            config.update_organization(&organization)?;
            save_config(&mut plan, &config)?;
        }
        Mutation::UpdateNode { kind, path, edit } => {
            let mut config = declarations::read_config(&mut plan)?;
            let mut organization = config.organization()?;
            if kind == NodeKind::Resource {
                config.resource_id(path.as_str())?;
            } else if !organization.groups().any(|group| group == &path) {
                return Err(error("group not registered"));
            }
            let original = organization.clone();
            if let Some(annotations) = &edit.annotations {
                let current = organization
                    .annotations(&path)
                    .ok_or_else(|| error("organization node not registered"))?;
                organization.set_annotations(&path, annotations.apply(current)?)?;
            }
            edit.apply_choices(&mut organization, &path)?;
            if organization != original {
                config.update_organization(&organization)?;
                save_config(&mut plan, &config)?;
            }
        }
        Mutation::SetLanguage(language) => {
            let mut config = declarations::read_config(&mut plan)?;
            if config.publication.language != language {
                config.publication.language = language;
                save_config(&mut plan, &config)?;
            }
        }
        Mutation::SetDefault { setting, value } => {
            let mut config = declarations::read_config(&mut plan)?;
            let selected = match setting {
                DefaultSetting::Inherit => &mut config.defaults.inherit,
                DefaultSetting::Propagate => &mut config.defaults.propagate,
            };
            if *selected != value {
                *selected = value;
                save_config(&mut plan, &config)?;
            }
        }
        Mutation::MoveNode { kind, from, to } => {
            let mut config = declarations::read_config(&mut plan)?;
            let mut organization = config.organization()?;
            match kind {
                NodeKind::Resource => organization.move_resource(&from, to.clone())?,
                NodeKind::Group => organization.move_group(&from, to.clone())?,
            }
            if from != to {
                config.update_organization(&organization)?;
                save_config(&mut plan, &config)?;
            }
        }
        Mutation::Publish => {
            let config = declarations::read_config(&mut plan)?;
            prepare_publication(&mut plan, &config)?;
        }
        Mutation::RemoveResource(address) => {
            let mut config = declarations::read_config(&mut plan)?;
            let id = config.resource_id(address.as_str())?;
            declarations::membership(&config)?.unregister(&id)?;
            config.resources.retain(|r| r.id != id.as_str());
            save_config(&mut plan, &config)?;
        }
        Mutation::RegisterEntry { kind, file } => {
            let path = reference(&file)?;
            let mut config = declarations::read_config(&mut plan)?;
            config.publication.entries.push(declarations::Entry {
                kind: kind.stored().into(),
                path,
            });
            declarations::validate_surfaces(&plan, &config)?;
            config
                .publication
                .entries
                .sort_by(|a, b| a.path.cmp(&b.path));
            save_config(&mut plan, &config)?;
        }
        Mutation::RemoveEntry(file) => {
            let path = reference(&file)?;
            let mut config = declarations::read_config(&mut plan)?;
            let selected = storage::resolve(plan.root(), &path)?;
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
            declarations::validate_surfaces(&plan, &config)?;
            config
                .publication
                .entries
                .sort_by(|a, b| a.path.cmp(&b.path));
            save_config(&mut plan, &config)?;
        }
        Mutation::SetOutput(path) => {
            let path = reference(&path)?;
            let mut config = declarations::read_config(&mut plan)?;
            config.publication.directory = path;
            declarations::validate_surfaces(&plan, &config)?;
            save_config(&mut plan, &config)?;
        }
        Mutation::Associate { .. } | Mutation::Dissociate { .. } => {
            let add = matches!(request, Mutation::Associate { .. });
            let (resource, tool) = match request {
                Mutation::Associate { resource, tool }
                | Mutation::Dissociate { resource, tool } => (resource, tool),
                _ => unreachable!(),
            };
            let mut config = declarations::read_config(&mut plan)?;
            let mut membership = declarations::membership(&config)?;
            let resource_id = config.resource_id(resource.as_str())?;
            let tool_id = config.resource_id(tool.as_str())?;
            if add {
                membership.associate(resource_id.clone(), tool_id.clone())?;
                config.access.push(declarations::Access {
                    resource: resource_id.as_str().into(),
                    tool: tool_id.as_str().into(),
                });
            } else {
                membership.dissociate(resource_id.clone(), tool_id.clone())?;
                config
                    .access
                    .retain(|a| a.resource != resource_id.as_str() || a.tool != tool_id.as_str());
            }
            config
                .access
                .sort_by(|a, b| (&a.resource, &a.tool).cmp(&(&b.resource, &b.tool)));
            save_config(&mut plan, &config)?;
        }
        Mutation::Recover => unreachable!("recovery returned before ordinary preparation"),
    }
    Ok(plan)
}

fn save_config(plan: &mut Plan, config: &declarations::Config) -> Result<()> {
    plan.set(
        plan.root().join(".devmeld/context.json"),
        Some(declarations::encode(config)?),
    )
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
