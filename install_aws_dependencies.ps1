# PowerShell script to install CMake and NASM for aws-lc-sys
# Run as Administrator: .\install_aws_dependencies.ps1

Write-Host "🔧 Installing CMake and NASM for AWS SDK dependencies..." -ForegroundColor Cyan
Write-Host ""

# Function to check if a command exists
function Test-Command {
    param($Command)
    $null = Get-Command $Command -ErrorAction SilentlyContinue
    return $?
}

# Check for CMake
Write-Host "📦 Checking for CMake..." -ForegroundColor Yellow
if (Test-Command "cmake") {
    $cmakeVersion = cmake --version | Select-Object -First 1
    Write-Host "✅ CMake already installed: $cmakeVersion" -ForegroundColor Green
} else {
    Write-Host "❌ CMake not found. Installing..." -ForegroundColor Red
    
    # Try winget first
    if (Test-Command "winget") {
        Write-Host "📥 Installing CMake via winget..." -ForegroundColor Yellow
        winget install Kitware.CMake --silent --accept-package-agreements --accept-source-agreements
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ CMake installed successfully!" -ForegroundColor Green
        } else {
            Write-Host "⚠️  Winget installation failed. Trying Chocolatey..." -ForegroundColor Yellow
        }
    }
    
    # Try Chocolatey if winget failed or not available
    if (-not (Test-Command "cmake") -and (Test-Command "choco")) {
        Write-Host "📥 Installing CMake via Chocolatey..." -ForegroundColor Yellow
        choco install cmake -y
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ CMake installed successfully!" -ForegroundColor Green
        }
    }
    
    # Manual download if both package managers failed
    if (-not (Test-Command "cmake")) {
        Write-Host "⚠️  Package manager installation failed." -ForegroundColor Yellow
        Write-Host "📥 Please download CMake manually:" -ForegroundColor Cyan
        Write-Host "   1. Visit: https://cmake.org/download/" -ForegroundColor White
        Write-Host "   2. Download Windows x64 Installer" -ForegroundColor White
        Write-Host "   3. Run installer and select 'Add CMake to system PATH'" -ForegroundColor White
        Write-Host ""
    }
}

# Check for NASM
Write-Host ""
Write-Host "📦 Checking for NASM..." -ForegroundColor Yellow
if (Test-Command "nasm") {
    $nasmVersion = nasm -v | Select-Object -First 1
    Write-Host "✅ NASM already installed: $nasmVersion" -ForegroundColor Green
} else {
    Write-Host "❌ NASM not found. Installing..." -ForegroundColor Red
    
    # Try winget first
    if (Test-Command "winget") {
        Write-Host "📥 Installing NASM via winget..." -ForegroundColor Yellow
        winget install NASM.NASM --silent --accept-package-agreements --accept-source-agreements
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ NASM installed successfully!" -ForegroundColor Green
        } else {
            Write-Host "⚠️  Winget installation failed. Trying Chocolatey..." -ForegroundColor Yellow
        }
    }
    
    # Try Chocolatey if winget failed or not available
    if (-not (Test-Command "nasm") -and (Test-Command "choco")) {
        Write-Host "📥 Installing NASM via Chocolatey..." -ForegroundColor Yellow
        choco install nasm -y
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ NASM installed successfully!" -ForegroundColor Green
        }
    }
    
    # Manual download if both package managers failed
    if (-not (Test-Command "nasm")) {
        Write-Host "⚠️  Package manager installation failed." -ForegroundColor Yellow
        Write-Host "📥 Please download NASM manually:" -ForegroundColor Cyan
        Write-Host "   1. Visit: https://www.nasm.us/pub/nasm/releasebuilds/" -ForegroundColor White
        Write-Host "   2. Download latest win64 installer (e.g. nasm-X.XX-win64.zip)" -ForegroundColor White
        Write-Host "   3. Extract and add to PATH, or install to C:\Program Files\NASM" -ForegroundColor White
        Write-Host ""
    }
}

Write-Host ""
Write-Host "🔄 Summary:" -ForegroundColor Cyan
if (Test-Command "cmake") {
    Write-Host "   ✅ CMake: Installed" -ForegroundColor Green
} else {
    Write-Host "   ❌ CMake: Not found - Please install manually" -ForegroundColor Red
}

if (Test-Command "nasm") {
    Write-Host "   ✅ NASM: Installed" -ForegroundColor Green
} else {
    Write-Host "   ❌ NASM: Not found - Please install manually" -ForegroundColor Red
}

Write-Host ""
if ((Test-Command "cmake") -and (Test-Command "nasm")) {
    Write-Host "✅ All dependencies installed! Restart PowerShell and run:" -ForegroundColor Green
    Write-Host "   cargo build" -ForegroundColor White
} else {
    Write-Host "⚠️  Some dependencies are missing. Please install them and restart PowerShell." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "💡 Alternative: Use pre-built binaries by setting environment variable:" -ForegroundColor Cyan
    Write-Host "   `$env:AWS_LC_SYS_USE_PKG_CONFIG = '1'" -ForegroundColor White
    Write-Host "   cargo build" -ForegroundColor White
}

