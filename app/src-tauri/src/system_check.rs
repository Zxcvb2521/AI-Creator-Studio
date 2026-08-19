use serde::Serialize;
use std::{env, path::PathBuf, process::Command};

#[derive(Debug, Serialize, Clone)]
pub struct CheckItem { pub name: String, pub status: String, pub detail: String, pub required: bool }
#[derive(Debug, Serialize)]
pub struct SystemCheck { pub ready: bool, pub items: Vec<CheckItem> }

fn command_exists(program: &str) -> bool { Command::new(if cfg!(windows) { "where" } else { "which" }).arg(program).output().map(|o| o.status.success()).unwrap_or(false) }
fn engine_root() -> PathBuf { env::var("WAN2GP_ROOT").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("F:\\XTTS\\Wan2GP")) }

pub fn run() -> SystemCheck {
    let mut items = Vec::new();
    let python = command_exists("python");
    items.push(CheckItem { name: "Python".into(), status: if python { "ready" } else { "missing" }.into(), detail: if python { "Python executable found" } else { "Install Python or use the Wan2GP installer" }.into(), required: true });
    let git = command_exists("git");
    items.push(CheckItem { name: "Git".into(), status: if git { "ready" } else { "missing" }.into(), detail: if git { "Git executable found" } else { "Git is required to install/update Wan2GP" }.into(), required: true });
    let conda = command_exists("conda");
    items.push(CheckItem { name: "Conda".into(), status: if conda { "ready" } else { "missing" }.into(), detail: if conda { "Conda executable found" } else { "Miniconda or Anaconda is required by the managed installer" }.into(), required: true });
    let ffmpeg = command_exists("ffmpeg");
    items.push(CheckItem { name: "FFmpeg".into(), status: if ffmpeg { "ready" } else { "limited" }.into(), detail: if ffmpeg { "FFmpeg executable found" } else { "Not found on PATH; export/render features may be unavailable" }.into(), required: false });
    let root = engine_root();
    let wan = root.join("wgp.py").exists();
    items.push(CheckItem { name: "Wan2GP".into(), status: if wan { "ready" } else { "missing" }.into(), detail: root.to_string_lossy().into_owned(), required: false });
    let ready = items.iter().all(|x| !x.required || x.status == "ready");
    SystemCheck { ready, items }
}
