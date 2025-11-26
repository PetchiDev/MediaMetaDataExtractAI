# 📋 API Endpoints Summary

## ✅ All Endpoints Now in Swagger UI

### 🔐 Authentication (Auth Tag)

1. **GET `/api/auth/google/login`**
   - Get Google OAuth redirect URL
   - Returns: `redirect_url` and `callback_url`
   - **No auth required**

2. **GET `/api/auth/google/callback?code=...&state=...`**
   - Google OAuth callback
   - Returns: JWT `access_token` and `refresh_token`
   - **No auth required**

3. **POST `/api/auth/api-keys`**
   - Generate API key for technical users
   - Requires: JWT token
   - Returns: API key (shown only once!)

---

### 📤 Media (Media Tag)

4. **POST `/api/media/submit`**
   - Submit media via API (technical users)
   - Requires: API Key
   - Upload: File + metadata + operational_tags

5. **POST `/api/media/upload`**
   - Upload media via UI (naive users)
   - Requires: JWT token
   - Upload: File + basic metadata

6. **GET `/api/media/{asset_id}`**
   - Get media asset information
   - Requires: JWT or API Key

7. **GET `/api/media/{asset_id}/download`**
   - Download/stream media file
   - Requires: JWT or API Key

---

### 📝 Metadata (Metadata Tag)

8. **GET `/api/metadata/{asset_id}`**
   - Get enriched metadata
   - Requires: JWT or API Key

9. **PUT `/api/metadata/{asset_id}`**
   - Update metadata
   - Requires: JWT or API Key
   - Supports: Conflict detection

10. **POST `/api/metadata/{asset_id}/resolve-conflict`**
    - Resolve metadata conflicts
    - Requires: JWT or API Key

---

### ⚙️ Workflow (Workflow Tag)

11. **GET `/api/workflow/status/{asset_id}`**
    - Get AI processing workflow status
    - Requires: JWT or API Key
    - Returns: Progress, capabilities completed

---

### 🔍 Graph Search (Graph Tag)

12. **POST `/api/graph/search`**
    - Search assets using graph relationships
    - Requires: JWT or API Key
    - Returns: Assets + relationship graph

---

### 👨‍💼 Admin (Admin Tag)

13. **GET `/api/admin/controllers/status`**
    - Get controller health metrics
    - Requires: JWT token
    - Returns: Ingress/Egress controller status

---

## 🎯 Complete Flow

### Step 1: Authenticate
```
GET /api/auth/google/login
→ Open redirect_url in browser
→ Sign in with Google
→ Get JWT token from callback
```

### Step 2: Upload Media
```
POST /api/media/upload
→ Upload file + metadata
→ Get asset_uuid
```

### Step 3: Check Status
```
GET /api/workflow/status/{asset_id}
→ Wait for status = "COMPLETED"
```

### Step 4: Get Metadata
```
GET /api/metadata/{asset_id}
→ View enriched metadata
```

### Step 5: Search Assets
```
POST /api/graph/search
→ Find related assets
```

---

## 📖 Documentation Files

1. **COMPLETE_API_FLOW_GUIDE.md** - Detailed step-by-step guide
2. **QUICK_START_API.md** - Quick reference
3. **API_ENDPOINTS_SUMMARY.md** - This file

---

## 🔗 Swagger UI

Access at: `http://localhost:3000/swagger-ui`

All endpoints are now visible with:
- ✅ Request/Response schemas
- ✅ Authentication requirements
- ✅ Try it out functionality
- ✅ Google SSO callback URL displayed

---

**All endpoints documented!** 🎉

