$ErrorActionPreference = "Stop"
$studio = Split-Path -Parent $PSScriptRoot
$runtime = Join-Path $studio "runtime"
$engine = Join-Path $runtime "wan2gp"
$python = Join-Path $runtime "python\python.exe"

Write-Host "AI Creator Studio runtime check" -ForegroundColor Cyan
Write-Host "Runtime: $runtime"
Write-Host "Wan2GP : $engine"
Write-Host "Python : $python"

if (-not (Test-Path (Join-Path $engine "wgp.py"))) {
  throw "Wan2GP is not installed at $engine"
}

if (Test-Path $python) {
  & $python --version
} else {
  Write-Host "Private Python runtime is not installed yet." -ForegroundColor Yellow
  Write-Host "The developer bootstrap currently uses the existing Python/Conda environment; the packaged installer will provision a private runtime." -ForegroundColor Yellow
}

Write-Host "Wan2GP OK: $((Test-Path (Join-Path $engine "wgp.py")))" -ForegroundColor Green
