use crate::ctx::{KelpieContext, ProjectId};
use crate::project::members::find_workspace_members;
use crate::project::{CONFIG_FILE, TomlConfig};
use crate::{Package, PackageBuilder, Project, ProjectKind, Workspace};
use anyhow::{Result, bail};
use std::path::Path;
use zirael_core::prelude::canonicalize_with_strip;

pub fn find_config<P: AsRef<Path>>(start_path: P, ctx: &mut KelpieContext) -> Result<ProjectId> {
    let mut current = canonicalize_with_strip(start_path.as_ref())?;

    loop {
        let manifest_path = current.join(CONFIG_FILE);
        if manifest_path.exists() {
            return ctx.load_from_manifest(manifest_path, false);
        }

        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => bail!(
                "No {} found in current directory or any parent directory",
                CONFIG_FILE
            ),
        }
    }
}

impl KelpieContext {
    pub fn load_from_manifest<P: AsRef<Path>>(
        &mut self,
        manifest_path: P,
        is_workspace_member: bool,
    ) -> Result<ProjectId> {
        let manifest_path = canonicalize_with_strip(manifest_path.as_ref())?;

        if let Some(existing_id) = self.find_project_by_path(&manifest_path) {
            return Ok(existing_id);
        }

        let file = fs_err::read_to_string(&manifest_path)?;
        let toml: TomlConfig = toml::from_str(&file)?;

        if is_workspace_member && toml.workspace.is_some() {
            bail!("cannot have workspace in a workspace member config file");
        }

        if toml.workspace.is_some() && toml.package.is_some() {
            bail!("cannot have both workspace and package in one config file");
        }

        if toml
            .workspace
            .as_ref()
            .and_then(|w| w.dependencies.as_ref())
            .is_some()
            && toml.dependencies.is_some()
        {
            bail!(
                "when in workspace mode, dependencies should be defined in the workspace config file"
            );
        }

        let project_id = if let Some(workspace_config) = toml.workspace {
            let member_package_ids = if let Some(members) = workspace_config.members {
                let member_paths = find_workspace_members(members)?;

                let mut member_ids = Vec::new();

                for member_path in member_paths {
                    let config_path = member_path.join(CONFIG_FILE);
                    if !config_path.exists() {
                        bail!(
                            "no config file found in workspace member: {}",
                            member_path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("<unknown>")
                        );
                    }

                    let member_project_id = self.load_from_manifest(&config_path, true)?;

                    if let Some(member_project) = self.get_project(member_project_id) {
                        if let ProjectKind::Package(package) = &member_project.kind {
                            member_ids.push(*package);
                        }
                    }
                }
                member_ids
            } else {
                Vec::new()
            };

            let dependencies = self.resolve_dependencies(
                workspace_config.dependencies.as_ref(),
                manifest_path.parent().unwrap().to_path_buf(),
            )?;
            let workspace = Workspace {
                members: member_package_ids,
            };

            let workspace_project = Project::new(ProjectKind::Workspace(workspace), dependencies);

            self.add_project(workspace_project, manifest_path)
        } else if let Some(package_config) = toml.package {
            let package_builder = PackageBuilder::from_toml(
                package_config,
                manifest_path.parent().unwrap().to_path_buf(),
            );

            if !package_builder.full_entrypoint_path().exists() {
                bail!(
                    "entrypoint of package {} doesn't exist",
                    package_builder.name
                )
            }

            let package_id = self.add_package(package_builder);

            let dependencies = self.resolve_dependencies(
                toml.dependencies.as_ref(),
                manifest_path.parent().unwrap().to_path_buf(),
            )?;

            let package_project = Project::new(ProjectKind::Package(package_id), dependencies);

            self.add_project(package_project, manifest_path)
        } else {
            bail!("no workspace or package defined in config file")
        };

        Ok(project_id)
    }
}
