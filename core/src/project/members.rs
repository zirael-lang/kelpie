use anyhow::Result;
use anyhow::bail;
use std::env::current_dir;
use std::path::PathBuf;

pub fn find_workspace_members(members: Vec<String>) -> Result<Vec<PathBuf>> {
    let dir = current_dir()?;
    let mut paths = vec![];

    for member in members {
        if member.contains('*') {
            let pattern = if member.starts_with("./") {
                dir.join(&member[2..]).to_string_lossy().into_owned()
            } else {
                dir.join(&member).to_string_lossy().into_owned()
            };

            match glob::Pattern::new(&pattern) {
                Ok(_) => {
                    let glob_result = glob::glob(&pattern)?;
                    for entry in glob_result {
                        match entry {
                            Ok(path) => {
                                if path.is_dir() {
                                    paths.push(path);
                                }
                            }
                            Err(e) => bail!("failed to process glob entry: {}", e),
                        }
                    }
                }
                Err(e) => bail!("invalid glob pattern '{}': {}", member, e),
            }
        } else {
            paths.push(dir.join(&member));
        }
    }

    paths.sort();

    if paths.is_empty() {
        bail!("no matching paths found for workspace members");
    }

    Ok(paths)
}
