# Windows Setup Guide - Visual C++ Build Tools

## 🔧 Problem
Rust needs a C++ linker (`link.exe`) to compile native dependencies. On Windows, this comes with Visual Studio Build Tools.

## ✅ Solution 1: Install Visual Studio Build Tools (Recommended)

### Step 1: Download Build Tools
1. Go to: https://visualstudio.microsoft.com/downloads/
2. Scroll down to **"Tools for Visual Studio"**
3. Download **"Build Tools for Visual Studio 2022"** (or 2019/2017)

### Step 2: Install
1. Run the installer (`vs_buildtools.exe`)
2. Select **"Desktop development with C++"** workload
3. Make sure these components are selected:
   - ✅ MSVC v143 - VS 2022 C++ x64/x86 build tools
   - ✅ Windows 10/11 SDK (latest version)
   - ✅ C++ CMake tools for Windows
4. Click **"Install"**
5. Wait for installation (may take 10-20 minutes)

### Step 3: Restart PowerShell
```powershell
# Close and reopen PowerShell
# Then try again:
cargo run
```

## ✅ Solution 2: Use GNU Toolchain (Alternative)

If you don't want to install Visual Studio, you can use the GNU toolchain:

### Step 1: Install MSYS2
1. Download from: https://www.msys2.org/
2. Install MSYS2
3. Open MSYS2 terminal and run:
```bash
pacman -S mingw-w64-x86_64-gcc
```

### Step 2: Configure Rust to use GNU toolchain
```powershell
# In PowerShell, set environment variable
$env:CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "x86_64-w64-mingw32-gcc"
$env:CC_x86_64_pc_windows_gnu = "x86_64-w64-mingw32-gcc"

# Or add to your .cargo/config.toml
```

### Step 3: Install GNU target
```powershell
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
```

## ✅ Solution 3: Quick Install Script

Create a file `install_build_tools.ps1`:

```powershell
# Download and install Visual Studio Build Tools
$url = "https://aka.ms/vs/17/release/vs_buildtools.exe"
$output = "$env:TEMP\vs_buildtools.exe"

Write-Host "Downloading Visual Studio Build Tools..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $url -OutFile $output

Write-Host "Installing Build Tools..." -ForegroundColor Yellow
Write-Host "Please select 'Desktop development with C++' workload" -ForegroundColor Yellow

Start-Process -FilePath $output -ArgumentList "--quiet", "--wait", "--add", "Microsoft.VisualStudio.Workload.VCTools" -Wait

Write-Host "Installation complete! Please restart PowerShell." -ForegroundColor Green
```

## 🚀 Quick Fix (One Command)

If you have Chocolatey installed:

```powershell
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools --quiet"
```

Or with winget:

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools --override "--quiet --add Microsoft.VisualStudio.Workload.VCTools"
```

## ✅ Verify Installation

After installing, restart PowerShell and verify:

```powershell
# Check if link.exe is available
where.exe link.exe

# Should show path like:
# C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.xx.xxxxx\bin\Hostx64\x64\link.exe
```

## 📝 After Installation

1. **Restart PowerShell** (important!)
2. Navigate to project:
   ```powershell
   cd C:\Users\Petchiappan.P\my_api
   ```
3. Try again:
   ```powershell
   cargo run
   ```

## 🔍 Troubleshooting

### Still getting linker error after installation
1. Make sure you restarted PowerShell
2. Check PATH includes Visual Studio tools:
   ```powershell
   $env:PATH -split ';' | Select-String "Visual Studio"
   ```
3. Try running from "Developer Command Prompt for VS":
   - Search for "Developer Command Prompt" in Start menu
   - Run from there

### Alternative: Use pre-built binaries
Some crates offer pre-built binaries. You can also try:
```powershell
# Use pre-built dependencies where possible
cargo build --release
```

## 💡 Recommendation

**Install Visual Studio Build Tools** - it's the most reliable solution and only takes one installation.



