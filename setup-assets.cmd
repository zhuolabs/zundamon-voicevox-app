@echo off
setlocal
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0setup-assets.ps1" %*
set "rc=%errorlevel%"
if not "%rc%"=="0" echo Setup failed. Review the messages above.
pause
exit /b %rc%
