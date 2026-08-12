use serde::Serialize;
use std::{env, path::PathBuf, process::Command};

#[derive(Debug, Serialize)]
pub struct ModelEntry {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub available: bool,
}

#[derive(Debug, Serialize)]
pub struct ModelCatalog {
    pub models: Vec<ModelEntry>,
    pub source: String,
    pub error: Option<String>,
}

fn engine_root() -> PathBuf {
    env::var("WAN2GP_ROOT").or_else(|_| env::var("WAN_GP_ROOT")).map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("Wan2GP"))
}

pub fn discover() -> ModelCatalog {
    let root = engine_root();
    let bridge = root.join("engine").join("wan_gp_bridge.py");
    if !bridge.exists() {
        return ModelCatalog { models: Vec::new(), source: "fallback".into(), error: Some(format!("WanGP adapter not found: {}", bridge.display())) };
    }

    let output = Command::new("python")
        .arg(&bridge)
        .arg("list-models")
        .current_dir(&root)
        .output();

    match output {
        Ok(out) if out.status.success() => {
            match serde_json::from_slice::<Vec<ModelEntry>>(&out.stdout) {
                Ok(models) => ModelCatalog { models, source: "wan2gp-api".into(), error: None },
                Err(e) => ModelCatalog { models: Vec::new(), source: "wan2gp-api".into(), error: Some(format!("Invalid model catalog: {e}")) },
            }
        }
        Ok(out) => ModelCatalog { models: Vec::new(), source: "wan2gp-api".into(), error: Some(String::from_utf8_lossy(&out.stderr).trim().to_string()) },
        Err(e) => ModelCatalog { models: Vec::new(), source: "wan2gp-api".into(), error: Some(format!("Failed to invoke WanGP adapter: {e}")) },
    }
}
