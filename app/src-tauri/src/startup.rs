use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StartupStep { pub id: String, pub status: String, pub detail: String }
#[derive(Debug, Serialize)]
pub struct StartupReport { pub ready: bool, pub steps: Vec<StartupStep> }
fn step(id: &str, status: &str, detail: impl Into<String>) -> StartupStep { StartupStep { id: id.into(), status: status.into(), detail: detail.into() } }

pub fn run() -> StartupReport {
    let mut steps = Vec::new();

    match crate::commands::start_engine() {
        Ok(detail) => steps.push(step("engine", "ready", detail)),
        Err(detail) => steps.push(step("engine", "failed", detail)),
    }

    let check = crate::system_check::run();
    let required_ok = check.items.iter().filter(|item| item.required).all(|item| item.status == "ready");
    if !required_ok {
        for item in check.items.iter().filter(|item| item.required && item.status != "ready") {
            steps.push(step(&item.name.to_lowercase(), "blocked", item.detail.clone()));
        }
        return StartupReport { ready: false, steps };
    }

    steps.push(step("system", "ready", "Required runtime components passed"));
    let engine_ok = steps.iter().any(|item| item.id == "engine" && item.status == "ready");
    StartupReport { ready: engine_ok && required_ok, steps }
}
