mod capabilities;
mod hardware;
mod model_catalog;
mod startup;
mod system_check;

use serde::Serialize;
use std::{env, path::PathBuf, process::Command};

#[derive(Serialize)]
pub struct EngineStatus { pub running: bool, pub runtime_dir: String, pub engine_dir: String }
fn engine_dir() -> PathBuf { env::var("WAN2GP_ROOT").or_else(|_| env::var("WAN_GP_ROOT")).map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("Wan2GP")) }
fn runtime_dir() -> PathBuf { env::var("WAN2GP_RUNTIME").map(PathBuf::from).unwrap_or_else(|_| engine_dir().join("runtime")) }
fn bridge_script() -> PathBuf { if let Ok(v) = env::var("AI_CREATOR_WANGP_ADAPTER") { return PathBuf::from(v); } if let Ok(v) = env::var("AI_CREATOR_STUDIO_ROOT") { return PathBuf::from(v).join("engine/wan-gp-adapter/wan_gp_api.py"); } PathBuf::from("engine/wan-gp-adapter/wan_gp_api.py") }

#[tauri::command] pub fn engine_status() -> EngineStatus { let engine = engine_dir(); let runtime = runtime_dir(); let running = std::net::TcpStream::connect("127.0.0.1:18765").is_ok(); EngineStatus { running, runtime_dir: runtime.to_string_lossy().into_owned(), engine_dir: engine.to_string_lossy().into_owned() } }
#[tauri::command] pub fn hardware_info() -> hardware::HardwareInfo { hardware::detect() }
#[tauri::command] pub fn system_check() -> system_check::SystemCheck { system_check::run() }
#[tauri::command] pub fn capabilities() -> capabilities::Capabilities { capabilities::detect() }
#[tauri::command] pub fn model_catalog() -> model_catalog::ModelCatalog { model_catalog::discover() }
#[tauri::command] pub fn start_engine() -> Result<String, String> { if std::net::TcpStream::connect("127.0.0.1:18765").is_ok() { return Ok("WanGP bridge already running".into()); } let script = bridge_script(); if !script.exists() { return Err(format!("WanGP adapter not found: {}", script.display())); } Command::new("python").arg(&script).arg("--root").arg(engine_dir()).arg("models").spawn().map_err(|e| format!("Failed to start WanGP adapter: {e}"))?; Ok("WanGP adapter start requested".into()) }
#[tauri::command] pub fn stop_engine() -> Result<String, String> { Ok("Bridge lifecycle is managed by the Studio process".into()) }
#[tauri::command] fn startup() -> startup::StartupReport { startup::run() }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() { tauri::Builder::default().plugin(tauri_plugin_shell::init()).invoke_handler(tauri::generate_handler![engine_status, hardware_info, system_check, capabilities, model_catalog, startup, start_engine, stop_engine]).run(tauri::generate_context!()).expect("error while running AI Creator Studio"); }
