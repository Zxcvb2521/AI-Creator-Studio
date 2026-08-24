use std::{fs, path::{Path, PathBuf}, process::Command};
use super::{detector, manifest};

const WAN2GP_ZIP_URL: &str = "https://github.com/deepbeepmeep/Wan2GP/archive/refs/heads/main.zip";
const UV_ZIP_URL: &str = "https://github.com/astral-sh/uv/releases/latest/download/uv-x86_64-pc-windows-msvc.zip";

fn run(program: &str, args: &[&str], cwd: Option<&Path>, envs: &[(&str, &str)]) -> Result<(), String> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    if let Some(dir) = cwd { cmd.current_dir(dir); }
    for (k, v) in envs { cmd.env(k, v); }
    let output = cmd.output().map_err(|e| format!("Failed to launch {program}: {e}"))?;
    if output.status.success() { return Ok(()); }
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Err(if !stderr.is_empty() { stderr } else if !stdout.is_empty() { stdout } else { format!("{program} failed with status {}", output.status) })
}

fn run_powershell(script: &str) -> Result<(), String> {
    let output = Command::new("powershell.exe")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", script])
        .output()
        .map_err(|e| format!("Failed to launch PowerShell: {e}"))?;
    if output.status.success() { return Ok(()); }
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Err(if !stderr.is_empty() { stderr } else { stdout })
}

fn ensure_uv(runtime: &Path) -> Result<PathBuf, String> {
    let bin = runtime.join("bin");
    fs::create_dir_all(&bin).map_err(|e| format!("Failed to create runtime bin: {e}"))?;
    let uv = detector::uv_path(runtime);
    if uv.exists() { return Ok(uv); }

    if let Ok(output) = Command::new(if cfg!(windows) { "where.exe" } else { "which" }).arg("uv").output() {
        if output.status.success() {
            let found = String::from_utf8_lossy(&output.stdout).lines().next().unwrap_or("").trim();
            if !found.is_empty() && Path::new(found).exists() {
                fs::copy(found, &uv).map_err(|e| format!("Failed to copy uv into Studio runtime: {e}"))?;
                return Ok(uv);
            }
        }
    }

    if !cfg!(windows) {
        return Err("Automatic uv bootstrap is currently implemented for Windows builds.".into());
    }

    let zip = runtime.join("uv-bootstrap.zip");
    let extract = runtime.join("uv-bootstrap");
    let zip_s = zip.to_string_lossy().replace('\'', "''");
    let extract_s = extract.to_string_lossy().replace('\'', "''");
    let uv_s = uv.to_string_lossy().replace('\'', "''");
    let script = format!(
        "$ErrorActionPreference='Stop'; $ProgressPreference='SilentlyContinue'; "
        "Invoke-WebRequest -UseBasicParsing -Uri '{}' -OutFile '{}'; "
        "if(Test-Path '{}'){{Remove-Item -Recurse -Force '{}'}}; "
        "Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force; "
        "$f=Get-ChildItem -LiteralPath '{}' -Recurse -Filter 'uv.exe' | Select-Object -First 1; "
        "if(-not $f){{throw 'uv.exe was not found in the downloaded archive'}}; "
        "Copy-Item -LiteralPath $f.FullName -Destination '{}' -Force; "
        "Remove-Item -Force '{}' -ErrorAction SilentlyContinue; "
        "Remove-Item -Recurse -Force '{}' -ErrorAction SilentlyContinue",
        UV_ZIP_URL, zip_s, extract_s, extract_s, zip_s, extract_s, extract_s, uv_s, zip_s, extract_s
    );
    run_powershell(&script)?;
    if !uv.exists() { return Err(format!("uv bootstrap completed but {} was not created", uv.display())); }
    Ok(uv)
}

fn ensure_engine(root: &Path) -> Result<PathBuf, String> {
    let engine = root.join("wan2gp");
    if engine.join("wgp.py").exists() { return Ok(engine); }
    fs::create_dir_all(root).map_err(|e| format!("Failed to create runtime directory: {e}"))?;

    if !cfg!(windows) {
        return Err("The automatic archive bootstrap is currently implemented for Windows builds.".into());
    }
    let zip = root.join("wan2gp-bootstrap.zip");
    let extract = root.join("wan2gp-bootstrap");
    let zip_s = zip.to_string_lossy().replace('\'', "''");
    let extract_s = extract.to_string_lossy().replace('\'', "''");
    let engine_s = engine.to_string_lossy().replace('\'', "''");
    let script = format!(
        "$ErrorActionPreference='Stop'; $ProgressPreference='SilentlyContinue'; "
        "Invoke-WebRequest -UseBasicParsing -Uri '{}' -OutFile '{}'; "
        "if(Test-Path '{}'){{Remove-Item -Recurse -Force '{}'}}; "
        "Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force; "
        "$src=Get-ChildItem -LiteralPath '{}' -Directory | Select-Object -First 1; "
        "if(-not $src){{throw 'Wan2GP archive has no root directory'}}; "
        "New-Item -ItemType Directory -Force -Path '{}' | Out-Null; "
        "Copy-Item -Path (Join-Path $src.FullName '*') -Destination '{}' -Recurse -Force; "
        "Remove-Item -Force '{}' -ErrorAction SilentlyContinue; "
        "Remove-Item -Recurse -Force '{}' -ErrorAction SilentlyContinue",
        WAN2GP_ZIP_URL, zip_s, extract_s, extract_s, zip_s, extract_s, extract_s, engine_s, engine_s, zip_s, extract_s
    );
    run_powershell(&script)?;
    if !engine.join("wgp.py").exists() { return Err(format!("Wan2GP download completed but {} was not found", engine.join("wgp.py").display())); }
    Ok(engine)
}

pub fn install(root: &Path) -> Result<(), String> {
    fs::create_dir_all(root).map_err(|e| format!("Failed to create runtime directory: {e}"))?;
    let uv = ensure_uv(root)?;
    let engine = ensure_engine(root)?;
    let cache_dir = root.join("cache");
    let python_dir = root.join("python");
    fs::create_dir_all(&cache_dir).map_err(|e| format!("Failed to create runtime cache: {e}"))?;
    fs::create_dir_all(&python_dir).map_err(|e| format!("Failed to create runtime Python cache: {e}"))?;

    // Reuse Wan2GP's own setup.py hardware matrix. The desktop launcher project
    // proved this is the safest way to stay aligned with upstream GPU wheels.
    let uv_path = uv.to_string_lossy().into_owned();
    let cache_path = cache_dir.to_string_lossy().into_owned();
    let py_cache = python_dir.to_string_lossy().into_owned();
    let mut path_env = std::env::var("PATH").unwrap_or_default();
    let uv_bin = uv.parent().unwrap_or(Path::new("."));
    path_env = format!("{};{}", uv_bin.to_string_lossy(), path_env);
    let envs = [
        ("UV_CACHE_DIR", cache_path.as_str()),
        ("UV_PYTHON_INSTALL_DIR", py_cache.as_str()),
        ("PYTHONNOUSERSITE", "1"),
        ("PYTHONUTF8", "1"),
        ("PYTHONUNBUFFERED", "1"),
        ("PATH", path_env.as_str()),
    ];

    run(
        &uv_path,
        &["run", "--python", "3.11.14", "python", "setup.py", "install", "--env", "uv", "--auto"],
        Some(&engine),
        &envs,
    )?;

    let python = detector::find_python(root)
        .ok_or_else(|| "Wan2GP setup completed but the Studio-managed Python environment was not found".to_string())?;
    manifest::write(root, python.to_string_lossy().into_owned())?;
    Ok(())
}
