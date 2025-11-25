# Database Setup Guide

## 🎯 Database Options

You can connect to either:
1. **Local PostgreSQL** (for development)
2. **AWS RDS PostgreSQL** (for production)

## 📋 Option 1: Local PostgreSQL Setup

### Step 1: Install PostgreSQL

**Windows:**
```powershell
# Download from https://www.postgresql.org/download/windows/
# Or use Chocolatey
choco install postgresql
```

**macOS:**
```bash
brew install postgresql
brew services start postgresql
```

**Linux:**
```bash
sudo apt-get install postgresql postgresql-contrib
sudo systemctl start postgresql
```

### Step 2: Create Database

```bash
# Connect to PostgreSQL
psql -U postgres

# Create database and user
CREATE DATABASE mediacorp;
CREATE USER mediacorp_user WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE mediacorp TO mediacorp_user;
\q
```

### Step 3: Set Environment Variables

Create a `.env` file in the project root:

```env
# Local PostgreSQL
DATABASE_URL=postgresql://mediacorp_user:your_password@localhost:5432/mediacorp
DATABASE_USE_SSL=false
DATABASE_MAX_CONNECTIONS=20

# AWS Configuration
AWS_REGION=us-east-1
S3_BUCKET_STAGING=mediacorp-ai-ingress-staging
S3_BUCKET_PROCESSED=mediacorp-ai-processed

# Security
JWT_SECRET=your-super-secret-jwt-key-change-in-production
SSO_PROVIDER=mediacorp-sso

# Logging
LOG_LEVEL=info
```

### Step 4: Run Migrations

The migrations will run automatically when you start the application:

```bash
cargo run
```

Or manually:
```bash
sqlx migrate run
```

---

## ☁️ Option 2: AWS RDS PostgreSQL Setup

### Step 1: Create RDS Instance

1. **Go to AWS Console → RDS → Create Database**

2. **Configuration:**
   - **Engine**: PostgreSQL
   - **Version**: 15.x or 16.x (recommended)
   - **Template**: Production (or Dev/Test for development)
   - **DB Instance Identifier**: `mediacorp-ai-db`
   - **Master Username**: `mediacorp_admin`
   - **Master Password**: (set a strong password)

3. **Instance Configuration:**
   - **DB Instance Class**: 
     - Development: `db.t3.micro` or `db.t3.small`
     - Production: `db.r6g.large` or higher
   - **Storage**: 
     - Type: General Purpose SSD (gp3)
     - Allocated Storage: 100 GB (minimum)

4. **Connectivity:**
   - **VPC**: Your application VPC
   - **Subnet Group**: Create new or use existing
   - **Public Access**: 
     - ✅ Yes (for development/testing)
     - ❌ No (for production - use VPC peering)
   - **VPC Security Group**: Create new security group
   - **Availability Zone**: Choose based on your region

5. **Database Authentication:**
   - **Password authentication** (default)

6. **Additional Configuration:**
   - **Initial Database Name**: `mediacorp`
   - **Backup Retention**: 7 days (production)
   - **Enable Encryption**: ✅ Yes
   - **Performance Insights**: ✅ Enable (optional)

7. **Click "Create Database"**

### Step 2: Configure Security Group

1. **Go to EC2 → Security Groups**
2. **Find your RDS security group**
3. **Add Inbound Rule:**
   - **Type**: PostgreSQL
   - **Port**: 5432
   - **Source**: 
     - Your application's security group (recommended)
     - Or your IP for testing: `0.0.0.0/0` (NOT for production!)

### Step 3: Get RDS Endpoint

1. **Go to RDS → Databases**
2. **Click on your database instance**
3. **Copy the Endpoint** (e.g., `mediacorp-ai-db.abc123.us-east-1.rds.amazonaws.com`)

### Step 4: Set Environment Variables

Create a `.env` file or set in your deployment environment:

```env
# AWS RDS PostgreSQL
DATABASE_URL=postgresql://mediacorp_admin:your_password@mediacorp-ai-db.abc123.us-east-1.rds.amazonaws.com:5432/mediacorp
DATABASE_USE_SSL=true
DATABASE_MAX_CONNECTIONS=50

# AWS Configuration
AWS_REGION=us-east-1
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
S3_BUCKET_STAGING=mediacorp-ai-ingress-staging
S3_BUCKET_PROCESSED=mediacorp-ai-processed

# Security
JWT_SECRET=your-super-secret-jwt-key-change-in-production
SSO_PROVIDER=mediacorp-sso

# Logging
LOG_LEVEL=info

# Neptune (Optional - for graph database)
NEPTUNE_ENDPOINT=https://your-neptune-cluster.region.neptune.amazonaws.com:8182
```

### Step 5: Test Connection

```bash
# Test connection from your local machine (if public access enabled)
psql -h mediacorp-ai-db.abc123.us-east-1.rds.amazonaws.com -U mediacorp_admin -d mediacorp

# Or use connection string
psql "postgresql://mediacorp_admin:password@mediacorp-ai-db.abc123.us-east-1.rds.amazonaws.com:5432/mediacorp"
```

### Step 6: Run Migrations

```bash
# Set environment variables
export DATABASE_URL="postgresql://mediacorp_admin:password@mediacorp-ai-db.abc123.us-east-1.rds.amazonaws.com:5432/mediacorp"
export DATABASE_USE_SSL=true

# Run migrations
cargo run
# Migrations run automatically on startup
```

---

## 🔐 Security Best Practices for AWS RDS

### 1. Use IAM Database Authentication (Optional but Recommended)

Instead of password, use IAM roles:

```bash
# Enable IAM authentication in RDS console
# Then use IAM token for connection
```

### 2. Use Secrets Manager

Store database credentials in AWS Secrets Manager:

```env
# Instead of DATABASE_URL, use:
AWS_SECRETS_MANAGER_SECRET_NAME=mediacorp/database/credentials
```

### 3. VPC Configuration

- **Production**: Place RDS in private subnet, no public access
- **Use VPC Peering** or **VPN** to connect from application
- **Use Security Groups** to restrict access

### 4. Encryption

- ✅ Enable encryption at rest
- ✅ Enable SSL/TLS for connections (DATABASE_USE_SSL=true)

---

## 🚀 Quick Start Commands

### Local Setup
```bash
# 1. Create .env file
cat > .env << EOF
DATABASE_URL=postgresql://mediacorp_user:password@localhost:5432/mediacorp
DATABASE_USE_SSL=false
AWS_REGION=us-east-1
JWT_SECRET=dev-secret-key
EOF

# 2. Run migrations and start server
cargo run
```

### AWS RDS Setup
```bash
# 1. Create .env file with RDS endpoint
cat > .env << EOF
DATABASE_URL=postgresql://mediacorp_admin:password@your-rds-endpoint:5432/mediacorp
DATABASE_USE_SSL=true
DATABASE_MAX_CONNECTIONS=50
AWS_REGION=us-east-1
AWS_ACCESS_KEY_ID=your_key
AWS_SECRET_ACCESS_KEY=your_secret
JWT_SECRET=production-secret-key
EOF

# 2. Run migrations and start server
cargo run
```

---

## 📊 Connection String Format

### Local PostgreSQL
```
postgresql://username:password@localhost:5432/database_name
```

### AWS RDS PostgreSQL
```
postgresql://username:password@rds-endpoint.region.rds.amazonaws.com:5432/database_name
```

### With SSL Parameters
```
postgresql://username:password@host:5432/database_name?sslmode=require
```

---

## ✅ Verification

After setup, verify the connection:

```bash
# Check if migrations ran successfully
psql $DATABASE_URL -c "SELECT COUNT(*) FROM assets;"

# Or check via API
curl http://localhost:3000/api/controllers/status
```

---

## 🆘 Troubleshooting

### Connection Refused (Local)
- Check PostgreSQL is running: `sudo systemctl status postgresql`
- Check port 5432 is open
- Verify credentials in `.env`

### Connection Timeout (AWS RDS)
- Check Security Group allows your IP/security group
- Verify RDS is in same VPC as application
- Check RDS endpoint is correct
- Verify SSL is enabled if required

### SSL Error
- Set `DATABASE_USE_SSL=true` for AWS RDS
- For local, set `DATABASE_USE_SSL=false`

### Migration Errors
- Ensure database exists
- Check user has CREATE TABLE permissions
- Verify PostgreSQL version is 12+ (for UUID extension)
