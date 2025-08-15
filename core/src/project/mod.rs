mod dependency_resolver;
mod finder;
mod members;
mod pretty;

use id_arena::Id;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub use finder::*;
pub use pretty::*;

pub const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlConfig {
    pub workspace: Option<TomlWorkspace>,
    pub package: Option<TomlPackage>,
    pub dependencies: Option<TomlDependencies>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlWorkspace {
    pub members: Option<Vec<String>>,
    pub dependencies: Option<TomlDependencies>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlPackage {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub ty: Option<PackageType>,
    pub entrypoint: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PackageType {
    Library,
    Binary,
}

pub type TomlDependencies = HashMap<String, TomlDependency>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TomlDependency {
    Version(String),
    Detailed(DetailedDependency),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedDependency {
    pub version: Option<String>,
    pub path: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Project {
    pub kind: ProjectKind,
    pub dependencies: Vec<Dependency>,
}

impl Project {
    pub fn new(kind: ProjectKind, dependencies: Vec<Dependency>) -> Self {
        Self { kind, dependencies }
    }
}

#[derive(Clone, Debug)]
pub enum ProjectKind {
    Workspace(Workspace),
    Package(PackageId),
}

#[derive(Clone, Debug)]
pub struct Workspace {
    pub members: Vec<PackageId>,
}

#[derive(Clone, Debug)]
pub struct Dependency {
    pub id: PackageId,
    pub version: String,
}

pub type PackageId = Id<Package>;

#[derive(Clone, Debug)]
pub struct Package {
    pub(crate) id: PackageId,
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub ty: PackageType,
    pub entrypoint: PathBuf,
}
#[derive(Clone, Debug)]
pub struct PackageBuilder {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub ty: PackageType,
    pub entrypoint: PathBuf,
}

impl PackageBuilder {
    pub fn from_toml(toml_package: TomlPackage) -> Self {
        Self {
            name: toml_package.name,
            version: toml_package.version,
            author: toml_package.author,
            description: toml_package.description,
            license: toml_package.license,
            repository: toml_package.repository,
            homepage: toml_package.homepage,
            keywords: toml_package.keywords,
            ty: toml_package.ty.clone().unwrap_or(PackageType::Library),
            entrypoint: toml_package.entrypoint.unwrap_or_else(|| {
                match toml_package.ty.unwrap_or(PackageType::Library) {
                    PackageType::Library => PathBuf::from("src/lib.rs"),
                    PackageType::Binary => PathBuf::from("src/main.rs"),
                }
            }),
        }
    }

    pub fn build(self, id: PackageId) -> Package {
        Package {
            id,
            name: self.name,
            version: self.version,
            author: self.author,
            description: self.description,
            license: self.license,
            repository: self.repository,
            homepage: self.homepage,
            keywords: self.keywords,
            ty: self.ty,
            entrypoint: self.entrypoint,
        }
    }
}
