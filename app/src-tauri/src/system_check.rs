use serde::Serialize;
use std::{env, path::PathBuf, process::Command};

#[derive(Debug, Serialize, Clone)]
pub struct CheckItem {
    pub name: String,
    pub status: String,
    pub detail: String,
    pub required: bool,
}

#[derive(Debug, Serialize)]
pub struct SystemCheck {
    pub ready: bool,
    pub items: Vec<CheckItem>,
}

fn command_exists(program: &str) -> bool {
    Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg(program)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn engine_root() -> PathBuf {
    env::var("WAN2GP_ROOT").or_else(|_| env::var("WAN_GP_ROOT")).map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("Wan2GP"))
}

pub fn run() -> SystemCheck {
    let mut items = Vec::new();
    let python = command_exists("python");
    items.push(CheckItem { name: "Python".into(), status: if python { "ready" } else { "missing" }.into(), detail: if python { "Python executable found" } else { "Install Python or configure WanGP runtime" }.into(), required: true });

    let ffmpeg = command_exists("ffmpeg");
    items.push(CheckItem { name: "FFmpeg".into(), status: if ffmpeg { "ready" } else { "limited" }.into(), detail: if ffmpeg { "FFmpeg executable found" } else { "Not found on PATH; export/render features may be unavailable" }.into(), required: false });

    let root = engine_root();
    let wan = root.exists();
    items.push(CheckItem { name: "WanGP".into(), status: if wan { "ready" } else { "missing" }.into(), detail: root.to_string_lossy().into_owned(), required: true });

    let bridge = std::net::TcpStream::connect("127.0.0.1:18765").is_ok();
    items.push(CheckItem { name: "WanGP Bridge".into(), status: if bridge { "ready" } else { "limited" }.into(), detail: "127.0.0.1:18765".into(), required: true });

    let ready = items.iter().all(|x| !x.required || x.status == "ready");
    SystemCheck { ready, items }
}
