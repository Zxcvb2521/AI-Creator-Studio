mod assets;
mod capabilities;
mod hardware;
mod jobs;
mod model_catalog;
mod project;
mod startup;
mod system_check;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, fs, path::{Path, PathBuf}};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
pub struct EngineStatus { pub running: bool, pub runtime_dir: String, pub engine_dir: String }

#[derive(Deserialize)]
struct AdapterEnvelope { error: Option<String> }

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
    for key in ["WAN2GP_PYTHON", "WAN_GP_PYTHON", "AI_CREATOR_PYTHON"] {
        if let Ok(value) = env::var(key) {
            let path = PathBuf::from(value);
            if path.exists() { return path; }
        }
    }
    let root = engine_dir();
    if cfg!(target_os = "windows") {
        for relative in ["venv/Scripts/python.exe", ".venv/Scripts/python.exe", "python/python.exe"] {
            let path = root.join(relative);
            if path.exists() { return path; }
        }
        if let Ok(value) = env::var("LOCALAPPDATA") {
            let path = PathBuf::from(value).join("Programs/Python/Python311/python.exe");
            if path.exists() { return path; }
        }
        PathBuf::from("python")
    } else {
        for relative in ["venv/bin/python", ".venv/bin/python", "python/bin/python"] {
            let path = root.join(relative);
            if path.exists() { return path; }
        }
        PathBuf::from("python3")
    }
}
fn run_adapter(args: &[String]) -> Result<String, String> {
    let script = bridge_script();
    if !script.exists() { return Err(format!("WanGP adapter not found: {}", script.display())); }
    let mut command = python_command(); command.arg(&script).arg("--root").arg(engine_dir());
    for arg in args { command.arg(arg); }
    let output = command.output().map_err(|e| format!("Failed to launch WanGP adapter with {}: {e}", resolve_python().display()))?;
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !output.status.success() {
        if let Ok(envelope) = serde_json::from_str::<AdapterEnvelope>(&stdout) { if let Some(error) = envelope.error { return Err(error); } }
        return Err(if !stderr.is_empty() { stderr } else { stdout });
    }
    Ok(stdout)
}

#[tauri::command]
pub fn engine_status() -> EngineStatus { let engine = engine_dir(); let runtime = runtime_dir(); EngineStatus { running: bridge_script().exists() && engine.exists(), runtime_dir: runtime.to_string_lossy().into_owned(), engine_dir: engine.to_string_lossy().into_owned() } }
#[tauri::command] pub fn hardware_info() -> hardware::HardwareInfo { hardware::detect() }
#[tauri::command] pub fn system_check() -> system_check::SystemCheck { system_check::run() }
#[tauri::command] pub fn capabilities() -> capabilities::Capabilities { capabilities::detect() }
#[tauri::command] pub fn model_catalog() -> model_catalog::ModelCatalog { model_catalog::discover() }

#[tauri::command]
pub fn model_schema(model_type: String) -> Result<Value, String> {
    if model_type.trim().is_empty() { return Err("model_type is required".into()); }
    let payload = run_adapter(&["schema".into(), "--model".into(), model_type])?;
    serde_json::from_str(&payload).map_err(|e| format!("Invalid schema response: {e}"))
}

#[tauri::command]
pub fn start_engine() -> Result<String, String> { let _ = run_adapter(&["models".into()])?; Ok(format!("WanGP adapter is ready · Python: {}", resolve_python().display())) }

#[tauri::command]
pub fn generate(model_type: String, settings: Value) -> Result<String, String> {
    if model_type.trim().is_empty() { return Err("model_type is required".into()); }
    if !settings.is_object() { return Err("settings must be a JSON object".into()); }
    let runtime = runtime_dir(); fs::create_dir_all(&runtime).map_err(|e| format!("Failed to create runtime directory: {e}"))?;
    let output_dir = runtime.join("generations");
    let mut request = settings;
    request["model_type"] = Value::String(model_type);
    job_manager().submit(resolve_python(), bridge_script(), engine_dir(), output_dir, request["model_type"].as_str().unwrap_or_default().to_string(), request)
}

#[tauri::command]
pub fn generation_status(job_id: String) -> Result<jobs::JobSnapshot, String> { job_manager().get(&job_id) }

#[tauri::command]
pub fn record_generation_assets(job_id: String, model_type: String) -> Result<Vec<assets::AssetRecord>, String> {
    let job = job_manager().get(&job_id)?;
    if !matches!(job.state, jobs::JobState::Completed) { return Err("Generation job is not completed".into()); }
    let result = job.result.ok_or_else(|| "Generation job has no result".to_string())?;
    let created_at = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| format!("Clock error: {e}"))?.as_secs();
    assets::from_generation(runtime_dir(), &result, model_type, created_at)
}

#[tauri::command]
pub fn asset_catalog() -> Result<Vec<assets::AssetRecord>, String> { assets::list(runtime_dir()) }
#[tauri::command]
pub fn project_load() -> Result<project::ProjectState, String> { project::load(runtime_dir()) }
#[tauri::command]
pub fn project_save(state: project::ProjectState) -> Result<project::ProjectState, String> { project::save(runtime_dir(), &state) }
#[tauri::command] pub fn stop_engine() -> Result<String, String> { Ok("WanGP adapter is invoked per operation; no persistent process to stop".into()) }
#[tauri::command] fn startup() -> startup::StartupReport { startup::run() }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() { tauri::Builder::default().plugin(tauri_plugin_shell::init()).invoke_handler(tauri::generate_handler![engine_status, hardware_info, system_check, capabilities, model_catalog, model_schema, startup, start_engine, generate, generation_status, record_generation_assets, asset_catalog, project_load, project_save, stop_engine]).run(tauri::generate_context!()).expect("error while running AI Creator Studio"); }
