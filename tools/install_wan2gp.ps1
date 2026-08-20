param(
  [string]$InstallRoot = ""
)
$ErrorActionPreference = "Stop"
$Repo = "https://github.com/deepbeepmeep/Wan2GP.git"

Write-Host "AI Creator Studio - Wan2GP developer installer" -ForegroundColor Cyan

# Keep development installations inside the Studio tree by default.
if ([string]::IsNullOrWhiteSpace($InstallRoot)) {
  $StudioRoot = Split-Path -Parent $PSScriptRoot
  $InstallRoot = Join-Path $StudioRoot "runtime\wan2gp"
}

Write-Host "Install root: $InstallRoot"

if (-not (Get-Command git -ErrorAction SilentlyContinue)) { throw "Git is required only for this developer installer. The packaged Studio installer will not require Git." }
if (-not (Get-Command conda -ErrorAction SilentlyContinue)) { throw "Conda is required only for this developer installer. The packaged Studio installer will use its private runtime." }

$parent = Split-Path -Parent $InstallRoot
New-Item -ItemType Directory -Force -Path $parent | Out-Null

# Migrate the old adjacent checkout when it exists, avoiding a second download.
$oldRoot = Join-Path (Split-Path -Parent $parent) "..\Wan2GP"
$oldRoot = [System.IO.Path]::GetFullPath($oldRoot)
if (-not (Test-Path $InstallRoot) -and (Test-Path (Join-Path $oldRoot "wgp.py"))) {
  Write-Host "Found existing Wan2GP checkout: $oldRoot" -ForegroundColor Yellow
  Write-Host "Migrating it into the Studio runtime..."
  Move-Item -LiteralPath $oldRoot -Destination $InstallRoot
}

if (-not (Test-Path $InstallRoot)) { git clone --depth 1 $Repo $InstallRoot }
elseif (-not (Test-Path (Join-Path $InstallRoot "wgp.py"))) { throw "$InstallRoot exists but is not a Wan2GP checkout." }

$gpu = ""
if (Get-Command nvidia-smi -ErrorAction SilentlyContinue) {
  try { $gpu = (& nvidia-smi --query-gpu=name --format=csv,noheader 2>$null | Select-Object -First 1).Trim() } catch {}
}
Write-Host "Detected GPU: $gpu"

$isLegacy = $gpu -match "GTX 10|GT 10|Quadro P|Quadro M"
if ($isLegacy) {
  Write-Host "Using legacy-compatible Wan2GP environment (Python 3.10.9 / PyTorch 2.7.1 / CUDA 12.8)." -ForegroundColor Yellow
  conda create -n wan2gp python=3.10.9 -y
  conda run -n wan2gp python -m pip install torch==2.7.1 torchvision==0.22.1 torchaudio==2.7.1 --index-url https://download.pytorch.org/whl/test/cu128
} else {
  Write-Host "Using current Wan2GP environment (Python 3.11.14 / PyTorch 2.10 / CUDA 13.0)." -ForegroundColor Green
  conda create -n wan2gp python=3.11.14 -y
  conda run -n wan2gp python -m pip install torch==2.10.0 torchvision==0.25.0 torchaudio==2.10.0 --index-url https://download.pytorch.org/whl/cu130
}

conda run -n wan2gp python -m pip install -r (Join-Path $InstallRoot "requirements.txt")

$envFile = Join-Path $InstallRoot "ai-creator-studio.env.json"
@{
  wan2gp_root = (Resolve-Path $InstallRoot).Path
  conda_env = "wan2gp"
  executable = "wgp.py"
} | ConvertTo-Json | Set-Content -Encoding UTF8 $envFile

Write-Host "Wan2GP installation completed." -ForegroundColor Green
Write-Host "Environment: wan2gp"
Write-Host "Root: $InstallRoot"
Write-Host "Test: conda run -n wan2gp python $InstallRoot\wgp.py"
