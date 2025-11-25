# Quick Fix for aws-lc-sys Build Error

## 🚨 Problem
Your build is failing because `aws-lc-sys` needs **CMake** and **NASM** to compile.

## ✅ Quick Solution (Choose One)

### Option 1: Install via winget (Fastest)
Open PowerShell as Administrator and run:
```powershell
winget install Kitware.CMake --source winget
winget install NASM.NASM --source winget
```

### Option 2: Install via Chocolatey
If you have Chocolatey installed:
```powershell
choco install cmake nasm -y
```

### Option 3: Manual Installation

#### CMake:
1. Download: https://cmake.org/download/
2. Choose "Windows x64 Installer"
3. **Important**: Check "Add CMake to system PATH" during installation
4. Install and restart PowerShell

#### NASM:
1. Download: https://www.nasm.us/pub/nasm/releasebuilds/2.16.01/win64/nasm-2.16.01-win64.zip
2. Extract to `C:\Program Files\NASM`
3. Add to PATH:
   ```powershell
   [Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\NASM", [EnvironmentVariableTarget]::User)
   ```
4. Restart PowerShell

### Option 4: Use Pre-built Binaries (Skip Building)
If you want to avoid building from source:
```powershell
$env:AWS_LC_SYS_USE_PKG_CONFIG = "1"
$env:AWS_LC_SYS_NO_ASM = "1"
cargo clean
cargo build
```

## 🔄 After Installation

1. **Close and restart PowerShell** (important!)
2. Verify installation:
   ```powershell
   cmake --version
   nasm -v
   ```
3. Clean and rebuild:
   ```powershell
   cd C:\Users\Petchiappan.P\my_api
   cargo clean
   cargo build
   ```

## 📝 What These Tools Do

- **CMake**: Build system used to compile native C/C++ code in AWS-LC
- **NASM**: Assembler needed for optimized cryptographic code

Both are standard tools for Rust projects with native dependencies.

## ✅ Recommended: Use winget
The fastest way is usually:
```powershell
# Run as Administrator
winget install Kitware.CMake --source winget
winget install NASM.NASM --source winget
# Then restart PowerShell and run: cargo build
```

