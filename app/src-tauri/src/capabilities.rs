use serde::Serialize;
use std::{env, fs, path::{Path, PathBuf}};

#[derive(Debug, Serialize)]
pub struct Capability {
    pub id: String,
    pub label: String,
    pub status: String,
    pub detail: String,
}

#[derive(Debug, Serialize)]
pub struct Capabilities {
    pub engine: String,
    pub root: String,
    pub capabilities: Vec<Capability>,
}

fn root() -> PathBuf {
    env::var("WAN2GP_ROOT").or_else(|_| env::var("WAN_GP_ROOT")).map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("Wan2GP"))
}

fn contains_any(root: &Path, names: &[&str]) -> bool {
    names.iter().any(|name| root.join(name).exists())
}

fn capability(id: &str, label: &str, status: &str, detail: impl Into<String>) -> Capability {
    Capability { id: id.into(), label: label.into(), status: status.into(), detail: detail.into() }
}

pub fn detect() -> Capabilities {
    let root = root();
    let video = contains_any(&root, &["wan", "wan2", "wan2gp.py", "app.py"]);
    let image = contains_any(&root, &["image", "i2v", "t2i"]);
    let text = contains_any(&root, &["deepy", "text", "prompt"]);
    let deepy = contains_any(&root, &["deepy", "Deepy"]);
    let audio = contains_any(&root, &["audio", "tts", "voice"]);
    let _ = fs::metadata(&root);

    Capabilities {
        engine: "Wan2GP".into(),
        root: root.to_string_lossy().into_owned(),
        capabilities: vec![
            capability("video", "Video generation", if video { "available" } else { "unknown" }, "Detected from installed Wan2GP files"),
            capability("image", "Image generation", if image { "available" } else { "unknown" }, "Capability requires confirmation from the installed engine"),
            capability("text", "Text / prompt generation", if text { "available" } else { "unknown" }, "Capability requires confirmation from the installed engine"),
            capability("deepy", "Deepy", if deepy { "available" } else { "unknown" }, "Deepy is treated as an engine capability, not a separate bundled model"),
            capability("audio", "Audio / TTS", if audio { "available" } else { "unknown" }, "Capability requires confirmation from the installed engine"),
        ],
    }
}
