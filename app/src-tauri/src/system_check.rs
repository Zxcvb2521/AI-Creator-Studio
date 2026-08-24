use serde::Serialize;
use std::{env, path::PathBuf, process::Command};

#[derive(Debug, Serialize, Clone)]
pub struct CheckItem { pub name: String, pub status: String, pub detail: String, pub required: bool }
#[derive(Debug, Serialize)]
pub struct SystemCheck { pub ready: bool, pub items: Vec<CheckItem> }

fn command_exists(program: &str) -> bool {
    Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg(program)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn engine_root() -> PathBuf {
    env::var("WAN2GP_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| crate::runtime::root().join("wan2gp"))
}

pub fn run() -> SystemCheck {
    let root = engine_root();
    let runtime = crate::runtime::root();
    let managed_python = crate::runtime::detector::find_python(&runtime);
    let wan = root.join("wgp.py").exists();
    let uv = crate::runtime::detector::uv_path(&runtime).exists() || command_exists("uv");
    let nvidia = crate::runtime::detector::nvidia_gpu();
    let ffmpeg = command_exists("ffmpeg");

    let mut items = Vec::new();
    items.push(CheckItem {
        name: "Python".into(),
        status: if managed_python.is_some() { "ready" } else { "missing" }.into(),
        detail: managed_python.unwrap_or_else(|| "Studio-managed Python 3.11 is not installed yet".into()),
        required: true,
    });
    items.push(CheckItem {
        name: "Wan2GP".into(),
        status: if wan { "ready" } else { "missing" }.into(),
        detail: root.to_string_lossy().into_owned(),
        required: true,
    });
    items.push(CheckItem {
        name: "uv".into(),
        status: if uv { "ready" } else { "missing" }.into(),
        detail: if uv { "Managed bootstrap tool available" } else { "Studio will download uv automatically" }.into(),
        required: false,
    });
    items.push(CheckItem {
        name: "GPU".into(),
        status: if nvidia.is_some() { "ready" } else { "limited" }.into(),
        detail: nvidia.unwrap_or_else(|| "No NVIDIA GPU detected; Wan2GP may use another backend".into()),
        required: false,
    });
    items.push(CheckItem {
        name: "FFmpeg".into(),
        status: if ffmpeg { "ready" } else { "limited" }.into(),
        detail: if ffmpeg { "FFmpeg executable found" } else { "Not found on PATH; export/render features may be limited" }.into(),
        required: false,
    });

    let ready = items.iter().all(|x| !x.required || x.status == "ready");
    SystemCheck { ready, items }
}
