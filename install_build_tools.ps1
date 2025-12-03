# PowerShell script to install Visual Studio Build Tools
# Run as Administrator: .\install_build_tools.ps1

Write-Host "🔧 Installing Visual Studio Build Tools for Rust..." -ForegroundColor Cyan
Write-Host ""

# Check if already installed
$vsPath = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path $vsPath) {
    $installed = & $vsPath -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    if ($installed) {
        Write-Host "✅ Visual Studio Build Tools already installed!" -ForegroundColor Green
        Write-Host "Location: $installed" -ForegroundColor Gray
        exit 0
    }
}

# Download URL
$url = "https://aka.ms/vs/17/release/vs_buildtools.exe"
$output = "$env:TEMP\vs_buildtools.exe"

Write-Host "📥 Downloading Visual Studio Build Tools..." -ForegroundColor Yellow
Write-Host "   This may take a few minutes..." -ForegroundColor Gray

try {
    Invoke-WebRequest -Uri $url -OutFile $output -UseBasicParsing
    Write-Host "✅ Download complete!" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to download. Please download manually from:" -ForegroundColor Red
    Write-Host "   https://visualstudio.microsoft.com/downloads/" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "🚀 Starting installation..." -ForegroundColor Yellow
Write-Host "   This will install 'Desktop development with C++' workload" -ForegroundColor Gray
Write-Host "   Installation may take 10-20 minutes..." -ForegroundColor Gray
Write-Host ""

# Install with C++ workload
$installArgs = @(
    "--quiet",
    "--wait",
    "--norestart",
    "--nocache",
    "--add", "Microsoft.VisualStudio.Workload.VCTools",
    "--add", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
    "--add", "Microsoft.VisualStudio.Component.Windows10SDK.19041"
)

try {
    $process = Start-Process -FilePath $output -ArgumentList $installArgs -Wait -PassThru -NoNewWindow
    
    if ($process.ExitCode -eq 0) {
        Write-Host ""
        Write-Host "✅ Installation completed successfully!" -ForegroundColor Green
        Write-Host ""
        Write-Host "🔄 Please restart your PowerShell/terminal and run:" -ForegroundColor Cyan
        Write-Host "   cargo run" -ForegroundColor White
    } else {
        Write-Host ""
        Write-Host "⚠️  Installation completed with exit code: $($process.ExitCode)" -ForegroundColor Yellow
        Write-Host "   You may need to restart PowerShell and try again" -ForegroundColor Yellow
    }
} catch {
    Write-Host ""
    Write-Host "❌ Installation failed. Please install manually:" -ForegroundColor Red
    Write-Host "   1. Run: $output" -ForegroundColor Yellow
    Write-Host "   2. Select 'Desktop development with C++'" -ForegroundColor Yellow
    Write-Host "   3. Click Install" -ForegroundColor Yellow
}

# Clean up
Remove-Item $output -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "📝 Note: After installation, restart PowerShell for PATH to update" -ForegroundColor Cyan











