use crate::{
    CONFIG_FILE, Dependency, KelpieContext, ProjectKind, TomlDependencies, TomlDependency,
};
use anyhow::{anyhow, bail};
use std::env::current_dir;
use std::path::PathBuf;
use zirael_core::prelude::{canonicalize_with_strip, debug};
use zirael_utils::prelude::PackageType;

impl KelpieContext {
    pub fn resolve_dependencies(
        &mut self,
        toml_deps: Option<&TomlDependencies>,
        base_path: PathBuf,
    ) -> anyhow::Result<Vec<Dependency>> {
        let mut dependencies = Vec::new();

        if let Some(deps) = toml_deps {
            for (name, dep) in deps {
                let dep = match dep {
                    TomlDependency::Version(version) => {
                        if let Some(&package_id) = self.name_to_package.get(name) {
                            Some(Dependency {
                                id: package_id,
                                version: version.clone(),
                            })
                        } else {
                            None
                        }
                    }
                    TomlDependency::Detailed(dep) => {
                        let Some(version) = &dep.version else {
                            bail!("missing version for dependency: {}", name);
                        };

                        // resolving by path
                        if let Some(path) = &dep.path {
                            let config_path = base_path.join(path).join(CONFIG_FILE);
                            let canonicalized =
                                canonicalize_with_strip(&config_path).map_err(|_| {
                                    anyhow!("couldn't resolve path dependency: {}", name)
                                })?;
                            debug!(
                                "resolved dependency {} to {}",
                                name,
                                canonicalized.display()
                            );
                            let project = self.load_from_manifest(config_path, false)?;
                            let project = self.get_project(project).unwrap();
                            let ProjectKind::Package(id) = project.kind else {
                                bail!("can't import package {} from workspace", name);
                            };

                            let pkg = self.get_package(id).unwrap();
                            if pkg.ty != PackageType::Library {
                                bail!("can't import package {} which is a binary", name);
                            }

                            Some(Dependency {
                                id,
                                version: version.clone(),
                            })
                        } else {
                            None
                        }
                    }
                };

                let Some(dep) = dep else {
                    bail!("couldn't resolve dependency: {}", name);
                };
                dependencies.push(dep);
            }
        }

        Ok(dependencies)
    }
}
