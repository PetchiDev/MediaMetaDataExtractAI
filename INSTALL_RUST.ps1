# PowerShell script to install Rust on Windows
# Run this script: .\INSTALL_RUST.ps1

Write-Host "🦀 Installing Rust on Windows..." -ForegroundColor Cyan

# Check if Rust is already installed
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "✅ Rust is already installed!" -ForegroundColor Green
    cargo --version
    rustc --version
    exit 0
}

# Download rustup-init
Write-Host "📥 Downloading rustup installer..." -ForegroundColor Yellow
$rustupUrl = "https://win.rustup.rs/x86_64"
$rustupFile = "$env:TEMP\rustup-init.exe"

try {
    Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupFile -UseBasicParsing
    Write-Host "✅ Download complete!" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to download rustup. Please download manually from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Run installer
Write-Host "🚀 Running Rust installer..." -ForegroundColor Yellow
Write-Host "   (This will install Rust with default settings)" -ForegroundColor Gray

Start-Process -FilePath $rustupFile -ArgumentList "-y" -Wait -NoNewWindow

# Clean up
Remove-Item $rustupFile -ErrorAction SilentlyContinue

# Check if installation was successful
Write-Host "`n🔄 Please restart your PowerShell/terminal and run:" -ForegroundColor Cyan
Write-Host "   cargo --version" -ForegroundColor White
Write-Host "`nThen navigate to your project and run:" -ForegroundColor Cyan
Write-Host "   cd C:\Users\Petchiappan.P\my_api" -ForegroundColor White
Write-Host "   cargo run" -ForegroundColor White

Write-Host "`n✅ Rust installation completed!" -ForegroundColor Green
Write-Host "   (You may need to restart your terminal for PATH to update)" -ForegroundColor Yellow
