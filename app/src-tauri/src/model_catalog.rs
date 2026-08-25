use serde::Serialize;
use std::{env, path::PathBuf, process::Command};
use crate::runtime;

#[derive(Debug, Serialize)]
pub struct ModelEntry { pub id: String, pub label: String, pub kind: String, pub available: bool, pub metadata: serde_json::Value }
#[derive(Debug, Serialize)]
pub struct ModelCatalog { pub models: Vec<ModelEntry>, pub source: String, pub error: Option<String> }

fn engine_root() -> PathBuf {
    env::var("WAN2GP_ROOT")
        .or_else(|_| env::var("WAN_GP_ROOT"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| runtime::root().join("wan2gp"))
}

fn adapter_script() -> PathBuf {
    if let Ok(value) = env::var("AI_CREATOR_WANGP_ADAPTER") { return PathBuf::from(value); }
    if let Ok(value) = env::var("AI_CREATOR_WAN2GP_ADAPTER") { return PathBuf::from(value); }
    if let Ok(value) = env::var("AI_CREATOR_STUDIO_ROOT") {
        return PathBuf::from(value).join("engine/wan-gp-adapter/wan_gp_api.py");
    }
    PathBuf::from("engine/wan-gp-adapter/wan_gp_api.py")
}

fn field(v: &serde_json::Value, keys: &[&str], fallback: String) -> String {
    keys.iter()
        .find_map(|k| v.get(*k).and_then(|x| x.as_str()).map(str::to_owned))
        .unwrap_or(fallback)
}

pub fn discover() -> ModelCatalog {
    let root = engine_root();
    let script = adapter_script();
    if !root.exists() {
        return ModelCatalog { models: vec![], source: "wan2gp-api".into(), error: Some(format!("Wan2GP root not found: {}", root.display())) };
    }
    if !script.exists() {
        return ModelCatalog { models: vec![], source: "wan2gp-api".into(), error: Some(format!("Wan2GP adapter not found: {}", script.display())) };
    }

    // Always use the Studio-managed interpreter so user-site Python packages
    // cannot leak torch/flash-attn/xformers into the Wan2GP process.
    let python = runtime::detector::find_python(&runtime::root())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("python"));
    let mut command = Command::new(&python);
    command
        .arg(&script)
        .arg("--root")
        .arg(&root)
        .arg("models")
        .env("PYTHONNOUSERSITE", "1")
        .env("PYTHONUTF8", "1")
        .env("PYTHONUNBUFFERED", "1");

    let output = match command.output() {
        Ok(v) => v,
        Err(e) => return ModelCatalog { models: vec![], source: "wan2gp-api".into(), error: Some(format!("Failed to invoke Wan2GP adapter with {}: {e}", python.display())) },
    };
    if !output.status.success() {
        return ModelCatalog { models: vec![], source: "wan2gp-api".into(), error: Some(String::from_utf8_lossy(&output.stderr).trim().to_string()) };
    }
    let raw: serde_json::Value = match serde_json::from_slice(&output.stdout) {
        Ok(v) => v,
        Err(e) => return ModelCatalog { models: vec![], source: "wan2gp-api".into(), error: Some(format!("Invalid model catalog: {e}")) },
    };
    let models = raw.as_array().cloned().unwrap_or_default().into_iter().enumerate().map(|(i, item)| {
        let id = field(&item, &["model_type", "id", "name", "model_name"], format!("model-{i}"));
        let label = field(&item, &["label", "title", "name", "model_type"], id.clone());
        let kind = field(&item, &["kind", "type", "task", "model_type"], "unknown".into());
        let available = item.get("available").and_then(|v| v.as_bool()).unwrap_or(true);
        ModelEntry { id, label, kind, available, metadata: item }
    }).collect();
    ModelCatalog { models, source: "wan2gp-api".into(), error: None }
}
