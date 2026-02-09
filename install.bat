@echo off
REM Mockie installer for Windows
REM Compiles and installs Mockie, automatically adding it to PATH

setlocal enabledelayedexpansion

echo.
echo ============================================================
echo   MOCKIE CROSS-PLATFORM INSTALLER - Windows
echo ============================================================
echo.

REM Compile in release
echo 🔨 Compiling Mockie in release mode...
cargo build --release
if errorlevel 1 (
    echo ❌ Error: Compilation failed
    pause
    exit /b 1
)
echo ✅ Compilation completed
echo.

REM Define paths
set SOURCE=target\release\mockie.exe
set APPDATA_PATH=%APPDATA%\mockie
set PROGRAM_FILES_PATH=%ProgramFiles%\mockie

REM Verify that the executable exists
if not exist "%SOURCE%" (
    echo ❌ Error: Executable not found at %SOURCE%
    pause
    exit /b 1
)

REM Try to install in Program Files first
echo ℹ️  Attempting to install in Program Files...
if exist "%ProgramFiles%" (
    mkdir "%PROGRAM_FILES_PATH%" >nul 2>&1
    copy /Y "%SOURCE%" "%PROGRAM_FILES_PATH%\mockie.exe" >nul 2>&1
    if !errorlevel! equ 0 (
        echo ✅ Installed in: %PROGRAM_FILES_PATH%\mockie.exe
        set INSTALL_PATH=%PROGRAM_FILES_PATH%
        goto add_to_path
    )
)

REM If it fails, install in AppData
echo ℹ️  Installing in AppData (alternative)...
mkdir "%APPDATA_PATH%" >nul 2>&1
copy /Y "%SOURCE%" "%APPDATA_PATH%\mockie.exe" >nul 2>&1
if errorlevel 1 (
    echo ❌ Error: Could not install the file
    pause
    exit /b 1
)
echo ✅ Installed in: %APPDATA_PATH%\mockie.exe
set INSTALL_PATH=%APPDATA_PATH%

:add_to_path
echo.
echo ℹ️  Adding to PATH...
setx PATH "!INSTALL_PATH!;!PATH!" >nul 2>&1
if errorlevel 1 (
    echo ⚠️  Warning: Could not automatically add to PATH
    echo    Manually add: !INSTALL_PATH! to your system PATH
) else (
    echo ✅ PATH updated
    echo ⚠️  Open a new terminal for changes to take effect
)

echo.
echo ============================================================
echo ✨ INSTALLATION COMPLETED!
echo ============================================================
echo.
echo Next steps:
echo  1. Open a NEW terminal (PowerShell or CMD)
echo  2. Run: mockie --help
echo  3. Start using Mockie!
echo.
pause
