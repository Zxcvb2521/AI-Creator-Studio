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

pub fn root() -> PathBuf {
    if let Ok(path) = std::env::var("AI_CREATOR_STUDIO_ROOT") { return PathBuf::from(path).join("runtime"); }
    if let Ok(local) = std::env::var("LOCALAPPDATA") { return PathBuf::from(local).join("AI Creator Studio").join("runtime"); }
    PathBuf::from("runtime")
}

pub fn status() -> RuntimeStatus {
    let runtime = root();
    let engine = runtime.join("Wan2GP");
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
