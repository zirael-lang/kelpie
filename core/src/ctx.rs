use crate::{
    Dependency, Package, PackageBuilder, PackageId, Project, TomlDependencies, TomlDependency,
};
use anyhow::{Result, bail};
use id_arena::{Arena, Id};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub type ProjectId = Id<Project>;

#[derive(Debug)]
pub struct KelpieContext {
    pub projects: Arena<Project>,
    pub packages: Arena<Package>,
    pub path_to_project: HashMap<PathBuf, ProjectId>,
    pub name_to_package: HashMap<String, PackageId>,
}

impl KelpieContext {
    pub fn new() -> Self {
        Self {
            projects: Arena::new(),
            packages: Arena::new(),
            path_to_project: HashMap::new(),
            name_to_package: HashMap::new(),
        }
    }

    pub fn get_project(&self, id: ProjectId) -> Option<&Project> {
        self.projects.get(id)
    }

    pub fn get_project_mut(&mut self, id: ProjectId) -> Option<&mut Project> {
        self.projects.get_mut(id)
    }

    pub fn get_package(&self, id: PackageId) -> Option<&Package> {
        self.packages.get(id)
    }

    pub fn get_package_by_name(&self, name: &str) -> Option<&Package> {
        let package_id = self.name_to_package.get(name)?;
        self.packages.get(*package_id)
    }

    pub fn find_project_by_path<P: AsRef<Path>>(&self, path: P) -> Option<ProjectId> {
        let canonical_path = path.as_ref().canonicalize().ok()?;
        self.path_to_project.get(&canonical_path).copied()
    }

    pub fn add_project(&mut self, project: Project, manifest_path: PathBuf) -> ProjectId {
        let project_id = self.projects.alloc(project);
        self.path_to_project.insert(manifest_path, project_id);
        project_id
    }

    pub fn add_package(&mut self, package_builder: PackageBuilder) -> PackageId {
        let name = package_builder.name.clone();
        let package_id = self.packages.alloc_with_id(|id| package_builder.build(id));

        self.name_to_package.insert(name, package_id);

        package_id
    }
}

impl Default for KelpieContext {
    fn default() -> Self {
        Self::new()
    }
}
