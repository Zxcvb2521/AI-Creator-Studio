@echo off
setlocal
cd /d "%~dp0.."

echo ========================================
echo AI Creator Studio - Wan2GP installer
echo ========================================
echo.
echo Installing into the Studio runtime folder:
echo %CD%\runtime\wan2gp
echo.

powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0install_wan2gp.ps1" %*
set CODE=%ERRORLEVEL%

echo.
if not "%CODE%"=="0" (
  echo Wan2GP installation failed. Exit code: %CODE%
) else (
  echo Wan2GP installation completed.
)
endlocal & exit /b %CODE%
