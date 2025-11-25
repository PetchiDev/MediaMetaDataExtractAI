# Quick Fix for Linker Error on Windows

## 🚨 Error
```
error: linker `link.exe` not found
```

## ✅ Quick Solution

### Option 1: Install Visual Studio Build Tools (5 minutes)

**Automated (PowerShell as Administrator):**
```powershell
.\install_build_tools.ps1
```

**Manual:**
1. Download: https://aka.ms/vs/17/release/vs_buildtools.exe
2. Run installer
3. Select **"Desktop development with C++"**
4. Click Install
5. **Restart PowerShell**
6. Run: `cargo run`

### Option 2: Use Chocolatey (if installed)

```powershell
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools --quiet"
```

### Option 3: Use winget (Windows 10/11)

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools --override "--quiet --add Microsoft.VisualStudio.Workload.VCTools"
```

## ⚡ After Installation

1. **Close and reopen PowerShell** (IMPORTANT!)
2. Navigate to project:
   ```powershell
   cd C:\Users\Petchiappan.P\my_api
   ```
3. Run:
   ```powershell
   cargo run
   ```

## ✅ Verify It Works

```powershell
# Check if link.exe is found
where.exe link.exe
```

Should show a path like:
```
C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\...\link.exe
```

## 📝 Why This Happens

Rust needs a C++ compiler to build native dependencies. On Windows, this is provided by Visual Studio Build Tools. It's a one-time installation.

## 🎯 Recommended

**Just run the installer script** - it's the easiest:
```powershell
# Run as Administrator
.\install_build_tools.ps1
```

Then restart PowerShell and `cargo run` will work!



