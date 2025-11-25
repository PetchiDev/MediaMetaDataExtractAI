# Simplified build script that avoids CMake/Visual Studio issues
# Uses prebuilt binaries when possible

param(
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]$CargoArgs
)

Write-Host "Building with simplified configuration..." -ForegroundColor Cyan

# Force use of prebuilt binaries to avoid native compilation
$env:AWS_LC_SYS_USE_PKG_CONFIG = "1"

# Disable assembly optimizations (optional, can help avoid NASM issues)
# $env:AWS_LC_SYS_NO_ASM = "1"

# If no arguments provided, default to "build"
if ($CargoArgs.Count -eq 0) {
    $CargoArgs = @("build")
}

Write-Host ""
Write-Host "Running: cargo $($CargoArgs -join ' ')" -ForegroundColor Yellow
Write-Host ""

# Run cargo
& cargo @CargoArgs

exit $LASTEXITCODE

