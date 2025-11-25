# Build script that lets CMake 4.2.0 auto-detect everything
# Since CMake 4.2.0 is installed, it should handle VS 2022 detection

param(
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]$CargoArgs
)

Write-Host "Setting up build with CMake 4.2.0 auto-detection..." -ForegroundColor Cyan

# Verify tools
$cmakeVersion = & cmake --version 2>&1 | Select-Object -First 1
Write-Host "CMake: $cmakeVersion" -ForegroundColor Green

# Find Visual Studio path
$vsPath = & "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath 2>$null

if ($vsPath) {
    Write-Host "Visual Studio found at: $vsPath" -ForegroundColor Green
    
    # Set up Visual Studio environment by calling vcvars64.bat
    $vcvarsPath = Join-Path $vsPath "VC\Auxiliary\Build\vcvars64.bat"
    if (Test-Path $vcvarsPath) {
        Write-Host "Setting up Visual Studio environment..." -ForegroundColor Yellow
        # Call vcvars to set up environment
        & cmd /c "`"$vcvarsPath`" && set" | ForEach-Object {
            if ($_ -match '^([^=]+)=(.*)$') {
                [Environment]::SetEnvironmentVariable($matches[1], $matches[2], 'Process')
            }
        }
    }
}

# Don't force a specific generator - let CMake 4.2.0 choose
# Remove any forced generator setting
Remove-Item Env:\CMAKE_GENERATOR -ErrorAction SilentlyContinue

# Set C11 standard
$env:CFLAGS = "/std:c11"
$env:CFLAGS_x86_64_pc_windows_msvc = "/std:c11"
$env:AWS_LC_SYS_C_STD = "c11"

# Try prebuilt binaries first
$env:AWS_LC_SYS_USE_PKG_CONFIG = "1"

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

