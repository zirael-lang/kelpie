mod finder;

use id_arena::Id;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub use finder::*;

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

#[derive(Clone, Debug)]
pub enum ProjectKind {
    Workspace(Workspace),
    Package(Package),
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
    id: PackageId,
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
