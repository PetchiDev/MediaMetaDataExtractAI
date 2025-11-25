# Setup Visual Studio environment for Rust builds
# This script initializes the Visual Studio developer environment

$vsPath = & "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath 2>$null

if (-not $vsPath) {
    Write-Host "Error: Visual Studio not found!" -ForegroundColor Red
    Write-Host "Please install Visual Studio 2022 with C++ development tools" -ForegroundColor Yellow
    exit 1
}

Write-Host "Found Visual Studio at: $vsPath" -ForegroundColor Green

# Find vcvars64.bat
$vcvarsPath = Join-Path $vsPath "VC\Auxiliary\Build\vcvars64.bat"

if (-not (Test-Path $vcvarsPath)) {
    Write-Host "Error: vcvars64.bat not found at: $vcvarsPath" -ForegroundColor Red
    exit 1
}

Write-Host "Setting up Visual Studio environment..." -ForegroundColor Cyan

# Call vcvars64.bat and capture environment variables
# Note: This is tricky in PowerShell, so we'll use a workaround
$tempScript = [System.IO.Path]::GetTempFileName() + ".bat"
$tempPs1 = [System.IO.Path]::GetTempFileName() + ".ps1"

# Create a batch file that calls vcvars and then exports env vars
$batchContent = @"
@echo off
call "$vcvarsPath" >nul 2>&1
set > "$tempScript"
"@

Set-Content -Path $tempScript -Value $batchContent

# Execute the batch file
& cmd /c $tempScript

# Read the environment variables from the temp file
if (Test-Path $tempScript) {
    $envVars = Get-Content $tempScript | Where-Object { $_ -match '^([^=]+)=(.*)$' }
    foreach ($line in $envVars) {
        if ($line -match '^([^=]+)=(.*)$') {
            $varName = $matches[1]
            $varValue = $matches[2]
            [Environment]::SetEnvironmentVariable($varName, $varValue, 'Process')
        }
    }
    Remove-Item $tempScript -ErrorAction SilentlyContinue
}

Write-Host "Visual Studio environment configured!" -ForegroundColor Green
Write-Host ""
Write-Host "You can now run: cargo build" -ForegroundColor Cyan

