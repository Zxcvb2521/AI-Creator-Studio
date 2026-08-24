use std::path::{Path, PathBuf};
use std::process::Command;

pub fn nvidia_gpu() -> Option<String> {
    let output = Command::new("nvidia-smi").args(["--query-gpu=name,memory.total", "--format=csv,noheader,nounits"]).output().ok()?;
    if !output.status.success() { return None; }
    let line = String::from_utf8_lossy(&output.stdout).lines().next()?.trim().to_string();
    if line.is_empty() { None } else { Some(line) }
}

/// Find the Python interpreter managed by Studio first. We deliberately do not
/// prefer a system Python: the desktop launcher must be self-contained.
pub fn find_python(runtime: &Path) -> Option<String> {
    let candidates = [
        runtime.join("wan2gp").join("env_uv").join("Scripts").join("python.exe"),
        runtime.join("python").join("python.exe"),
        runtime.join("venv").join("Scripts").join("python.exe"),
        runtime.join("wan2gp").join("env_uv").join("bin").join("python"),
        runtime.join("python").join("bin").join("python"),
        runtime.join("venv").join("bin").join("python"),
    ];
    for candidate in candidates {
        if !candidate.exists() { continue; }
        if let Ok(output) = Command::new(&candidate).arg("--version").output() {
            if output.status.success() { return Some(candidate.to_string_lossy().into_owned()); }
        }
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

pub fn uv_path(runtime: &Path) -> PathBuf {
    if cfg!(windows) { runtime.join("bin").join("uv.exe") } else { runtime.join("bin").join("uv") }
}
