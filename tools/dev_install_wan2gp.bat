@echo off
setlocal
cd /d F:\XTTS\AI-Creator-Studio
if not exist "tools\install_wan2gp.ps1" (
  echo [ERROR] tools\install_wan2gp.ps1 not found.
  pause
  exit /b 1
)
echo ================================================
echo AI Creator Studio - Wan2GP Installer
 echo ================================================
echo.
echo This will prepare the external Wan2GP engine.
echo Run this from an elevated PowerShell if the installer requests it.
echo.
powershell -NoProfile -ExecutionPolicy Bypass -File "%CD%\tools\install_wan2gp.ps1"
set CODE=%ERRORLEVEL%
echo.
echo Installer exited with code %CODE%.
pause
exit /b %CODE%
