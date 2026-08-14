mod capabilities;
mod hardware;
mod model_catalog;
mod startup;
mod system_check;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, fs, path::PathBuf, process::Command, time::{SystemTime, UNIX_EPOCH}};

#[derive(Serialize)]
pub struct EngineStatus {
    pub running: bool,
    pub runtime_dir: String,
    pub engine_dir: String,
}

#[derive(Deserialize)]
struct AdapterEnvelope {
    error: Option<String>,
}

fn engine_dir() -> PathBuf {
    env::var("WAN2GP_ROOT")
        .or_else(|_| env::var("WAN_GP_ROOT"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("Wan2GP"))
}

fn runtime_dir() -> PathBuf {
    env::var("WAN2GP_RUNTIME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| engine_dir().join("runtime"))
}

fn bridge_script() -> PathBuf {
    if let Ok(v) = env::var("AI_CREATOR_WANGP_ADAPTER") {
        return PathBuf::from(v);
    }
    if let Ok(v) = env::var("AI_CREATOR_STUDIO_ROOT") {
        return PathBuf::from(v).join("engine/wan-gp-adapter/wan_gp_api.py");
    }
    PathBuf::from("engine/wan-gp-adapter/wan_gp_api.py")
}

fn python_command() -> Command {
    if cfg!(target_os = "windows") {
        Command::new("python")
    } else {
        Command::new("python3")
    }
}

fn run_adapter(args: &[String]) -> Result<String, String> {
    let script = bridge_script();
    if !script.exists() {
        return Err(format!("WanGP adapter not found: {}", script.display()));
    }

    let mut command = python_command();
    command.arg(&script).arg("--root").arg(engine_dir());
    for arg in args {
        command.arg(arg);
    }

    let output = command
        .output()
        .map_err(|e| format!("Failed to launch WanGP adapter: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if !output.status.success() {
        if !stdout.is_empty() {
            if let Ok(envelope) = serde_json::from_str::<AdapterEnvelope>(&stdout) {
                if let Some(error) = envelope.error {
                    return Err(error);
                }
            }
        }
        return Err(if !stderr.is_empty() { stderr } else { stdout });
    }

    Ok(stdout)
}

#[tauri::command]
pub fn engine_status() -> EngineStatus {
    let engine = engine_dir();
    let runtime = runtime_dir();
    let ready = bridge_script().exists() && engine.exists();
    EngineStatus {
        running: ready,
        runtime_dir: runtime.to_string_lossy().into_owned(),
        engine_dir: engine.to_string_lossy().into_owned(),
    }
}

#[tauri::command]
pub fn hardware_info() -> hardware::HardwareInfo { hardware::detect() }

#[tauri::command]
pub fn system_check() -> system_check::SystemCheck { system_check::run() }

#[tauri::command]
pub fn capabilities() -> capabilities::Capabilities { capabilities::detect() }

#[tauri::command]
pub fn model_catalog() -> model_catalog::ModelCatalog { model_catalog::discover() }

#[tauri::command]
pub fn start_engine() -> Result<String, String> {
    let _ = run_adapter(&["models".into()])?;
    Ok("WanGP adapter is ready".into())
}

#[tauri::command]
pub fn generate(model_type: String, settings: Value) -> Result<Value, String> {
    if model_type.trim().is_empty() {
        return Err("model_type is required".into());
    }
    let runtime = runtime_dir();
    fs::create_dir_all(&runtime).map_err(|e| format!("Failed to create runtime directory: {e}"))?;
    let output_dir = runtime.join("generations");
    fs::create_dir_all(&output_dir).map_err(|e| format!("Failed to create output directory: {e}"))?;

    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Clock error: {e}"))?
        .as_millis();
    let settings_path = runtime.join(format!("generation-{stamp}.json"));

    let mut request = settings;
    if !request.is_object() {
        return Err("settings must be a JSON object".into());
    }
    request["model_type"] = Value::String(model_type.clone());
    fs::write(&settings_path, serde_json::to_vec_pretty(&request).map_err(|e| format!("Invalid settings: {e}"))?)
        .map_err(|e| format!("Failed to write generation settings: {e}"))?;

    let args = vec![
        "--output-dir".into(), output_dir.to_string_lossy().into_owned(),
        "--model".into(), model_type,
        "generate".into(),
        "--settings".into(), settings_path.to_string_lossy().into_owned(),
    ];
    let result = run_adapter(&args);
    let _ = fs::remove_file(settings_path);
    let payload = result?;
    serde_json::from_str(&payload).map_err(|e| format!("Invalid adapter response: {e}"))
}

#[tauri::command]
pub fn stop_engine() -> Result<String, String> {
    Ok("WanGP adapter is invoked per operation; no persistent process to stop".into())
}

#[tauri::command]
fn startup() -> startup::StartupReport { startup::run() }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            engine_status,
            hardware_info,
            system_check,
            capabilities,
            model_catalog,
            startup,
            start_engine,
            generate,
            stop_engine
        ])
        .run(tauri::generate_context!())
        .expect("error while running AI Creator Studio");
}
