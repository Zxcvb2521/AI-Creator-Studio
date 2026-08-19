use std::{fs, path::Path};
use serde::{Deserialize, Serialize};

const MANIFEST: &str = "runtime.json";
const ENGINE_URL: &str = "https://github.com/deepbeepmeep/Wan2GP.git";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeManifest { pub engine: String, pub engine_url: String, pub python: String, pub installed: bool }

pub fn path(root: &Path) -> std::path::PathBuf { root.join(MANIFEST) }
pub fn is_ready(root: &Path) -> bool { fs::read(path(root)).ok().and_then(|bytes| serde_json::from_slice::<RuntimeManifest>(&bytes).ok()).map(|m| m.installed && root.join("wan2gp").join("wgp.py").exists()).unwrap_or(false) }
pub fn write(root: &Path, python: String) -> Result<(), String> {
    fs::create_dir_all(root).map_err(|e| format!("Failed to create runtime directory: {e}"))?;
    let manifest = RuntimeManifest { engine: "Wan2GP".into(), engine_url: ENGINE_URL.into(), python, installed: true };
    let data = serde_json::to_vec_pretty(&manifest).map_err(|e| format!("Failed to serialize runtime manifest: {e}"))?;
    fs::write(path(root), data).map_err(|e| format!("Failed to write runtime manifest: {e}"))
}
