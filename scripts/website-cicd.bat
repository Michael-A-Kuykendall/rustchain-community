@echo off
REM RustChain Website CI/CD Trigger Script (Windows)
REM This script should be called whenever you want to update the website

echo 🚀 Starting RustChain Website CI/CD Pipeline...

REM Get current RustChain version from Cargo.toml
for /f "tokens=3 delims= " %%a in ('findstr "^version = " Cargo.toml') do (
    set RUSTCHAIN_VERSION=%%a
)
set RUSTCHAIN_VERSION=%RUSTCHAIN_VERSION:"=%

REM Get commit hash
for /f %%a in ('git rev-parse --short HEAD') do set COMMIT_HASH=%%a

REM Get current timestamp
for /f "tokens=2-4 delims=/ " %%a in ('date /t') do set DATE=%%c-%%a-%%b
for /f "tokens=1-2 delims=: " %%a in ('time /t') do set TIME=%%a:%%b
set CURRENT_TIMESTAMP=%DATE% %TIME% UTC

echo 📦 RustChain Version: %RUSTCHAIN_VERSION%
echo 📝 Commit Hash: %COMMIT_HASH%
echo 🕐 Timestamp: %CURRENT_TIMESTAMP%

REM Run the website update mission
echo 🎯 Executing website update mission...
.\target\debug\rustchain.exe run missions\website_update.yaml --variable "rustchain_version=%RUSTCHAIN_VERSION%" --variable "commit_hash=%COMMIT_HASH%" --variable "current_timestamp=%CURRENT_TIMESTAMP%"

echo ✅ Website CI/CD Pipeline Completed!
echo 🌐 Your website should now be updated and deployed automatically.
echo.
echo To manually check the website locally:
echo   cd website ^&^& npm run dev
echo   Open: http://localhost:8080

pause