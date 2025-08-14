use crate::config::{CONFIG_FILE, TomlConfig};
use anyhow::{Result, bail};
use std::path::Path;

pub fn find_config<P: AsRef<Path>>(start_path: P) -> Result<TomlConfig> {
    let mut current = start_path.as_ref().canonicalize()?;

    loop {
        let manifest_path = current.join(CONFIG_FILE);
        if manifest_path.exists() {
            println!("Found config file: {:?}", manifest_path);
            return load_from_manifest(manifest_path);
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

pub fn load_from_manifest<P: AsRef<Path>>(manifest_path: P) -> Result<TomlConfig> {
    let file = fs_err::read_to_string(manifest_path)?;

    // initial config that will be filled with values
    let toml: TomlConfig = toml::from_str(&file)?;

    println!("{:?}", toml);

    if toml.workspace.is_some() && toml.package.is_some() {
        bail!("cannot have both workspace and package in one config file");
    }

    if toml.workspace.as_ref().map(|w| &w.dependencies).is_some() && toml.dependencies.is_some() {
        bail!("when in workspace mode, dependencies should be defined in the workspace config file")
    }

    Ok(toml)
}
