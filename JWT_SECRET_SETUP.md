# 🔐 JWT Secret Setup Guide

## What is JWT_SECRET?

JWT_SECRET is a secret key used to sign and verify JWT tokens. It's critical for security!

## How to Generate JWT_SECRET

### Method 1: Using Rust (Recommended)

```bash
# Run this command to generate a secure secret
cargo run --bin generate-jwt-secret
```

Or create a simple script:

```rust
// In src/bin/generate_secret.rs
use my_api::utils::jwt::JWTService;

fn main() {
    let secret = JWTService::generate_secret();
    println!("Generated JWT_SECRET:");
    println!("{}", secret);
    println!("\nAdd this to your .env file:");
    println!("JWT_SECRET={}", secret);
}
```

### Method 2: Using PowerShell (Windows)

```powershell
# Generate random secret
$secret = -join ((65..90) + (97..122) + (48..57) | Get-Random -Count 64 | ForEach-Object {[char]$_})
$secret = "jwt_secret_" + $secret
Write-Host "JWT_SECRET=$secret"
```

### Method 3: Using OpenSSL (Linux/Mac)

```bash
# Generate 64 character random secret
openssl rand -hex 32
```

### Method 4: Using Online Generator

Go to: https://www.uuidgenerator.net/ or use any secure random string generator

**Generate at least 32 characters** for security.

## Set JWT_SECRET in .env

### Current .env File

Your `.env` file should have:

```bash
JWT_SECRET=your-secret-key-here
```

### Update .env with Generated Secret

```bash
# Option 1: Edit .env manually
# Add or update this line:
JWT_SECRET=jwt_secret_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6

# Option 2: Use PowerShell to update
(Get-Content .env) -replace 'JWT_SECRET=.*', "JWT_SECRET=jwt_secret_$(New-Guid)" | Set-Content .env
```

## Example JWT_SECRET

Here's an example secure JWT_SECRET (don't use this in production!):

```
JWT_SECRET=jwt_secret_7f8a9b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2
```

## How JWT Works

### 1. Token Generation (After Google Sign-In)

```rust
// In google_callback handler
let token = JWTService::generate_access_token(
    &user.id.to_string(),
    &user.email,
    &format!("{:?}", user.role),
)?;
```

**Token contains:**
- `user_id`: User UUID
- `email`: User email
- `role`: User role (VIEWER, ADMIN, etc.)
- `exp`: Expiration timestamp (24 hours)

### 2. Token Validation (In Middleware)

```rust
// In auth middleware
let claims = JWTService::validate_token(token)?;
// Claims extracted and added to request
```

### 3. Using Token in API Calls

```bash
# Include token in Authorization header
curl http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIs..."
```

## Security Best Practices

### ✅ DO:
- Use **long random strings** (at least 32 characters)
- Keep JWT_SECRET **secret** (never commit to git)
- Use **different secrets** for dev/staging/production
- **Rotate secrets** periodically

### ❌ DON'T:
- Use simple words like "password" or "secret"
- Share JWT_SECRET publicly
- Use the same secret everywhere
- Commit .env to git

## Quick Setup

### Step 1: Generate Secret

```powershell
# PowerShell
$secret = "jwt_secret_" + (-join ((48..57) + (97..122) | Get-Random -Count 48 | ForEach-Object {[char]$_}))
echo "JWT_SECRET=$secret"
```

### Step 2: Add to .env

```bash
# Copy the generated secret and add to .env
JWT_SECRET=jwt_secret_your_generated_secret_here
```

### Step 3: Restart Server

```bash
cargo run
```

## Verify JWT_SECRET is Set

### Check Environment Variable

```bash
# Windows PowerShell
$env:JWT_SECRET

# Linux/Mac
echo $JWT_SECRET
```

### Test JWT Generation

After Google Sign-In, you should get a JWT token. Decode it at https://jwt.io to verify it's signed correctly.

## Troubleshooting

### Error: "JWT_SECRET not set"
- Check `.env` file exists
- Verify `JWT_SECRET=...` line is present
- Restart server after adding to .env

### Error: "Token validation failed"
- Check JWT_SECRET matches between generation and validation
- Verify token hasn't expired
- Check token format is correct

### Token Not Working
- Make sure JWT_SECRET is the same in all environments
- Verify token is included in Authorization header
- Check token hasn't expired (24 hour expiry)

## Current Setup

Your `.env` file should have:

```bash
JWT_SECRET=change-me-in-production-secret-key
```

**⚠️ Change this to a secure random string!**

## Generate and Update Now

Run this to generate and update:

```powershell
# Generate new secret
$newSecret = "jwt_secret_" + (-join ((48..57) + (97..122) | Get-Random -Count 48 | ForEach-Object {[char]$_}))

# Update .env
$content = Get-Content .env
$updated = $content | ForEach-Object {
    if ($_ -match '^JWT_SECRET=') {
        "JWT_SECRET=$newSecret"
    } else {
        $_
    }
}
$updated | Set-Content .env

Write-Host "✅ JWT_SECRET updated in .env"
Write-Host "New secret: $newSecret"
```

## Summary

✅ **JWT_SECRET** is used to sign/verify tokens  
✅ **Generate** a secure random string  
✅ **Add to .env** file  
✅ **Keep it secret** - never commit to git  
✅ **Restart server** after updating  

Your JWT implementation is ready! 🎉

