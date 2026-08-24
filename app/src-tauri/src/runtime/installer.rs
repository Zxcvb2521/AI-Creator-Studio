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

fn install_spec(uv: &Path, python: &Path, args: &[&str], root: &Path) -> Result<(), String> {
    let mut cmd = Command::new(uv);
    cmd.args(["pip", "install", "--python"]);
    cmd.arg(python);
    cmd.args(args);
    cmd.current_dir(root);
    cmd.env("PYTHONNOUSERSITE", "1");
    cmd.env("PYTHONUTF8", "1");
    cmd.env("PYTHONUNBUFFERED", "1");
    let output = cmd.output().map_err(|e| format!("Failed to launch uv kernel install: {e}"))?;
    if output.status.success() { return Ok(()); }
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Err(if !stderr.is_empty() { stderr } else { stdout })
}

fn sync_nvidia_kernels(uv: &Path, python: &Path, gpu_name: &str, root: &Path) -> Result<(), String> {
    if !cfg!(windows) { return Ok(()); }
    let gpu = gpu_name.to_ascii_uppercase();
    if !gpu.contains("NVIDIA") && !gpu.contains("GEFORCE") && !gpu.contains("RTX") && !gpu.contains("GTX") { return Ok(()); }

    // Ported from the maintained Wan2GP Desktop launcher's hardware-aware kernel
    // matrix. This runs only during the explicit first-run/install step, not on
    // every generation, and uses the Studio-managed Python so user-site packages
    // cannot poison the runtime (the source of the old flash_attn DLL failure).
    let mut wheels: Vec<String> = Vec::new();
    if gpu.contains("50") {
        wheels.push("https://github.com/woct0rdho/SageAttention/releases/download/v2.2.0-windows.post6/sageattention-2.2.0+cu130torch2.10.0andhigher.post6-cp310-abi3-win_amd64.whl".into());
        wheels.push("https://github.com/woct0rdho/SpargeAttn/releases/download/v0.1.0-windows.post4/spas_sage_attn-0.1.0%2Bcu130torch2.9.0andhigher.post4-cp39-abi3-win_amd64.whl".into());
        wheels.push("https://github.com/deepbeepmeep/kernels/releases/download/Flash2/flash_attn-2.8.3-cp311-cp311-win_amd64.whl".into());
        wheels.push("https://github.com/nunchaku-ai/nunchaku/releases/download/v1.2.1/nunchaku-1.2.1+cu13.0torch2.10-cp311-cp311-win_amd64.whl".into());
        wheels.push("https://github.com/deepbeepmeep/kernels/releases/download/Light2xv/lightx2v_kernel-0.0.2+torch2.10.0-cp311-abi3-win_amd64.whl".into());
        wheels.push("https://github.com/deepbeepmeep/kernels/releases/download/GGUF_Kernels/llamacpp_gguf_cuda-1.0.11+torch210cu130py311-cp311-cp311-win_amd64.whl".into());
        install_spec(uv, python, &["-U", "triton-windows"], root)?;
    } else if gpu.contains("40") || gpu.contains("30") {
        wheels.push("https://github.com/woct0rdho/SageAttention/releases/download/v2.2.0-windows.post6/sageattention-2.2.0+cu130torch2.10.0andhigher.post6-cp310-abi3-win_amd64.whl".into());
        wheels.push("https://github.com/woct0rdho/SpargeAttn/releases/download/v0.1.0-windows.post4/spas_sage_attn-0.1.0%2Bcu130torch2.9.0andhigher.post4-cp39-abi3-win_amd64.whl".into());
        wheels.push("https://github.com/deepbeepmeep/kernels/releases/download/Flash2/flash_attn-2.8.3-cp311-cp311-win_amd64.whl".into());
        wheels.push("https://github.com/nunchaku-ai/nunchaku/releases/download/v1.2.1/nunchaku-1.2.1+cu13.0torch2.10-cp311-cp311-win_amd64.whl".into());
        wheels.push("https://github.com/deepbeepmeep/kernels/releases/download/GGUF_Kernels/llamacpp_gguf_cuda-1.0.11+torch210cu130py311-cp311-cp311-win_amd64.whl".into());
        install_spec(uv, python, &["-U", "triton-windows"], root)?;
    } else if gpu.contains("20") || gpu.contains("QUADRO") {
        wheels.push("sageattention==1.0.6".into());
        wheels.push("https://github.com/deepbeepmeep/kernels/releases/download/Flash2/flash_attn-2.8.3-cp311-cp311-win_amd64.whl".into());
        wheels.push("https://github.com/nunchaku-ai/nunchaku/releases/download/v1.2.1/nunchaku-1.2.1+cu13.0torch2.10-cp311-cp311-win_amd64.whl".into());
        wheels.push("https://github.com/deepbeepmeep/kernels/releases/download/GGUF_Kernels/llamacpp_gguf_cuda-1.0.11+torch210cu130py311-cp311-cp311-win_amd64.whl".into());
        install_spec(uv, python, &["-U", "triton-windows"], root)?;
    } else {
        return Ok(());
    }

    for wheel in wheels {
        install_spec(uv, python, &["-U", &wheel], root)?;
    }
    Ok(())
}

pub fn install(root: &Path) -> Result<(), String> {
    fs::create_dir_all(root).map_err(|e| format!("Failed to create runtime directory: {e}"))?;
    let uv = ensure_uv(root)?;
    let engine = ensure_engine(root)?;
    let cache_dir = root.join("cache");
    let python_dir = root.join("python");
    fs::create_dir_all(&cache_dir).map_err(|e| format!("Failed to create runtime cache: {e}"))?;
    fs::create_dir_all(&python_dir).map_err(|e| format!("Failed to create runtime Python cache: {e}"))?;

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

    if let Some(gpu) = detector::nvidia_gpu() {
        sync_nvidia_kernels(&uv, &python, &gpu, &engine)?;
    }

    manifest::write(root, python.to_string_lossy().into_owned())?;
    Ok(())
}
