# Fix for aws-lc-sys Build Error

## Problem
The `aws-lc-sys` crate (used by AWS SDK) requires CMake and NASM to build from source, but they're not installed on your system.

## ✅ Solution 1: Install Required Dependencies (Recommended)

### Quick Install Script
Run the provided PowerShell script as Administrator:

```powershell
.\install_aws_dependencies.ps1
```

This will attempt to install:
- **CMake** - Build system required for native compilation
- **NASM** - Netwide Assembler for assembly code

### Manual Installation

#### Install CMake:
1. Download from: https://cmake.org/download/
2. Choose "Windows x64 Installer"
3. **Important**: During installation, check "Add CMake to system PATH"
4. Restart PowerShell after installation

#### Install NASM:
1. Download from: https://www.nasm.us/pub/nasm/releasebuilds/
2. Get the latest win64 version (e.g., `nasm-2.16.01-win64.zip`)
3. Extract to `C:\Program Files\NASM`
4. Add to PATH:
   ```powershell
   [Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\NASM", [EnvironmentVariableTarget]::Machine)
   ```
5. Restart PowerShell

### Verify Installation
```powershell
cmake --version
nasm -v
```

## ✅ Solution 2: Use Pre-built Binaries (Faster)

If you want to avoid building from source, you can use pre-built binaries:

```powershell
# Set environment variable to use pre-built binaries
$env:AWS_LC_SYS_USE_PKG_CONFIG = "1"
cargo build
```

Or add to your PowerShell profile (`$PROFILE`):
```powershell
$env:AWS_LC_SYS_USE_PKG_CONFIG = "1"
```

## ✅ Solution 3: Switch to rustls (Alternative TLS Backend)

If you continue having issues, you can switch the AWS SDK to use `rustls` instead of `native-tls`:

1. Update `Cargo.toml`:
   ```toml
   [dependencies]
   # Change from:
   sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "uuid", "chrono"] }
   # To:
   sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono"] }
   ```

2. For AWS SDK, you may need to use a different feature set. However, AWS SDK v1 typically uses native-tls by default.

## ✅ Solution 4: Use Package Managers

### Using winget (Windows Package Manager):
```powershell
winget install Kitware.CMake
winget install NASM.NASM
```

### Using Chocolatey:
```powershell
choco install cmake nasm -y
```

## 🔄 After Installation

1. **Restart PowerShell** (important for PATH updates)
2. Navigate to project:
   ```powershell
   cd C:\Users\Petchiappan.P\my_api
   ```
3. Clean and rebuild:
   ```powershell
   cargo clean
   cargo build
   ```

## 🐛 Troubleshooting

### Still getting CMake error?
- Make sure you restarted PowerShell after installation
- Verify CMake is in PATH: `where.exe cmake`
- Try running from a fresh PowerShell window

### Still getting NASM error?
- Verify NASM is in PATH: `where.exe nasm`
- Check if NASM is installed: `nasm -v`
- You may need to manually add to PATH if auto-installation didn't work

### Build still fails?
Try using pre-built binaries:
```powershell
$env:AWS_LC_SYS_USE_PKG_CONFIG = "1"
$env:AWS_LC_SYS_NO_ASM = "1"  # Disable assembly optimizations
cargo build
```

## 📝 Notes

- CMake is a large download (~100MB) but essential for building native dependencies
- NASM is smaller (~2MB) but required for assembly code in AWS-LC
- Both tools are commonly used in Rust projects with native dependencies
- The installation script will try multiple methods (winget, Chocolatey, manual)

## ✅ Recommended Approach

1. Run `.\install_aws_dependencies.ps1` as Administrator
2. Restart PowerShell
3. Run `cargo build`

If that doesn't work, manually install CMake and NASM using the links above.

