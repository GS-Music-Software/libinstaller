@echo off
setlocal enabledelayedexpansion

net session >nul 2>&1
if %errorlevel% neq 0 (
    echo error: this script requires administrator privileges
    echo please right-click and select 'run as administrator'
    pause
    exit /b 1
)

echo gs-music dependency installer Windows AMD64
echo.

call :check_dependencies
if !errorlevel! equ 0 (
    echo you already have all the dependencies installed!
    pause
    exit /b 0
)

call :detect_package_manager
if !errorlevel! neq 0 (
    echo no package manager found, installing chocolatey...
    call :install_chocolatey
    set package_manager=choco
)

echo.
echo installing dependencies with %package_manager%...

if "%package_manager%"=="winget" (
    call :install_with_winget
) else (
    call :install_with_chocolatey
)

echo.
echo refreshing environment...
call :refresh_path

echo.
echo verifying installation...
call :check_dependencies
if !errorlevel! equ 0 (
    echo.
    echo install complete! you can now use gs-music
) else (
    echo warning: some dependencies may not have installed correctly
    echo you may need to restart your terminal or computer
    pause
    exit /b 1
)

pause
exit /b 0

:detect_package_manager
where winget >nul 2>&1
if %errorlevel% equ 0 (
    set package_manager=winget
    echo detected: winget
    exit /b 0
)

where choco >nul 2>&1
if %errorlevel% equ 0 (
    set package_manager=choco
    echo detected: chocolatey
    exit /b 0
)

exit /b 1

:install_chocolatey
powershell -NoProfile -ExecutionPolicy Bypass -Command "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))"
call :refresh_path
exit /b 0

:install_with_winget
winget install --id=mpv.net --exact --silent --accept-source-agreements --accept-package-agreements
winget install --id=yt-dlp.yt-dlp --exact --silent --accept-source-agreements --accept-package-agreements
winget install --id=Gyan.FFmpeg --exact --silent --accept-source-agreements --accept-package-agreements
exit /b 0

:install_with_chocolatey
choco install mpv -y
choco install yt-dlp -y
choco install ffmpeg -y
exit /b 0

:check_dependencies
set missing=
set has_all=1

where mpv >nul 2>&1
if %errorlevel% neq 0 (
    set missing=!missing! mpv
    set has_all=0
)

where yt-dlp >nul 2>&1
if %errorlevel% neq 0 (
    set missing=!missing! yt-dlp
    set has_all=0
)

where ffprobe >nul 2>&1
if %errorlevel% neq 0 (
    set missing=!missing! ffmpeg
    set has_all=0
)

if !has_all! equ 0 (
    echo missing:!missing!
    exit /b 1
)

exit /b 0

:refresh_path
for /f "tokens=2*" %%a in ('reg query "HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment" /v Path') do set machine_path=%%b
for /f "tokens=2*" %%a in ('reg query "HKCU\Environment" /v Path') do set user_path=%%b
set PATH=%machine_path%;%user_path%
exit /b 0
