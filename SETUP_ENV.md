# 🔧 Environment Variables Setup

## Create .env File

Create a file named `.env` in the project root with the following content:

```bash
# Google OAuth Configuration
GOOGLE_CLIENT_ID=5531435566-5c4eqmipl0c78pnoihotr54m2su6ckac.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=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
GOOGLE_REDIRECT_URI=http://localhost:3000/api/auth/google/callback

# JWT Secret
JWT_SECRET=change-me-in-production-secret-key

# Database Configuration
DATABASE_URL=postgresql://postgres:password@localhost/mediacorp

# Local Storage (for testing)
USE_LOCAL_STORAGE=true
LOCAL_STORAGE_PATH=./local_storage
```

## Quick Setup Commands

### Windows PowerShell
```powershell
@"
# Google OAuth Configuration
GOOGLE_CLIENT_ID=5531435566-5c4eqmipl0c78pnoihotr54m2su6ckac.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=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
GOOGLE_REDIRECT_URI=http://localhost:3000/api/auth/google/callback

# JWT Secret
JWT_SECRET=change-me-in-production-secret-key

# Database Configuration
DATABASE_URL=postgresql://postgres:password@localhost/mediacorp

# Local Storage (for testing)
USE_LOCAL_STORAGE=true
LOCAL_STORAGE_PATH=./local_storage
"@ | Out-File -FilePath .env -Encoding utf8
```

### Linux/Mac
```bash
cat > .env << 'EOF'
# Google OAuth Configuration
GOOGLE_CLIENT_ID=5531435566-5c4eqmipl0c78pnoihotr54m2su6ckac.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=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
GOOGLE_REDIRECT_URI=http://localhost:3000/api/auth/google/callback

# JWT Secret
JWT_SECRET=change-me-in-production-secret-key

# Database Configuration
DATABASE_URL=postgresql://postgres:password@localhost/mediacorp

# Local Storage (for testing)
USE_LOCAL_STORAGE=true
LOCAL_STORAGE_PATH=./local_storage
EOF
```

## Verify Setup

After creating `.env` file, verify it exists:

```bash
# Windows
dir .env

# Linux/Mac
ls -la .env
```

## Test Google Sign-In

Once `.env` is created:

```bash
# Start server
cargo run

# Test Google login (in another terminal)
curl http://localhost:3000/api/auth/google/login
```

## Important

✅ **Credentials Set**:
- Client ID: `5531435566-5c4eqmipl0c78pnoihotr54m2su6ckac.apps.googleusercontent.com`
- Client Secret: `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`
- Redirect URI: `http://localhost:3000/api/auth/google/callback`

⚠️ **Security**: 
- `.env` file is in `.gitignore` (won't be committed)
- Keep credentials secure
- Don't share Client Secret

## Next Steps

1. ✅ Create `.env` file with above content
2. ✅ Update `DATABASE_URL` with your PostgreSQL credentials
3. ✅ Run `cargo run` to start server
4. ✅ Test Google Sign-In

