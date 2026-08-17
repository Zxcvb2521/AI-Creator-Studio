use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, path::PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ProjectState { pub version: u32, pub name: String, pub prompt: String, pub model_type: String, pub settings: Value, pub asset_ids: Vec<String>, pub timeline: Vec<TimelineClip> }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimelineClip { pub id: String, pub asset_id: String, pub start: f64, pub duration: f64 }
fn project_path(runtime_dir: &PathBuf) -> PathBuf { runtime_dir.join("project.json") }
pub fn load(runtime_dir: PathBuf) -> Result<ProjectState, String> {
    let path = project_path(&runtime_dir); if !path.exists() { return Ok(ProjectState { version: 1, ..Default::default() }); }
    let data = fs::read(path).map_err(|e| format!("Failed to read project: {e}"))?;
    serde_json::from_slice(&data).map_err(|e| format!("Invalid project file: {e}"))
}
pub fn save(runtime_dir: PathBuf, state: &ProjectState) -> Result<ProjectState, String> {
    fs::create_dir_all(&runtime_dir).map_err(|e| format!("Failed to create runtime directory: {e}"))?;
    let target = project_path(&runtime_dir); let temp = runtime_dir.join("project.json.tmp");
    let bytes = serde_json::to_vec_pretty(state).map_err(|e| format!("Failed to encode project: {e}"))?;
    fs::write(&temp, bytes).map_err(|e| format!("Failed to write project temp file: {e}"))?;
    fs::rename(&temp, &target).map_err(|e| { let _ = fs::remove_file(&temp); format!("Failed to commit project file: {e}") })?; Ok(state.clone())
}
