use anyhow::Result;
use std::collections::HashMap;
use std::env::set_current_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use tempdir::TempDir;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct Project {
    files: HashMap<String, String>,
    args: Vec<String>,
    expected_output: Option<String>,
    command: Option<String>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            args: Vec::new(),
            expected_output: None,
            command: None,
        }
    }

    pub fn file(mut self, path: &str, content: &str) -> Self {
        self.files.insert(path.to_string(), content.to_string());
        self
    }

    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn expected_output(mut self, output: &str) -> Self {
        self.expected_output = Some(output.to_string());
        self
    }

    pub fn command(mut self, command: &str) -> Self {
        self.command = Some(command.to_string());
        self
    }

    pub fn expected_success(mut self) -> Self {
        self.expected_output = None;
        self
    }

    pub fn run(&self) -> Result<()> {
        let exe_name = if cfg!(windows) { "cli.exe" } else { "cli" };
        let exe_path = Path::new("../target").join("debug").join(exe_name);

        if !exe_path.exists() {
            let status = Command::new("cargo")
                .args(["build", "-p", "cli"])
                .status()?;

            if !status.success() {
                return Err(anyhow::anyhow!("Failed to build CLI executable"));
            }

            if !exe_path.exists() {
                return Err(anyhow::anyhow!(
                    "CLI executable not found at: {}",
                    exe_path.display()
                ));
            }
        }

        let exe_path = fs_err::canonicalize(&exe_path)?;

        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        let temp_name = format!("kelpie-test-{}", counter);
        let temp = TempDir::new(&temp_name)?;
        let temp_path = temp.path();

        let original_dir = std::env::current_dir()?;
        let _guard = Guard(original_dir.clone());

        for (path, content) in &self.files {
            let full_path = temp_path.join(path);
            if let Some(parent) = full_path.parent() {
                fs_err::create_dir_all(parent)?;
            }
            fs_err::write(full_path, content)?;
        }

        set_current_dir(temp_path)?;

        let mut cmd = Command::new(&exe_path);
        cmd.arg(self.command.as_ref().unwrap_or(&"run".to_string()))
            .args(&self.args)
            .arg("--test-logger")
            .current_dir(temp_path)
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let output = cmd.output()?;

        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{}", stderr);

        if let Some(expected_output) = &self.expected_output {
            let actual_output = stderr.trim();
            let expected_output = expected_output.trim();

            if actual_output != expected_output {
                return Err(anyhow::anyhow!(
                    "Output mismatch!\nExpected:\n{}\nActual:\n{}",
                    expected_output,
                    actual_output
                ));
            }
        } else if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Command failed with exit code {:?}\nOutput:\n{}",
                output.status.code(),
                stderr
            ));
        }

        Ok(())
    }
}

struct Guard(PathBuf);

impl Drop for Guard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.0);
    }
}
