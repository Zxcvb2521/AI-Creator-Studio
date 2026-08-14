use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, path::PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetRecord {
    pub id: String,
    pub path: String,
    pub media_type: String,
    pub model_type: String,
    pub created_at: u64,
}

fn manifest_path(runtime_dir: &PathBuf) -> PathBuf { runtime_dir.join("assets.json") }

pub fn record(runtime_dir: PathBuf, asset: AssetRecord) -> Result<AssetRecord, String> {
    fs::create_dir_all(&runtime_dir).map_err(|e| format!("Failed to create runtime directory: {e}"))?;
    let path = manifest_path(&runtime_dir);
    let mut items: Vec<AssetRecord> = if path.exists() {
        serde_json::from_slice(&fs::read(&path).map_err(|e| format!("Failed to read assets manifest: {e}"))?)
            .map_err(|e| format!("Invalid assets manifest: {e}"))?
    } else { Vec::new() };
    items.retain(|item| item.path != asset.path);
    items.insert(0, asset.clone());
    fs::write(&path, serde_json::to_vec_pretty(&items).map_err(|e| format!("Failed to encode assets manifest: {e}"))?)
        .map_err(|e| format!("Failed to save assets manifest: {e}"))?;
    Ok(asset)
}

pub fn list(runtime_dir: PathBuf) -> Result<Vec<AssetRecord>, String> {
    let path = manifest_path(&runtime_dir);
    if !path.exists() { return Ok(Vec::new()); }
    serde_json::from_slice(&fs::read(path).map_err(|e| format!("Failed to read assets manifest: {e}")))
        .map_err(|e| format!("Invalid assets manifest: {e}"))
}

pub fn from_generation(runtime_dir: PathBuf, result: &Value, model_type: String, created_at: u64) -> Result<Vec<AssetRecord>, String> {
    let mut records = Vec::new();
    if let Some(files) = result.get("generated_files").and_then(|v| v.as_array()) {
        for (index, value) in files.iter().enumerate() {
            if let Some(path) = value.as_str() {
                let media_type = result.get("artifacts").and_then(|v| v.as_array()).and_then(|items| items.iter().find(|a| a.get("path").and_then(|p| p.as_str()) == Some(path))).and_then(|a| a.get("media_type")).and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
                records.push(record(runtime_dir.clone(), AssetRecord { id: format!("generation-{created_at}-{index}"), path: path.to_string(), media_type, model_type: model_type.clone(), created_at })?);
            }
        }
    }
    Ok(records)
}
