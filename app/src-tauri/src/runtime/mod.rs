pub mod detector;
pub mod installer;
pub mod manifest;

use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeStatus {
    pub ready: bool,
    pub phase: String,
    pub detail: String,
    pub runtime_dir: String,
    pub engine_dir: String,
    pub python: Option<String>,
    pub gpu: Option<String>,
}

/// Development uses the Studio source root. Packaged builds keep runtime beside the executable.
pub fn root() -> PathBuf {
    if let Ok(path) = std::env::var("AI_CREATOR_STUDIO_ROOT") {
        return PathBuf::from(path).join("runtime");
    }

    #[cfg(debug_assertions)]
    {
        let manifest_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        if let Some(studio_root) = manifest_root.parent() {
            if let Some(runtime) = studio_root.parent().map(|p| p.join("runtime")) {
                return runtime;
            }
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            return parent.join("runtime");
        }
    }
    PathBuf::from("runtime")
}

pub fn status() -> RuntimeStatus {
    let runtime = root();
    let engine = runtime.join("wan2gp");
    let python = detector::find_python(&runtime);
    let gpu = detector::nvidia_gpu();
    let ready = manifest::is_ready(&runtime);
    RuntimeStatus {
        ready,
        phase: if ready { "ready" } else { "not_installed" }.into(),
        detail: if ready { "Wan2GP runtime is installed" } else { "Wan2GP runtime is not installed" }.into(),
        runtime_dir: runtime.to_string_lossy().into_owned(),
        engine_dir: engine.to_string_lossy().into_owned(),
        python,
        gpu,
    }
}

pub fn install() -> Result<RuntimeStatus, String> {
    installer::install(&root())?;
    Ok(status())
}
