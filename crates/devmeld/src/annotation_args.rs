//! Parse explicit descriptive annotations, independently of native path operands.
use crate::{Result, error};
use devmeld_resources::organization::{
    AnnotationEdit, LocalAnnotations, Organization, OrganizationPath,
};
use std::collections::BTreeSet;

pub(crate) fn takes_no_value(flag: &str) -> bool {
    matches!(
        flag,
        "--shared"
            | "--no-shared"
            | "--clear-description"
            | "--inherit"
            | "--no-inherit"
            | "--propagate"
            | "--no-propagate"
    )
}

pub(crate) struct NodeOptions {
    pub annotations: AnnotationEdit,
    pub remaining: Vec<String>,
    inherit: Option<bool>,
    propagate: Option<bool>,
}
impl NodeOptions {
    pub fn apply_choices(
        &self,
        organization: &mut Organization,
        path: &OrganizationPath,
    ) -> Result<()> {
        if let Some(value) = self.inherit {
            organization.set_inherit(path, value)?;
        }
        if let Some(value) = self.propagate {
            organization.set_propagate(path, value)?;
        }
        Ok(())
    }
}

pub(crate) fn parse(options: &[String]) -> Result<NodeOptions> {
    let mut description = None;
    let mut clear_description = false;
    let mut tags = Vec::new();
    let mut fields = Vec::new();
    let mut remove_tags = BTreeSet::new();
    let mut remove_fields = BTreeSet::new();
    let mut other = Vec::new();
    let mut inherit = None;
    let mut propagate = None;
    let mut i = 0;
    while i < options.len() {
        let flag = options[i].as_str();
        if takes_no_value(flag) {
            if matches!(
                flag,
                "--inherit" | "--no-inherit" | "--propagate" | "--no-propagate"
            ) {
                let (choice, value) = if matches!(flag, "--inherit" | "--no-inherit") {
                    (&mut inherit, flag == "--inherit")
                } else {
                    (&mut propagate, flag == "--propagate")
                };
                if choice.replace(value).is_some() {
                    return Err(error(format!(
                        "duplicate or conflicting inheritance switch: {flag}"
                    )));
                }
            } else if flag == "--clear-description" {
                if clear_description {
                    return Err(error("duplicate --clear-description"));
                }
                clear_description = true;
            } else {
                fields.push(("shared".into(), (flag == "--shared").to_string()));
            }
            i += 1;
            continue;
        }
        let value = options
            .get(i + 1)
            .ok_or_else(|| error(format!("missing value for {flag}")))?;
        match flag {
            "--description" if description.is_none() => description = Some(value.clone()),
            "--description" => return Err(error("duplicate --description")),
            "--tag" => tags.push(value.clone()),
            "--remove-tag" => {
                remove_tags.insert(value.clone());
            }
            "--remove-field" => {
                remove_fields.insert(value.clone());
            }
            "--environment" | "--attention" => fields.push((flag[2..].into(), value.clone())),
            "--field" => {
                let (key, value) = value
                    .split_once('=')
                    .ok_or_else(|| error("--field requires KEY=VALUE"))?;
                fields.push((key.into(), value.into()));
            }
            _ => other.extend_from_slice(&options[i..i + 2]),
        }
        i += 2;
    }
    let values = LocalAnnotations::new(description, tags, fields)?;
    Ok(NodeOptions {
        annotations: AnnotationEdit::new(
            values,
            clear_description,
            remove_tags.into_iter().collect(),
            remove_fields.into_iter().collect(),
        )?,
        remaining: other,
        inherit,
        propagate,
    })
}
