# Wrapper script to build with Visual Studio 2022 compatibility settings
# Usage: .\cargo-build-vs2022.ps1 [cargo-args...]
# This script uses prebuilt binaries to avoid native compilation issues

param(
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]$CargoArgs
)

Write-Host "Setting up build environment (using prebuilt binaries to avoid compilation)..." -ForegroundColor Cyan

# Use prebuilt binaries - this avoids all CMake/Visual Studio compilation issues
$env:AWS_LC_SYS_USE_PKG_CONFIG = "1"

# Try to set CMAKE_GENERATOR if we need to fall back to building
# But prefer prebuilt binaries
$vsPath = & "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath 2>$null
if ($vsPath) {
    # Set generator for fallback, but we prefer prebuilt
    $env:CMAKE_GENERATOR = "Visual Studio 17 2022"
    
    # Set C11 standard for MSVC (if building from source)
    $env:CFLAGS = "/std:c11"
    $env:CFLAGS_x86_64_pc_windows_msvc = "/std:c11"
    $env:AWS_LC_SYS_C_STD = "c11"
} else {
    Write-Host "Warning: Visual Studio not found. Using prebuilt binaries only." -ForegroundColor Yellow
}

# If no arguments provided, default to "build"
if ($CargoArgs.Count -eq 0) {
    $CargoArgs = @("build")
}

Write-Host ""
Write-Host "Running: cargo $($CargoArgs -join ' ')" -ForegroundColor Yellow
Write-Host "Using prebuilt AWS-LC binaries (no native compilation needed)" -ForegroundColor Gray
Write-Host ""

# Run cargo with the provided arguments
& cargo @CargoArgs

# Exit with cargo's exit code
exit $LASTEXITCODE
