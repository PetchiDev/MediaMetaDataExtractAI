# Quick Start - Local PostgreSQL Connection

## ✅ Database Connection Configured

Your `.env` file has been created with:
- **Username**: `postgres`
- **Password**: `password@1`
- **Database**: `MediaAI`
- **Host**: `localhost:5432`

## 🔧 Important: Password URL Encoding

Since your password contains special characters (`@`), it's been URL-encoded in the connection string:
- `password@1` → `password%401` (in connection string)

The connection string format:
```
postgresql://postgres:password%401@localhost:5432/MediaAI
```

## 📋 Next Steps

### 1. Verify PostgreSQL is Running

**Windows:**
```powershell
# Check if PostgreSQL service is running
Get-Service -Name postgresql*

# Or check in Services app
services.msc
```

**macOS/Linux:**
```bash
# Check PostgreSQL status
sudo systemctl status postgresql
# Or
brew services list | grep postgresql
```

### 2. Create Database (if not exists)

Connect to PostgreSQL and create the database:

```bash
# Connect as postgres user
psql -U postgres

# Create database
CREATE DATABASE "MediaAI";

# Verify
\l

# Exit
\q
```

Or using command line:
```bash
psql -U postgres -c "CREATE DATABASE \"MediaAI\";"
```

### 3. Test Connection

```bash
# Test connection
psql -U postgres -d MediaAI -h localhost

# Or with connection string
psql "postgresql://postgres:password@1@localhost:5432/MediaAI"
```

### 4. Run the Application

```bash
# Build and run
cargo run
```

The application will:
1. ✅ Load `.env` file automatically
2. ✅ Connect to PostgreSQL database `MediaAI`
3. ✅ Run migrations automatically
4. ✅ Start the API server on `http://localhost:3000`

## 🐛 Troubleshooting

### Connection Refused
```bash
# Check if PostgreSQL is running
# Windows: Check Services
# macOS: brew services start postgresql
# Linux: sudo systemctl start postgresql
```

### Authentication Failed
- Verify password is correct: `password@1`
- Check if user `postgres` exists
- Try connecting manually: `psql -U postgres`

### Database Does Not Exist
```sql
-- Connect as postgres
psql -U postgres

-- Create database
CREATE DATABASE "MediaAI";
```

### Password with Special Characters
If you have issues with the password encoding, try:
1. Use quotes in connection string (some drivers support this)
2. Or URL-encode manually:
   - `@` → `%40`
   - `#` → `%23`
   - `%` → `%25`
   - etc.

## ✅ Verify Setup

After running `cargo run`, you should see:
```
Starting AI Media Metadata Processing Platform
Database migrations completed
Server listening on 0.0.0.0:3000
```

Then test the API:
```bash
# Check health
curl http://localhost:3000/api/controllers/status
```

## 📝 Alternative: Use Environment Variables Directly

If `.env` file doesn't work, set environment variables directly:

**Windows PowerShell:**
```powershell
$env:DATABASE_URL="postgresql://postgres:password%401@localhost:5432/MediaAI"
$env:DATABASE_USE_SSL="false"
$env:AWS_REGION="us-east-1"
$env:JWT_SECRET="dev-secret-key"
cargo run
```

**Windows CMD:**
```cmd
set DATABASE_URL=postgresql://postgres:password%401@localhost:5432/MediaAI
set DATABASE_USE_SSL=false
set AWS_REGION=us-east-1
set JWT_SECRET=dev-secret-key
cargo run
```

**macOS/Linux:**
```bash
export DATABASE_URL="postgresql://postgres:password%401@localhost:5432/MediaAI"
export DATABASE_USE_SSL="false"
export AWS_REGION="us-east-1"
export JWT_SECRET="dev-secret-key"
cargo run
```

## 🎯 Ready to Go!

Your database connection is configured. Just run:
```bash
cargo run
```

The application will automatically:
- Connect to your local PostgreSQL
- Create all necessary tables
- Start the API server
