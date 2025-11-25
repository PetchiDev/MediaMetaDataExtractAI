# Visual Studio 2022 Build Fix

## 🚨 Problem

The build fails with two main issues:
1. **CMake crate compatibility**: The `cmake` crate (v0.1.54) doesn't recognize Visual Studio 2022
2. **C11 standard**: MSVC compiler needs explicit C11 standard flag

## ✅ Quick Fix (Choose One Method)

### Method 1: Use Build Wrapper Script (Recommended)

```powershell
# Run the wrapper script
.\cargo-build-vs2022.ps1

# Or with specific cargo commands
.\cargo-build-vs2022.ps1 build --release
.\cargo-build-vs2022.ps1 run
```

### Method 2: Set Environment Variables Manually

```powershell
# Set environment variables
$env:CMAKE_GENERATOR = "Visual Studio 17 2022"
$env:AWS_LC_SYS_C_STD = "c11"
$env:AWS_LC_SYS_USE_PKG_CONFIG = "1"
$env:CFLAGS = "/std:c11"
$env:CFLAGS_x86_64_pc_windows_msvc = "/std:c11"

# Then build
cargo clean
cargo build
```

### Method 3: Use Pre-built Binaries (Fastest, No Compilation)

```powershell
# This avoids building aws-lc from source
$env:AWS_LC_SYS_USE_PKG_CONFIG = "1"
$env:CMAKE_GENERATOR = "Visual Studio 17 2022"

cargo clean
cargo build
```

### Method 4: Update Dependencies (Long-term Solution)

Add this to your `Cargo.toml` to use newer AWS SDK versions that might have better compatibility:

```toml
[patch.crates-io]
# Force newer cmake crate if available
cmake = { version = "0.2", optional = true }
```

However, since `aws-lc-sys` is a transitive dependency, this may not work directly.

## 🔧 Detailed Explanation

### Issue 1: CMake Crate Compatibility

The error message:
```
Visual studio version detected but this crate doesn't know how to generate cmake files for it
```

**Solution**: Explicitly set `CMAKE_GENERATOR` to `"Visual Studio 17 2022"` which tells CMake to use VS 2022.

### Issue 2: C11 Standard

The error message:
```
"C atomics require C11 or later"
```

**Solution**: Set compiler flags to use C11 standard:
- `CFLAGS=/std:c11`
- `CFLAGS_x86_64_pc_windows_msvc=/std:c11`
- `AWS_LC_SYS_C_STD=c11`

## 📝 Persistent Setup

To make these settings persistent, you can:

1. **Create a PowerShell profile script**:
   ```powershell
   # Add to $PROFILE
   $env:CMAKE_GENERATOR = "Visual Studio 17 2022"
   $env:AWS_LC_SYS_C_STD = "c11"
   ```

2. **Use the wrapper script** for all builds:
   ```powershell
   # Always use
   .\cargo-build-vs2022.ps1 build
   ```

3. **Set system environment variables** (Windows):
   - Open System Properties → Environment Variables
   - Add `CMAKE_GENERATOR` = `Visual Studio 17 2022`
   - Add `AWS_LC_SYS_C_STD` = `c11`

## 🧪 Verify the Fix

After applying the fix:

```powershell
# Clean build
cargo clean

# Build with wrapper
.\cargo-build-vs2022.ps1 build

# Or manually with env vars set
cargo build
```

## 🔍 Alternative: Use Different AWS SDK Version

If the issue persists, you might try updating to the latest AWS SDK:

```toml
[dependencies]
aws-sdk-s3 = "1.50"  # Try latest version
aws-sdk-lambda = "1.50"
aws-sdk-sfn = "1.50"
aws-config = "1.1"
```

Then run:
```powershell
cargo update
cargo build
```

## 📚 References

- [AWS-LC-SYS Documentation](https://github.com/aws/aws-lc-rs)
- [CMake Generators](https://cmake.org/cmake/help/latest/manual/cmake-generators.7.html)
- [Rust on Windows](https://rust-lang.github.io/rustup/installation/windows.html)

