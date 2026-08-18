mod assets;
mod capabilities;
mod commands;
mod hardware;
mod jobs;
mod model_catalog;
mod project;
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
fn engine_dir() -> PathBuf { env::var("WAN2GP_ROOT").or_else(|_| env::var("WAN_GP_ROOT")).map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("Wan2GP")) }
fn runtime_dir() -> PathBuf { env::var("WAN2GP_RUNTIME").map(PathBuf::from).unwrap_or_else(|_| engine_dir().join("runtime")) }
fn bridge_script() -> PathBuf {
    if let Ok(v) = env::var("AI_CREATOR_WANGP_ADAPTER") { return PathBuf::from(v); }
    if let Ok(v) = env::var("AI_CREATOR_WAN2GP_ADAPTER") { return PathBuf::from(v); }
    if let Ok(v) = env::var("AI_CREATOR_STUDIO_ROOT") { return PathBuf::from(v).join("engine/wan-gp-adapter/wan_gp_api.py"); }
    PathBuf::from("engine/wan-gp-adapter/wan_gp_api.py")
}
fn python_command() -> std::process::Command { std::process::Command::new(resolve_python().to_string_lossy().as_ref()) }
fn resolve_python() -> PathBuf {
    for key in ["WAN2GP_PYTHON", "WAN_GP_PYTHON", "AI_CREATOR_PYTHON"] { if let Ok(value) = env::var(key) { let path = PathBuf::from(value); if path.exists() { return path; } } }
    let root = engine_dir();
    if cfg!(target_os = "windows") {
        for relative in ["venv/Scripts/python.exe", ".venv/Scripts/python.exe", "python/python.exe"] { let path = root.join(relative); if path.exists() { return path; } }
        if let Ok(value) = env::var("LOCALAPPDATA") { let path = PathBuf::from(value).join("Programs/Python/Python311/python.exe"); if path.exists() { return path; } }
        PathBuf::from("python")
    } else {
        for relative in ["venv/bin/python", ".venv/bin/python", "python/bin/python"] { let path = root.join(relative); if path.exists() { return path; } }
        PathBuf::from("python3")
    }
}
fn run_adapter(args: &[String]) -> Result<String, String> {
    let script = bridge_script(); if !script.exists() { return Err(format!("WanGP adapter not found: {}", script.display())); }
    let mut command = python_command(); command.arg(&script).arg("--root").arg(engine_dir()); for arg in args { command.arg(arg); }
    let output = command.output().map_err(|e| format!("Failed to launch WanGP adapter with {}: {e}", resolve_python().display()))?;
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string(); let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !output.status.success() { if let Ok(envelope) = serde_json::from_str::<AdapterEnvelope>(&stdout) { if let Some(error) = envelope.error { return Err(error); } } return Err(if !stderr.is_empty() { stderr } else { stdout }); }
    Ok(stdout)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() { tauri::Builder::default().plugin(tauri_plugin_shell::init()).invoke_handler(tauri::generate_handler![commands::engine_status, commands::hardware_info, commands::system_check, commands::capabilities, commands::model_catalog, commands::model_schema, commands::startup, commands::start_engine, commands::generate, commands::generation_status, commands::cancel_generation, commands::record_generation_assets, commands::asset_catalog, commands::project_load, commands::project_save, commands::stop_engine]).run(tauri::generate_context!()).expect("error while running AI Creator Studio"); }
