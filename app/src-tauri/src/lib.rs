mod assets;
mod capabilities;
mod commands;
mod hardware;
mod jobs;
mod model_catalog;
mod project;
mod runtime;
mod startup;
mod system_check;

use serde::Deserialize;
use std::{env, path::PathBuf};
use std::sync::OnceLock;

#[derive(serde::Serialize)]
pub struct EngineStatus { pub running: bool, pub runtime_dir: String, pub engine_dir: String }
#[derive(Deserialize)] struct AdapterEnvelope { error: Option<String> }
static JOBS: OnceLock<jobs::JobManager> = OnceLock::new();
fn job_manager() -> &'static jobs::JobManager { JOBS.get_or_init(jobs::JobManager::new) }
fn engine_dir() -> PathBuf { env::var("WAN2GP_ROOT").or_else(|_| env::var("WAN_GP_ROOT")).map(PathBuf::from).unwrap_or_else(|_| runtime::root().join("wan2gp")) }
fn runtime_dir() -> PathBuf { env::var("WAN2GP_RUNTIME").map(PathBuf::from).unwrap_or_else(|_| runtime::root()) }

fn find_upward(start: &std::path::Path, relative: &str) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    for _ in 0..8 {
        let candidate = dir.join(relative);
        if candidate.exists() { return Some(candidate); }
        if !dir.pop() { break; }
    }
    None
}

fn bridge_script() -> PathBuf {
    if let Ok(v) = env::var("AI_CREATOR_WANGP_ADAPTER") { return PathBuf::from(v); }
    if let Ok(v) = env::var("AI_CREATOR_WAN2GP_ADAPTER") { return PathBuf::from(v); }
    if let Ok(v) = env::var("AI_CREATOR_STUDIO_ROOT") {
        let candidate = PathBuf::from(v).join("engine/wan-gp-adapter/wan_gp_api.py");
        if candidate.exists() { return candidate; }
    }
    if let Ok(cwd) = env::current_dir() {
        if let Some(candidate) = find_upward(&cwd, "engine/wan-gp-adapter/wan_gp_api.py") { return candidate; }
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(candidate) = find_upward(&exe, "engine/wan-gp-adapter/wan_gp_api.py") { return candidate; }
    }
    let manifest_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Some(root) = manifest_root.parent().and_then(|p| p.parent()) {
        let candidate = root.join("engine/wan-gp-adapter/wan_gp_api.py");
        if candidate.exists() { return candidate; }
    }
    PathBuf::from("engine/wan-gp-adapter/wan_gp_api.py")
}

fn python_command() -> std::process::Command { std::process::Command::new(resolve_python().to_string_lossy().as_ref()) }
fn resolve_python() -> PathBuf { if let Some(python) = runtime::detector::find_python(&runtime::root()) { return PathBuf::from(python); } PathBuf::from("python") }
fn run_adapter(args: &[String]) -> Result<String, String> {
    let script = bridge_script();
    let root = engine_dir();
    if !script.exists() { return Err(format!("Wan2GP adapter not found: {}", script.display())); }
    if !root.join("wgp.py").exists() { return Err(format!("Wan2GP root does not exist: {}", root.display())); }
    let python = resolve_python();
    let mut command = python_command();
    command
        .current_dir(&root)
        .env("PYTHONNOUSERSITE", "1")
        .env("PYTHONUTF8", "1")
        .env("PYTHONUNBUFFERED", "1")
        .env("WAN2GP_ROOT", &root)
        .env("HF_HOME", runtime_dir().join("models").join("huggingface"))
        .arg(&script)
        .arg("--root")
        .arg(&root);
    for arg in args { command.arg(arg); }
    let output = command.output().map_err(|e| format!("Failed to launch Wan2GP adapter with {}: {e}", python.display()))?;
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !output.status.success() {
        if let Ok(envelope) = serde_json::from_str::<AdapterEnvelope>(&stdout) { if let Some(error) = envelope.error { return Err(error); } }
        return Err(if !stderr.is_empty() { stderr } else { stdout });
    }
    Ok(stdout)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() { tauri::Builder::default().plugin(tauri_plugin_shell::init()).invoke_handler(tauri::generate_handler![commands::engine_status, commands::hardware_info, commands::system_check, commands::capabilities, commands::model_catalog, commands::model_schema, commands::startup, commands::start_engine, commands::generate, commands::generation_status, commands::cancel_generation, commands::record_generation_assets, commands::asset_catalog, commands::project_load, commands::project_save, commands::stop_engine, commands::runtime_status, commands::runtime_install]).run(tauri::generate_context!()).expect("error while running AI Creator Studio"); }
