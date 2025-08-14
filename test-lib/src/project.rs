use anyhow::Result;
use std::collections::HashMap;
use std::env::set_current_dir;
use std::fs;
use std::path::Path;
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

    pub fn run(&self) -> Result<()> {
        let exe_path = Path::new("..").join("target").join("debug").join("cli.exe");
        let exe_path = fs::canonicalize(exe_path)?;

        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        let temp_name = format!("kelpie-test-{}", counter);
        let temp = TempDir::new(&temp_name)?;
        let temp_path = temp.path();

        let original_dir = std::env::current_dir()?;
        set_current_dir(temp_path)?;

        for (path, content) in &self.files {
            if let Some(parent) = Path::new(path).parent() {
                fs::create_dir_all(temp_path.join(parent))?;
            }
            fs::write(temp_path.join(path), content)?;
        }

        let mut cmd = Command::new(exe_path);
        cmd.arg(self.command.as_ref().unwrap_or(&"run".to_string()))
            .args(&self.args)
            .arg("--test-logger") // simplifies logger by removing modules and colors
            .current_dir(temp_path)
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let output = cmd.output()?;

        set_current_dir(original_dir)?;
        let actual_output = String::from_utf8_lossy(&output.stderr);

        println!("{}", actual_output);

        if let Some(expected_output) = &self.expected_output {
            let actual_output = actual_output.trim();
            let expected_output = expected_output.trim();

            if actual_output != expected_output {
                return Err(anyhow::anyhow!(
                    "Output mismatch!\nExpected:\n{}\nActual:\n{}",
                    expected_output,
                    actual_output
                ));
            }
        }

        Ok(())
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(".");
    }
}
