use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StartupStep { pub id: String, pub status: String, pub detail: String }
#[derive(Debug, Serialize)]
pub struct StartupReport { pub ready: bool, pub steps: Vec<StartupStep> }
fn step(id: &str, status: &str, detail: impl Into<String>) -> StartupStep { StartupStep { id: id.into(), status: status.into(), detail: detail.into() } }

/// First-run bootstrap is intentionally automatic. The user should only have
/// to download supported models; Git, Conda, system Python and other build
/// prerequisites are not part of the Studio contract.
pub fn run() -> StartupReport {
    let mut steps = Vec::new();
    let runtime = crate::runtime::status();

    if runtime.ready {
        steps.push(step("runtime", "ready", format!("Studio runtime ready at {}", runtime.runtime_dir)));
    } else {
        steps.push(step("runtime", "installing", format!("Installing Wan2GP into {}", runtime.runtime_dir)));
        match crate::runtime::install() {
            Ok(installed) => steps.push(step("runtime", "ready", format!("Installed Wan2GP and managed Python at {}", installed.runtime_dir))),
            Err(detail) => {
                steps.push(step("runtime", "failed", detail));
                return StartupReport { ready: false, steps };
            }
        }
    }

    let check = crate::system_check::run();
    for item in check.items.iter() {
        if item.required {
            steps.push(step(&item.name.to_lowercase(), item.status.as_str(), item.detail.clone()));
        }
    }
    if !check.ready {
        return StartupReport { ready: false, steps };
    }

    match crate::commands::start_engine() {
        Ok(detail) => steps.push(step("engine", "ready", detail)),
        Err(detail) => steps.push(step("engine", "failed", detail)),
    }

    let engine_ok = steps.iter().any(|item| item.id == "engine" && item.status == "ready");
    steps.push(step("system", "ready", "Studio-managed runtime check passed"));
    StartupReport { ready: engine_ok, steps }
}
