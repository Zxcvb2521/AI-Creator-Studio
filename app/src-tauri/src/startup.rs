use serde::Serialize;
use std::{thread, time::Duration};

#[derive(Debug, Serialize)]
pub struct StartupStep {
    pub id: String,
    pub status: String,
    pub detail: String,
}

#[derive(Debug, Serialize)]
pub struct StartupReport {
    pub ready: bool,
    pub steps: Vec<StartupStep>,
}

fn step(id: &str, status: &str, detail: impl Into<String>) -> StartupStep {
    StartupStep { id: id.into(), status: status.into(), detail: detail.into() }
}

pub fn run() -> StartupReport {
    let mut steps = Vec::new();
    let check = crate::system_check::run();
    if !check.ready {
        steps.push(step("system", "blocked", "Required runtime components are missing"));
        return StartupReport { ready: false, steps };
    }
    steps.push(step("system", "ready", "System requirements passed"));

    if std::net::TcpStream::connect("127.0.0.1:18765").is_err() {
        match crate::start_engine() {
            Ok(detail) => steps.push(step("engine", "starting", detail)),
            Err(detail) => {
                steps.push(step("engine", "failed", detail));
                return StartupReport { ready: false, steps };
            }
        }
    }

    for _ in 0..60 {
        if std::net::TcpStream::connect("127.0.0.1:18765").is_ok() {
            steps.push(step("bridge", "ready", "WanGP Studio bridge is reachable"));
            return StartupReport { ready: true, steps };
        }
        thread::sleep(Duration::from_millis(500));
    }

    steps.push(step("bridge", "timeout", "WanGP bridge did not become reachable within 30 seconds"));
    StartupReport { ready: false, steps }
}
