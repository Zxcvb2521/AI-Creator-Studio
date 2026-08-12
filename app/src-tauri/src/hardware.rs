use serde::Serialize;
use std::process::Command;

#[derive(Debug, Serialize)]
pub struct HardwareInfo {
    pub os: String,
    pub arch: String,
    pub cpu: String,
    pub ram_gb: Option<u64>,
    pub gpu: Option<String>,
    pub vram_gb: Option<u64>,
}

pub fn detect() -> HardwareInfo {
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();
    let cpu = command_text("powershell", &["-NoProfile", "-Command", "(Get-CimInstance Win32_Processor | Select-Object -First 1 -ExpandProperty Name)"])
        .unwrap_or_else(|| "Unknown CPU".into());
    let ram_gb = command_text("powershell", &["-NoProfile", "-Command", "[math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory/1GB)"])
        .and_then(|v| v.trim().parse().ok());
    let gpu = command_text("nvidia-smi", &["--query-gpu=name", "--format=csv,noheader"])
        .or_else(|| command_text("powershell", &["-NoProfile", "-Command", "(Get-CimInstance Win32_VideoController | Select-Object -First 1 -ExpandProperty Name)"]));
    let vram_gb = command_text("nvidia-smi", &["--query-gpu=memory.total", "--format=csv,noheader,nounits"])
        .and_then(|v| v.trim().parse::<u64>().ok())
        .map(|mb| (mb + 512) / 1024);
    HardwareInfo { os, arch, cpu, ram_gb, gpu, vram_gb }
}

fn command_text(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() { return None; }
    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (!value.is_empty()).then_some(value)
}
