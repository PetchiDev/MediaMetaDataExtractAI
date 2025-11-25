# Fix for Visual Studio 2022 build issues with aws-lc-sys
# This script sets environment variables to work around cmake crate compatibility issues

Write-Host "Fixing Visual Studio 2022 build compatibility..." -ForegroundColor Cyan
Write-Host ""

# Detect Visual Studio 2022
$vs2022Path = "${env:ProgramFiles}\Microsoft Visual Studio\2022\Community"
$vs2022BuildTools = "${env:ProgramFiles}\Microsoft Visual Studio\2022\BuildTools"

if (Test-Path $vs2022Path) {
    $vsPath = $vs2022Path
    Write-Host "[OK] Found Visual Studio 2022 Community" -ForegroundColor Green
} elseif (Test-Path $vs2022BuildTools) {
    $vsPath = $vs2022BuildTools
    Write-Host "[OK] Found Visual Studio 2022 Build Tools" -ForegroundColor Green
} else {
    Write-Host "[WARN] Visual Studio 2022 not found in standard locations" -ForegroundColor Yellow
    Write-Host "       Will try to detect automatically..." -ForegroundColor Gray
    $vsPath = $null
}

# Set CMAKE_GENERATOR to explicitly use Visual Studio 2022
# This tells cmake to use VS 2022 even if the crate doesn't recognize it
Write-Host ""
Write-Host "Setting CMAKE_GENERATOR environment variable..." -ForegroundColor Yellow
$env:CMAKE_GENERATOR = "Visual Studio 17 2022"
Write-Host "   CMAKE_GENERATOR = $env:CMAKE_GENERATOR" -ForegroundColor Gray

# Set C11 standard for MSVC compiler
Write-Host ""
Write-Host "Setting C11 standard for MSVC compiler..." -ForegroundColor Yellow
$env:CFLAGS = "/std:c11"
$env:CFLAGS_x86_64_pc_windows_msvc = "/std:c11"
Write-Host "   CFLAGS = $env:CFLAGS" -ForegroundColor Gray

# Set AWS_LC_SYS to use C11
$env:AWS_LC_SYS_C_STD = "c11"
Write-Host "   AWS_LC_SYS_C_STD = $env:AWS_LC_SYS_C_STD" -ForegroundColor Gray

# Try to find and set up Visual Studio Developer Command Prompt environment
if ($vsPath) {
    $vcvarsPath = Join-Path $vsPath "VC\Auxiliary\Build\vcvars64.bat"
    if (Test-Path $vcvarsPath) {
        Write-Host ""
        Write-Host "[OK] Found vcvars64.bat at: $vcvarsPath" -ForegroundColor Green
        Write-Host "     Note: You may need to run this script before cargo build" -ForegroundColor Gray
    }
}

# Alternative: Use prebuilt binaries to avoid compilation
Write-Host ""
Write-Host "Alternative: Using prebuilt binaries (faster, no compilation needed)..." -ForegroundColor Cyan
$env:AWS_LC_SYS_USE_PKG_CONFIG = "1"
Write-Host "   AWS_LC_SYS_USE_PKG_CONFIG = $env:AWS_LC_SYS_USE_PKG_CONFIG" -ForegroundColor Gray

# Disable assembly optimizations if NASM is still causing issues
# $env:AWS_LC_SYS_NO_ASM = "1"

Write-Host ""
Write-Host "[OK] Environment variables set!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "   1. Clean the build cache:" -ForegroundColor White
Write-Host "      cargo clean" -ForegroundColor Yellow
Write-Host ""
Write-Host "   2. Build the project:" -ForegroundColor White
Write-Host "      cargo build" -ForegroundColor Yellow
Write-Host ""
Write-Host "Note: These environment variables are set for this PowerShell session only." -ForegroundColor Gray
Write-Host "     If you open a new terminal, run this script again or set them manually." -ForegroundColor Gray
Write-Host ""

# Create a .env file for persistent environment variables
$envFile = ".env.build"
$envContent = @"
# Visual Studio 2022 build compatibility settings
CMAKE_GENERATOR=Visual Studio 17 2022
AWS_LC_SYS_C_STD=c11
AWS_LC_SYS_USE_PKG_CONFIG=1
CFLAGS=/std:c11
CFLAGS_x86_64_pc_windows_msvc=/std:c11
"@

Set-Content -Path $envFile -Value $envContent -Encoding UTF8
Write-Host "[OK] Created $envFile with build settings" -ForegroundColor Green
Write-Host "     Note: Use the cargo-build-vs2022.ps1 wrapper script for automatic setup" -ForegroundColor Gray
Write-Host ""
