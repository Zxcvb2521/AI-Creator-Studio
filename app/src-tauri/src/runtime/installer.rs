use std::{fs, path::{Path, PathBuf}, process::Command};
use super::{detector, manifest};

const WAN2GP_URL: &str = "https://github.com/deepbeepmeep/Wan2GP.git";

fn run(program: &str, args: &[&str], cwd: Option<&Path>) -> Result<(), String> {
    let mut cmd = Command::new(program); cmd.args(args); if let Some(dir) = cwd { cmd.current_dir(dir); }
    let output = cmd.output().map_err(|e| format!("Failed to launch {program}: {e}"))?;
    if output.status.success() { return Ok(()); }
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    Err(if stderr.is_empty() { format!("{program} failed with status {}", output.status) } else { stderr })
}

fn ensure_python(runtime: &Path) -> Result<PathBuf, String> {
    if let Some(python) = detector::find_python(runtime) {
        return Ok(PathBuf::from(python));
    }
    // Prefer the official Python launcher when available; no system-wide Python mutation is required.
    if detector::python_version("py").is_some() {
        return Ok(PathBuf::from("py"));
    }
    Err("Python 3.11 is required. Automatic Python bootstrap will be enabled in the packaged installer; this development build will not modify the system Python.".into())
}

fn ensure_engine(root: &Path) -> Result<PathBuf, String> {
    let engine = root.join("Wan2GP");
    if engine.join("wgp.py").exists() { return Ok(engine); }
    fs::create_dir_all(root).map_err(|e| format!("Failed to create runtime directory: {e}"))?;
    if Command::new("git").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
        run("git", &["clone", WAN2GP_URL, engine.to_string_lossy().as_ref()], None)?;
    } else {
        return Err("Git is required for the development bootstrap. The packaged installer will bundle/download the bootstrap tool without requiring Git.".into());
    }
    if !engine.join("wgp.py").exists() { return Err("Wan2GP checkout completed but wgp.py was not found.".into()); }
    Ok(engine)
}

pub fn install(root: &Path) -> Result<(), String> {
    fs::create_dir_all(root).map_err(|e| format!("Failed to create runtime directory: {e}"))?;
    let engine = ensure_engine(root)?;
    let python = ensure_python(root)?;
    let python_display = python.to_string_lossy().into_owned();
    if python_display == "py" {
        run("py", &["-3.11", "-m", "pip", "install", "--upgrade", "pip"], Some(&engine))?;
        run("py", &["-3.11", "-m", "pip", "install", "torch", "torchvision", "torchaudio", "--index-url", "https://download.pytorch.org/whl/cu130"], Some(&engine))?;
        run("py", &["-3.11", "-m", "pip", "install", "-r", "requirements.txt"], Some(&engine))?;
    } else {
        run(python.to_string_lossy().as_ref(), &["-m", "pip", "install", "--upgrade", "pip"], Some(&engine))?;
        run(python.to_string_lossy().as_ref(), &["-m", "pip", "install", "-r", "requirements.txt"], Some(&engine))?;
    }
    manifest::write(root, python_display)?;
    Ok(())
}
