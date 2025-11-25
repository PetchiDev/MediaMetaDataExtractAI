# Rust Installation Guide for Windows

## 🦀 Install Rust on Windows

### Method 1: Using rustup (Recommended)

1. **Download rustup-init.exe:**
   - Go to: https://rustup.rs/
   - Or direct download: https://win.rustup.rs/x86_64
   - Click "Download rustup-init.exe"

2. **Run the installer:**
   ```powershell
   # Double-click rustup-init.exe
   # Or run from PowerShell:
   .\rustup-init.exe
   ```

3. **Follow the prompts:**
   - Press `1` to proceed with default installation
   - The installer will download and install Rust

4. **Restart your terminal/PowerShell:**
   - Close and reopen PowerShell/CMD
   - Or run: `refreshenv` (if using Chocolatey)

5. **Verify installation:**
   ```powershell
   rustc --version
   cargo --version
   ```

### Method 2: Using Chocolatey (If you have Chocolatey)

```powershell
# Install Chocolatey first (if not installed)
# Then install Rust
choco install rust

# Restart terminal
refreshenv
```

### Method 3: Using Scoop (If you have Scoop)

```powershell
scoop install rust
```

## ✅ After Installation

1. **Verify Rust is installed:**
   ```powershell
   rustc --version
   cargo --version
   ```

2. **Add Rust to PATH (if needed):**
   - Rust should automatically add itself to PATH
   - If not, add: `C:\Users\YourName\.cargo\bin` to PATH

3. **Restart your terminal/PowerShell**

4. **Navigate to your project:**
   ```powershell
   cd C:\Users\Petchiappan.P\my_api
   ```

5. **Run the application:**
   ```powershell
   cargo run
   ```

## 🔧 Troubleshooting

### Cargo not found after installation

1. **Check PATH:**
   ```powershell
   $env:PATH -split ';' | Select-String cargo
   ```

2. **Add to PATH manually:**
   - Open System Properties → Environment Variables
   - Add: `C:\Users\YourName\.cargo\bin`
   - Restart terminal

3. **Or use full path:**
   ```powershell
   C:\Users\YourName\.cargo\bin\cargo.exe run
   ```

### Build errors

If you get build errors, make sure:
- PostgreSQL is running
- Database `MediaAI` exists
- `.env` file is configured correctly

## 📦 What Gets Installed

- **rustc**: Rust compiler
- **cargo**: Rust package manager and build tool
- **rustup**: Rust toolchain installer
- **rustfmt**: Code formatter
- **clippy**: Linter

## 🚀 Quick Install Command

If you have PowerShell with internet access:

```powershell
# Download and install in one command
Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe -y
```

Then restart your terminal and run:
```powershell
cargo run
```

## 📝 Next Steps After Installation

1. ✅ Install Rust (using one of the methods above)
2. ✅ Restart PowerShell/terminal
3. ✅ Verify: `cargo --version`
4. ✅ Navigate to project: `cd C:\Users\Petchiappan.P\my_api`
5. ✅ Run: `cargo run`

The application will:
- Connect to PostgreSQL database `MediaAI`
- Run database migrations
- Start the API server on `http://localhost:3000`
