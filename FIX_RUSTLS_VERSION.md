# Fix: Force rustls 0.21 to Avoid aws-lc-sys

## Problem
Even with AWS SDK 1.28.0, Cargo is resolving to newer versions that use `rustls` 0.23, which pulls in `aws-lc-rs` and `aws-lc-sys`.

## Solution: Manually Edit Cargo.lock

### Step 1: Build once to generate Cargo.lock
```cmd
cargo build
```
(This will fail, but it will create/update Cargo.lock)

### Step 2: Edit Cargo.lock

Open `Cargo.lock` in a text editor and find all instances of:
```
name = "rustls"
version = "0.23.x"
```

Change them to:
```
name = "rustls"
version = "0.21.12"
```

Also find and change:
```
name = "aws-lc-rs"
```

Remove or comment out the entire `[[package]]` block for `aws-lc-rs` and `aws-lc-sys`.

### Step 3: Rebuild
```cmd
cargo build
```

## Alternative: Use Exact Version Constraints

Edit `Cargo.toml` and add explicit version constraints:

```toml
[dependencies]
# ... existing dependencies ...

# Force rustls 0.21
rustls = { version = "0.21.12", default-features = false, features = ["tls12", "std"] }
rustls-webpki = "0.102"
webpki-roots = "0.26"
```

Then run:
```cmd
cargo update -p rustls
cargo build
```

## Note
This is a workaround. The proper fix would be for the `cmake` crate to support VS 2022, or for `aws-lc-sys` to have prebuilt binaries for Windows.

