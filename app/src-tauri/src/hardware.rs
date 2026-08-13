use serde::Serialize;
use std::process::Command;

#[derive(Debug, Serialize, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_info_serialization() {
        let info = HardwareInfo {
            os: "windows".to_string(),
            arch: "x86_64".to_string(),
            cpu: "Intel CPU".to_string(),
            ram_gb: Some(16),
            gpu: Some("RTX 4090".to_string()),
            vram_gb: Some(24),
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"os\":\"windows\""));
        assert!(json.contains("\"arch\":\"x86_64\""));
        assert!(json.contains("\"cpu\":\"Intel CPU\""));
        assert!(json.contains("\"ram_gb\":16"));
        assert!(json.contains("\"gpu\":\"RTX 4090\""));
        assert!(json.contains("\"vram_gb\":24"));
    }

    #[test]
    fn test_hardware_info_with_missing_fields() {
        let info = HardwareInfo {
            os: "linux".to_string(),
            arch: "x86_64".to_string(),
            cpu: "AMD CPU".to_string(),
            ram_gb: None,
            gpu: None,
            vram_gb: None,
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"os\":\"linux\""));
        assert!(json.contains("\"ram_gb\":null"));
        assert!(json.contains("\"gpu\":null"));
    }

    #[test]
    fn test_hardware_info_debug_format() {
        let info = HardwareInfo {
            os: "windows".to_string(),
            arch: "x86_64".to_string(),
            cpu: "Test CPU".to_string(),
            ram_gb: Some(32),
            gpu: Some("Test GPU".to_string()),
            vram_gb: Some(12),
        };

        let debug = format!("{:?}", info);
        assert!(debug.contains("HardwareInfo"));
        assert!(debug.contains("os: \"windows\""));
    }

    #[test]
    fn test_hardware_info_clone() {
        let info1 = HardwareInfo {
            os: "windows".to_string(),
            arch: "x86_64".to_string(),
            cpu: "CPU".to_string(),
            ram_gb: Some(16),
            gpu: Some("GPU".to_string()),
            vram_gb: Some(8),
        };

        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_command_text_nonexistent_program() {
        let result = command_text("nonexistent_program_xyz", &[]);
        assert!(result.is_none());
    }

    #[test]
    fn test_detect_returns_valid_structure() {
        let info = detect();
        
        // OS and arch should always be present
        assert!(!info.os.is_empty());
        assert!(!info.arch.is_empty());
        // CPU might not be detected on non-Windows or in CI
        assert!(!info.cpu.is_empty() || info.cpu == "Unknown CPU");
    }
}
