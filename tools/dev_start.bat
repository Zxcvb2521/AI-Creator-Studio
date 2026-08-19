@echo off
setlocal
cd /d F:\XTTS\AI-Creator-Studio\app
if not exist "src-tauri\Cargo.toml" (
  echo [ERROR] AI Creator Studio project not found.
  pause
  exit /b 1
)
echo ================================================
echo AI Creator Studio - Development Launcher
echo ================================================
echo.
echo Starting Tauri development application...
echo Close this window or press Ctrl+C to stop it.
echo.
cargo tauri dev
set CODE=%ERRORLEVEL%
echo.
echo Application exited with code %CODE%.
pause
exit /b %CODE%
