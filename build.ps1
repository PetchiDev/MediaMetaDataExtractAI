# Build script that sets up environment for aws-lc-sys compilation
# This works around the CMake/Visual Studio 2022 compatibility issue

Write-Host "Setting up build environment..." -ForegroundColor Cyan

# Set CMake generator for Visual Studio 2022
$env:CMAKE_GENERATOR = "Visual Studio 17 2022"
$env:CMAKE_GENERATOR_PLATFORM = "x64"

# Set C11 standard for MSVC compiler
$env:CFLAGS = "/std:c11"
$env:CFLAGS_x86_64_pc_windows_msvc = "/std:c11"
$env:AWS_LC_SYS_C_STD = "c11"

# Try to use prebuilt binaries first (if available)
$env:AWS_LC_SYS_USE_PKG_CONFIG = "1"

Write-Host "Environment configured. Building..." -ForegroundColor Green
Write-Host ""

# Build
cargo build

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "Build successful!" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "Build failed. Error code: $LASTEXITCODE" -ForegroundColor Red
    Write-Host ""
    Write-Host "If build still fails, try:" -ForegroundColor Yellow
    Write-Host "1. Open 'Developer Command Prompt for VS 2022'" -ForegroundColor White
    Write-Host "2. Navigate to this directory" -ForegroundColor White
    Write-Host "3. Run: cargo build" -ForegroundColor White
}

