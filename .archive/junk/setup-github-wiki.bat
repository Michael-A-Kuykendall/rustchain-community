@echo off
echo 🔧 RustChain GitHub Wiki Setup
echo ================================

echo.
echo This script will set up the GitHub Wiki integration for RustChain.
echo.

:: Check if we're in a git repository
git status >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Error: Not in a git repository
    echo Please run this script from the rustchain-community directory
    pause
    exit /b 1
)

:: Get the current repository URL
for /f "tokens=*" %%i in ('git remote get-url origin') do set REPO_URL=%%i
echo 📂 Current repository: %REPO_URL%

:: Extract organization and repo name
echo %REPO_URL% | findstr "github.com" >nul
if %errorlevel% neq 0 (
    echo ❌ Error: This doesn't appear to be a GitHub repository
    pause
    exit /b 1
)

:: Create wiki directory
echo.
echo 📁 Setting up wiki directory...
if not exist "wiki" (
    mkdir wiki
    echo ✅ Created wiki directory
) else (
    echo ℹ️  Wiki directory already exists
)

cd wiki

:: Check if wiki is already cloned
if exist ".git" (
    echo ℹ️  Wiki repository already cloned, pulling latest changes...
    git pull
) else (
    echo 📥 Cloning wiki repository...
    
    :: Construct wiki URL
    set WIKI_URL=%REPO_URL%.wiki
    echo Wiki URL: %WIKI_URL%
    
    git clone %WIKI_URL% .
    if %errorlevel% neq 0 (
        echo.
        echo ⚠️  Wiki repository doesn't exist yet. Creating initial wiki structure...
        
        :: Initialize wiki with basic structure
        git init
        
        :: Create Home page
        echo # RustChain Wiki > Home.md
        echo. >> Home.md
        echo Welcome to the **RustChain** documentation wiki. >> Home.md
        echo. >> Home.md
        echo This wiki serves as the single source of truth for all RustChain documentation. >> Home.md
        echo. >> Home.md
        echo ## Categories >> Home.md
        echo. >> Home.md
        echo - [Quick Start](Quick-Start-Guide) >> Home.md
        echo - [Installation](Installation-Guide) >> Home.md
        echo - [API Reference](API-Reference) >> Home.md
        echo - [Architecture](Architecture-Overview) >> Home.md
        echo - [Security](Security-and-Compliance) >> Home.md
        
        :: Create initial pages
        echo # Installation Guide > Installation-Guide.md
        echo. >> Installation-Guide.md
        echo ---  >> Installation-Guide.md
        echo title: Installation Guide >> Installation-Guide.md
        echo category: quickstart >> Installation-Guide.md
        echo tags: [installation, setup, getting-started] >> Installation-Guide.md
        echo lastUpdated: 2024-12-16 >> Installation-Guide.md
        echo ---  >> Installation-Guide.md
        echo. >> Installation-Guide.md
        echo ## Install RustChain >> Installation-Guide.md
        echo. >> Installation-Guide.md
        echo ```bash >> Installation-Guide.md
        echo cargo install rustchain --features all >> Installation-Guide.md
        echo ``` >> Installation-Guide.md
        echo. >> Installation-Guide.md
        echo ## Verify Installation >> Installation-Guide.md
        echo. >> Installation-Guide.md
        echo ```bash >> Installation-Guide.md
        echo rustchain --version >> Installation-Guide.md
        echo ``` >> Installation-Guide.md
        
        echo # Architecture Overview > Architecture-Overview.md
        echo. >> Architecture-Overview.md
        echo ---  >> Architecture-Overview.md
        echo title: Architecture Overview >> Architecture-Overview.md
        echo category: development >> Architecture-Overview.md
        echo tags: [architecture, technical, design] >> Architecture-Overview.md
        echo lastUpdated: 2024-12-16 >> Architecture-Overview.md
        echo ---  >> Architecture-Overview.md
        echo. >> Architecture-Overview.md
        echo RustChain uses a **mission-based architecture** for AI task execution. >> Architecture-Overview.md
        echo. >> Architecture-Overview.md
        echo ## Core Components >> Architecture-Overview.md
        echo. >> Architecture-Overview.md
        echo - **Mission Engine**: DAG processing with dependency resolution >> Architecture-Overview.md
        echo - **Agent System**: ReAct pattern implementation >> Architecture-Overview.md
        echo - **Tool Framework**: Extensible tool ecosystem >> Architecture-Overview.md
        echo - **Security Layer**: Multi-layer validation and audit trails >> Architecture-Overview.md
        
        echo # Security and Compliance > Security-and-Compliance.md
        echo. >> Security-and-Compliance.md
        echo ---  >> Security-and-Compliance.md
        echo title: Security and Compliance >> Security-and-Compliance.md
        echo category: enterprise >> Security-and-Compliance.md
        echo tags: [security, compliance, enterprise, sox, gdpr] >> Security-and-Compliance.md
        echo lastUpdated: 2024-12-16 >> Security-and-Compliance.md
        echo ---  >> Security-and-Compliance.md
        echo. >> Security-and-Compliance.md
        echo RustChain provides enterprise-grade security for mission-critical deployments. >> Security-and-Compliance.md
        echo. >> Security-and-Compliance.md
        echo ## Compliance Standards >> Security-and-Compliance.md
        echo. >> Security-and-Compliance.md
        echo - ✅ **SOX**: Sarbanes-Oxley compliance for financial data >> Security-and-Compliance.md
        echo - ✅ **GDPR**: Data protection and privacy controls >> Security-and-Compliance.md
        echo - ✅ **SOC2**: Security and availability controls >> Security-and-Compliance.md
        echo - ✅ **ISO 27001**: Information security management >> Security-and-Compliance.md
        
        :: Stage and commit initial files
        git add .
        git commit -m "Initial wiki structure with foundational pages

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
        
        echo ✅ Created initial wiki structure
        echo.
        echo ⚠️  You need to push this to GitHub to create the wiki repository:
        echo    git remote add origin %WIKI_URL%
        echo    git push -u origin main
        echo.
        echo After pushing, the wiki will be available at:
        echo https://github.com/%REPO_URL:.git=%/wiki
    ) else (
        echo ✅ Wiki repository cloned successfully
    )
)

cd ..

echo.
echo 🎉 Wiki setup complete!
echo.
echo Next steps:
echo 1. Edit wiki pages in the 'wiki' directory
echo 2. Commit and push changes to sync with GitHub
echo 3. The website will automatically fetch content from GitHub
echo.
echo Wiki directory: %CD%\wiki
echo.
pause