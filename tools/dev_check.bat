@echo off
setlocal
cd /d F:\XTTS\AI-Creator-Studio\app
if not exist "src-tauri\Cargo.toml" (
  echo [ERROR] Cargo project not found.
  pause
  exit /b 1
)
echo ================================================
echo AI Creator Studio - Development Check
 echo ================================================
echo.
echo [1/2] Checking Rust/Tauri...
cargo check --manifest-path src-tauri\Cargo.toml
if errorlevel 1 goto :failed
echo.
echo [2/2] Checking frontend build...
cd /d F:\XTTS\AI-Creator-Studio\app\frontend
call npm run build
if errorlevel 1 goto :failed
echo.
echo ================================================
echo ALL CHECKS PASSED
echo ================================================
pause
exit /b 0
:failed
echo.
echo ================================================
echo CHECK FAILED - see error above
echo ================================================
pause
exit /b 1
