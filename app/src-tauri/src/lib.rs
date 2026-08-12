mod hardware;
mod system_check;

use serde::Serialize;
use std::{env, path::PathBuf, process::Command};

#[derive(Serialize)]
pub struct EngineStatus {
    pub running: bool,
    pub runtime_dir: String,
    pub engine_dir: String,
}

fn engine_dir() -> PathBuf {
    if let Ok(value) = env::var("WAN2GP_ROOT") { return PathBuf::from(value); }
    if let Ok(value) = env::var("WAN_GP_ROOT") { return PathBuf::from(value); }
    PathBuf::from("Wan2GP")
}

fn runtime_dir() -> PathBuf {
    if let Ok(value) = env::var("WAN2GP_RUNTIME") { return PathBuf::from(value); }
    engine_dir().join("runtime")
}

#[tauri::command]
fn engine_status() -> EngineStatus {
    let engine = engine_dir();
    let runtime = runtime_dir();
    let running = std::net::TcpStream::connect("127.0.0.1:18765").is_ok();
    EngineStatus { running, runtime_dir: runtime.to_string_lossy().into_owned(), engine_dir: engine.to_string_lossy().into_owned() }
}

#[tauri::command]
fn hardware_info() -> hardware::HardwareInfo { hardware::detect() }

#[tauri::command]
fn system_check() -> system_check::SystemCheck { system_check::run() }

#[tauri::command]
fn start_engine() -> Result<String, String> {
    if std::net::TcpStream::connect("127.0.0.1:18765").is_ok() { return Ok("WanGP bridge already running".into()); }
    let root = engine_dir();
    let launcher = root.join("start_studio.py");
    if !launcher.exists() { return Err(format!("WanGP launcher not found: {}", launcher.display())); }
    Command::new("python").arg(&launcher).current_dir(&root).spawn().map_err(|e| format!("Failed to start WanGP: {e}"))?;
    Ok("WanGP start requested".into())
}

#[tauri::command]
fn stop_engine() -> Result<String, String> {
    Ok("Stop is delegated to the configured WanGP runtime lifecycle".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![engine_status, hardware_info, system_check, start_engine, stop_engine])
        .run(tauri::generate_context!())
        .expect("error while running AI Creator Studio");
}
