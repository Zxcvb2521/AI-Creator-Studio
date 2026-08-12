use serde::Serialize;
use std::{thread, time::Duration};

#[derive(Debug, Serialize)]
pub struct StartupStep { pub id: String, pub status: String, pub detail: String }
#[derive(Debug, Serialize)]
pub struct StartupReport { pub ready: bool, pub steps: Vec<StartupStep> }
fn step(id: &str, status: &str, detail: impl Into<String>) -> StartupStep { StartupStep { id: id.into(), status: status.into(), detail: detail.into() } }

pub fn run() -> StartupReport {
    let mut steps = Vec::new();

    // The bridge is a process started by Studio, so it must not be a prerequisite
    // for the system check itself. Start it first, then validate the full system.
    if std::net::TcpStream::connect("127.0.0.1:18765").is_err() {
        match crate::start_engine() {
            Ok(detail) => steps.push(step("engine", "starting", detail)),
            Err(detail) => steps.push(step("engine", "failed", detail)),
        }
    } else {
        steps.push(step("engine", "ready", "WanGP Studio bridge already running"));
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

    for _ in 0..60 {
        if std::net::TcpStream::connect("127.0.0.1:18765").is_ok() {
            steps.push(step("bridge", "ready", "WanGP API bridge is reachable"));
            return StartupReport { ready: true, steps };
        }
        thread::sleep(Duration::from_millis(500));
    }
    steps.push(step("bridge", "timeout", "WanGP API bridge did not become reachable within 30 seconds"));
    StartupReport { ready: false, steps }
}
