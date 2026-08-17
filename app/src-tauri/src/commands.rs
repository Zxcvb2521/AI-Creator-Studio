use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{assets, capabilities, engine_dir, hardware, job_manager, jobs, model_catalog, project, resolve_python, run_adapter, runtime_dir, system_check, bridge_script};

#[tauri::command]
pub fn engine_status() -> crate::EngineStatus { let engine = engine_dir(); let runtime = runtime_dir(); crate::EngineStatus { running: bridge_script().exists() && engine.exists(), runtime_dir: runtime.to_string_lossy().into_owned(), engine_dir: engine.to_string_lossy().into_owned() } }
#[tauri::command] pub fn hardware_info() -> hardware::HardwareInfo { hardware::detect() }
#[tauri::command] pub fn system_check() -> system_check::SystemCheck { system_check::run() }
#[tauri::command] pub fn capabilities() -> capabilities::Capabilities { capabilities::detect() }
#[tauri::command] pub fn model_catalog() -> model_catalog::ModelCatalog { model_catalog::discover() }
#[tauri::command] pub fn model_schema(model_type: String) -> Result<Value, String> { if model_type.trim().is_empty() { return Err("model_type is required".into()); } let payload = run_adapter(&["schema".into(), "--model".into(), model_type])?; serde_json::from_str(&payload).map_err(|e| format!("Invalid schema response: {e}")) }
#[tauri::command] pub fn start_engine() -> Result<String, String> { let _ = run_adapter(&["models".into()])?; Ok(format!("WanGP adapter is ready · Python: {}", resolve_python().display())) }
#[tauri::command] pub fn generate(model_type: String, settings: Value) -> Result<String, String> { if model_type.trim().is_empty() { return Err("model_type is required".into()); } if !settings.is_object() { return Err("settings must be a JSON object".into()); } let runtime = runtime_dir(); std::fs::create_dir_all(&runtime).map_err(|e| format!("Failed to create runtime directory: {e}"))?; let output_dir = runtime.join("generations"); let mut request = settings; request["model_type"] = Value::String(model_type); job_manager().submit(resolve_python(), bridge_script(), engine_dir(), output_dir, request["model_type"].as_str().unwrap_or_default().to_string(), request) }
#[tauri::command] pub fn generation_status(job_id: String) -> Result<jobs::JobSnapshot, String> { job_manager().get(&job_id) }
#[tauri::command] pub fn cancel_generation(job_id: String) -> Result<jobs::JobSnapshot, String> { job_manager().cancel(&job_id) }
#[tauri::command] pub fn record_generation_assets(job_id: String, model_type: String) -> Result<Vec<assets::AssetRecord>, String> { let job = job_manager().get(&job_id)?; if !matches!(job.state, jobs::JobState::Completed) { return Err("Generation job is not completed".into()); } let result = job.result.ok_or_else(|| "Generation job has no result".to_string())?; let created_at = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| format!("Clock error: {e}"))?.as_secs(); assets::from_generation(runtime_dir(), &result, model_type, created_at) }
#[tauri::command] pub fn asset_catalog() -> Result<Vec<assets::AssetRecord>, String> { assets::list(runtime_dir()) }
#[tauri::command] pub fn project_load() -> Result<project::ProjectState, String> { project::load(runtime_dir()) }
#[tauri::command] pub fn project_save(state: project::ProjectState) -> Result<project::ProjectState, String> { project::save(runtime_dir(), &state) }
#[tauri::command] pub fn stop_engine() -> Result<String, String> { Ok("WanGP adapter is invoked per operation; no persistent process to stop".into()) }
#[tauri::command] pub fn startup() -> crate::startup::StartupReport { crate::startup::run() }
