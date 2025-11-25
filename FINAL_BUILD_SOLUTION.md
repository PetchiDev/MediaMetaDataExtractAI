# Final Build Solution for Visual Studio 2022

## 🎯 The Core Problem

The `cmake` crate (v0.1.54) used by `aws-lc-sys` doesn't recognize Visual Studio 2022 (version 18). This is a known compatibility issue.

## ✅ Working Solutions (In Order of Preference)

### Solution 1: Use Developer Command Prompt (Easiest)

1. **Open "Developer Command Prompt for VS 2022"** (search in Start menu)
2. Navigate to your project:
   ```cmd
   cd C:\Users\Petchiappan.P\my_api
   ```
3. Build:
   ```cmd
   cargo build
   ```

The Developer Command Prompt has all Visual Studio environment variables pre-configured, which helps CMake find Visual Studio correctly.

### Solution 2: Patch Cargo Dependencies

Add this to your `Cargo.toml` to force a newer cmake crate (if compatible):

```toml
[patch.crates-io]
# Try to use a newer cmake if available
cmake = { version = "0.2", optional = true }
```

**Note**: This may not work if `aws-lc-sys` has strict version requirements.

### Solution 3: Use Rustls Instead of Native TLS

Switch from `native-tls` to `rustls` in your `Cargo.toml`:

```toml
# Change this line:
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "uuid", "chrono"] }

# To this:
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono"] }
```

This avoids the `aws-lc-sys` dependency entirely. **However**, you may need to update your AWS SDK configuration to use rustls as well.

### Solution 4: Manual CMake Generator Setup

If you have CMake installed separately, you can try:

```powershell
# Find your Visual Studio installation
$vsPath = "C:\Program Files\Microsoft Visual Studio\18\Community"

# Set up environment
$env:CMAKE_GENERATOR = "Visual Studio 17 2022"
$env:CMAKE_GENERATOR_PLATFORM = "x64"

# Try to point CMake to Visual Studio
$env:CMAKE_PREFIX_PATH = $vsPath

# Build
cargo build
```

### Solution 5: Wait for Upstream Fix

The proper fix would be:
- Update `aws-lc-sys` to use a newer `cmake` crate
- Or update the `cmake` crate itself to support VS 2022

You can track this issue and contribute to the upstream projects.

## 🚀 Recommended Immediate Action

**Use Solution 1** (Developer Command Prompt) - it's the quickest and most reliable:

1. Press `Win + S`
2. Type "Developer Command Prompt for VS 2022"
3. Open it
4. Navigate to your project and run `cargo build`

## 📝 Long-term Fix

For a permanent solution, consider:

1. **Switch to rustls** (Solution 3) - avoids native compilation entirely
2. **File an issue** with the `aws-lc-sys` or `cmake` crate maintainers
3. **Use Docker/WSL** for builds if Windows compatibility is problematic

## 🔍 Verify Your Setup

Check if Visual Studio is properly installed:

```powershell
& "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
```

This should return a path like: `C:\Program Files\Microsoft Visual Studio\18\Community`

## 💡 Why This Happens

- Visual Studio 2022 uses version number 17 internally
- But newer installations might show as version 18
- The `cmake` crate v0.1.54 only recognizes specific VS versions
- This is a compatibility gap between the Rust ecosystem and newer VS versions

## ✅ Quick Test

Try the Developer Command Prompt approach first - it usually works immediately!

