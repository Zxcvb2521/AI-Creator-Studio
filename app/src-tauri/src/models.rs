use serde::Serialize;
use std::{env, path::PathBuf, process::Command};

#[derive(Debug, Serialize)]
pub struct ModelRecord { pub raw: serde_json::Value }
#[derive(Debug, Serialize)]
pub struct ModelDiscovery { pub status: String, pub detail: String, pub models: Vec<ModelRecord> }

fn engine_root() -> PathBuf {
    env::var("WAN2GP_ROOT").or_else(|_| env::var("WAN_GP_ROOT")).map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("Wan2GP"))
}

fn adapter_script() -> PathBuf {
    if let Ok(root) = env::var("AI_CREATOR_STUDIO_ROOT") {
        return PathBuf::from(root).join("engine/wan-gp-adapter/wan_gp_api.py");
    }
    PathBuf::from("engine/wan-gp-adapter/wan_gp_api.py")
}

pub fn discover() -> ModelDiscovery {
    let root = engine_root();
    if !root.exists() { return ModelDiscovery { status: "unavailable".into(), detail: format!("WanGP root not found: {}", root.display()), models: vec![] }; }
    let script = adapter_script();
    if !script.exists() { return ModelDiscovery { status: "unavailable".into(), detail: format!("Adapter script not found: {}", script.display()), models: vec![] }; }
    let output = Command::new("python").arg(&script).arg("--root").arg(&root).arg("models").output();
    let Ok(output) = output else { return ModelDiscovery { status: "unavailable".into(), detail: "Unable to start Python WanGP API adapter".into(), models: vec![] }; };
    if !output.status.success() { return ModelDiscovery { status: "unavailable".into(), detail: String::from_utf8_lossy(&output.stderr).trim().to_string(), models: vec![] }; }
    match serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout) {
        Ok(values) => ModelDiscovery { status: "available".into(), detail: format!("WanGP API returned {} model definitions", values.len()), models: values.into_iter().map(|raw| ModelRecord { raw }).collect() },
        Err(error) => ModelDiscovery { status: "unavailable".into(), detail: format!("Invalid response from WanGP API adapter: {error}"), models: vec![] },
    }
}
