# Build script that uses the installed CMake 4.2.0 properly
# CMake 4.2.0 should be able to detect Visual Studio 2022 automatically

param(
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]$CargoArgs
)

Write-Host "Using installed CMake 4.2.0 for build..." -ForegroundColor Cyan

# Verify CMake is available
$cmakeVersion = & cmake --version 2>&1 | Select-Object -First 1
Write-Host "CMake: $cmakeVersion" -ForegroundColor Green

# Don't set CMAKE_GENERATOR - let CMake 4.2.0 auto-detect Visual Studio
# CMake 4.2.0 should be able to find VS 2022 automatically
$env:CMAKE_GENERATOR = ""

# Set C11 standard for MSVC compiler
$env:CFLAGS = "/std:c11"
$env:CFLAGS_x86_64_pc_windows_msvc = "/std:c11"
$env:AWS_LC_SYS_C_STD = "c11"

# Try to use prebuilt binaries first (faster)
$env:AWS_LC_SYS_USE_PKG_CONFIG = "1"

# If no arguments provided, default to "build"
if ($CargoArgs.Count -eq 0) {
    $CargoArgs = @("build")
}

Write-Host ""
Write-Host "Running: cargo $($CargoArgs -join ' ')" -ForegroundColor Yellow
Write-Host "CMake will auto-detect Visual Studio 2022" -ForegroundColor Gray
Write-Host ""

# Run cargo with the provided arguments
& cargo @CargoArgs

# Exit with cargo's exit code
exit $LASTEXITCODE

