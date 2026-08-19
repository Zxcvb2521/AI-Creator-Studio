use std::path::{Path, PathBuf};
use std::process::Command;

pub fn nvidia_gpu() -> Option<String> {
    let output = Command::new("nvidia-smi").args(["--query-gpu=name,memory.total", "--format=csv,noheader,nounits"]).output().ok()?;
    if !output.status.success() { return None; }
    let line = String::from_utf8_lossy(&output.stdout).lines().next()?.trim().to_string();
    if line.is_empty() { None } else { Some(line) }
}

pub fn find_python(runtime: &Path) -> Option<String> {
    let candidates = [
        runtime.join("python").join("python.exe"),
        runtime.join("venv").join("Scripts").join("python.exe"),
        PathBuf::from("python"),
        PathBuf::from("py"),
    ];
    for candidate in candidates {
        let result = if candidate.to_string_lossy() == "py" { Command::new(&candidate).args(["-3.11", "--version"]).output() } else { Command::new(&candidate).arg("--version").output() };
        if let Ok(output) = result { if output.status.success() { return Some(candidate.to_string_lossy().into_owned()); } }
    }
    None
}

pub fn python_version(python: &str) -> Option<String> {
    let mut command = Command::new(python);
    if python == "py" { command.args(["-3.11", "--version"]); } else { command.arg("--version"); }
    let output = command.output().ok()?;
    let text = format!("{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    Some(text.trim().to_string())
}
