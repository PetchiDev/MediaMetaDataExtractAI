# Script to remove aws-lc-rs dependencies from Cargo.lock
# This allows the build to proceed without aws-lc-sys compilation

Write-Host "Fixing Cargo.lock to remove aws-lc-rs dependencies..." -ForegroundColor Cyan

$lockFile = "Cargo.lock"

if (-not (Test-Path $lockFile)) {
    Write-Host "Error: Cargo.lock not found. Run 'cargo build' first to generate it." -ForegroundColor Red
    exit 1
}

# Backup Cargo.lock
Copy-Item $lockFile "$lockFile.backup"
Write-Host "Backup created: $lockFile.backup" -ForegroundColor Green

# Read the file
$content = Get-Content $lockFile -Raw

# Remove aws-lc-rs from rustls 0.23.35 dependencies
$content = $content -replace '("aws-lc-rs",\s*\r?\n)', ''

# Remove aws-lc-rs from rustls-webpki dependencies  
$content = $content -replace '("aws-lc-rs",\s*\r?\n)', ''

# Comment out aws-lc-rs package block
$content = $content -replace '(\[\[package\]\]\s*\r?\nname = "aws-lc-rs"[\s\S]*?)(\r?\n\[\[package\]\])', '# $1$2'

# Comment out aws-lc-sys package block
$content = $content -replace '(\[\[package\]\]\s*\r?\nname = "aws-lc-sys"[\s\S]*?)(\r?\n\[\[package\]\])', '# $1$2'

# Write back
Set-Content -Path $lockFile -Value $content -NoNewline

Write-Host "Cargo.lock updated!" -ForegroundColor Green
Write-Host ""
Write-Host "Now run: cargo build" -ForegroundColor Yellow

