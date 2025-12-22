use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct UnidocCommand {
    standalone: bool,
    includes_header: Vec<PathBuf>,
    includes_before: Vec<PathBuf>,
    includes_after: Vec<PathBuf>,
    output: Option<PathBuf>,
    variables: Vec<(String, String)>,
}

impl UnidocCommand {
    pub fn new() -> Self {
        Self {
            standalone: false,
            includes_header: Vec::new(),
            includes_before: Vec::new(),
            includes_after: Vec::new(),
            output: None,
            variables: Vec::new(),
        }
    }

    pub fn standalone(mut self) -> Self {
        self.standalone = true;
        self
    }

    pub fn include_in_header(mut self, path: PathBuf) -> Self {
        self.includes_header.push(path);
        self
    }

    pub fn include_before_body(mut self, path: PathBuf) -> Self {
        self.includes_before.push(path);
        self
    }

    pub fn include_after_body(mut self, path: PathBuf) -> Self {
        self.includes_after.push(path);
        self
    }

    pub fn output(mut self, path: PathBuf) -> Self {
        self.output = Some(path);
        self
    }

    #[allow(dead_code)]
    pub fn variable(mut self, key: String, value: String) -> Self {
        self.variables.push((key, value));
        self
    }

    pub fn execute(&self, input: &Path) -> Result<()> {
        let mut cmd = Command::new("unidoc");

        if self.standalone {
            cmd.arg("-s");
        }

        for header in &self.includes_header {
            cmd.arg("-H").arg(header);
        }

        for before in &self.includes_before {
            cmd.arg("-B").arg(before);
        }

        for after in &self.includes_after {
            cmd.arg("-A").arg(after);
        }

        for (key, value) in &self.variables {
            cmd.arg("-V").arg(format!("{}:{}", key, value));
        }

        if let Some(output) = &self.output {
            cmd.arg("-o").arg(output);
        }

        cmd.arg(input);

        let output = cmd
            .output()
            .context("Failed to execute unidoc. Is it installed and in PATH?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            // Log stderr to help with debugging
            eprintln!("\n--- unidoc failed ---");
            eprintln!("Exit code: {:?}", output.status.code());
            if !stderr.is_empty() {
                eprintln!("stderr:\n{}", stderr);
            }
            if !stdout.is_empty() {
                eprintln!("stdout:\n{}", stdout);
            }
            eprintln!("--- end unidoc error ---\n");

            anyhow::bail!("unidoc failed with exit code {:?}", output.status.code());
        }

        Ok(())
    }
}

impl Default for UnidocCommand {
    fn default() -> Self {
        Self::new()
    }
}

// Check if unidoc is available
pub fn check_unidoc_available() -> Result<()> {
    Command::new("unidoc")
        .arg("--version")
        .output()
        .context("unidoc not found. Please install unidoc first.")?;
    Ok(())
}
