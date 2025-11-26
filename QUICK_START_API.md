# ⚡ Quick Start - API Usage

## 🚀 Step 1: Start Server

```bash
cargo run
```

Server runs on: `http://localhost:3000`

---

## 🔐 Step 2: Get JWT Token (Google Sign-In)

### Option A: Using Swagger UI

1. Open: `http://localhost:3000/swagger-ui`
2. Find: **Auth** → `GET /api/auth/google/login`
3. Click **"Try it out"** → **"Execute"**
4. Copy `redirect_url` from response
5. Open `redirect_url` in browser
6. Sign in with Google
7. Google redirects to callback
8. Copy `access_token` from callback response

### Option B: Using curl

```bash
# 1. Get redirect URL
curl http://localhost:3000/api/auth/google/login

# Response:
# {
#   "redirect_url": "https://accounts.google.com/o/oauth2/v2/auth?...",
#   "callback_url": "http://localhost:3000/api/auth/google/callback"
# }

# 2. Open redirect_url in browser
# 3. After Google auth, you'll be redirected to callback_url
# 4. Copy access_token from response
```

**Save your token**:
```bash
export JWT_TOKEN="eyJhbGciOiJIUzI1NiIs..."
```

---

## 📤 Step 3: Upload Media

### Using Swagger UI

1. Click **"Authorize"** button (top right)
2. Enter: `Bearer YOUR_JWT_TOKEN`
3. Go to **Media** → `POST /api/media/upload`
4. Click **"Try it out"**
5. Upload file, fill metadata
6. Click **"Execute"**

### Using curl

```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -F "file=@video.mp4" \
  -F "title=My Video" \
  -F "description=Test video" \
  -F "category=news" \
  -F "tags=tag1,tag2"
```

**Response**:
```json
{
  "asset_uuid": "abc-123-def",
  "status": "PROCESSING",
  "message": "Upload successful. AI processing started."
}
```

**Save**: `asset_uuid`

---

## 📊 Step 4: Check Workflow Status

```bash
curl http://localhost:3000/api/workflow/status/ASSET_UUID \
  -H "Authorization: Bearer $JWT_TOKEN"
```

**Response**:
```json
{
  "status": "PROCESSING",
  "progress_percentage": 65,
  "capabilities_completed": ["TextRecognition", "SentimentAnalysis"]
}
```

**Wait until**: `status = "COMPLETED"`

---

## 📝 Step 5: Get Enriched Metadata

```bash
curl http://localhost:3000/api/metadata/ASSET_UUID \
  -H "Authorization: Bearer $JWT_TOKEN"
```

**Response**:
```json
{
  "metadata": {
    "enriched_metadata": {
      "ocr_text": ["Detected text"],
      "transcript": "Full transcript...",
      "sentiment": {"overall": "POSITIVE", "score": 0.78},
      "speakers": [...],
      "keywords": [...]
    }
  }
}
```

---

## 🔍 Step 6: Search Assets

```bash
curl -X POST http://localhost:3000/api/graph/search \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "CEO strategy",
    "filters": {"asset_type": "VIDEO"}
  }'
```

---

## 📋 All Endpoints Quick Reference

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/auth/google/login` | GET | Get Google login URL |
| `/api/auth/google/callback` | GET | Get JWT token (auto redirect) |
| `/api/media/upload` | POST | Upload media file |
| `/api/media/{id}` | GET | Get media info |
| `/api/media/{id}/download` | GET | Download media |
| `/api/metadata/{id}` | GET | Get metadata |
| `/api/metadata/{id}` | PUT | Update metadata |
| `/api/workflow/status/{id}` | GET | Check processing status |
| `/api/graph/search` | POST | Search assets |
| `/api/admin/controllers/status` | GET | Monitor controllers |

---

## 🎯 Complete Flow Example

```bash
# 1. Get JWT Token
TOKEN=$(curl -s http://localhost:3000/api/auth/google/login | jq -r '.redirect_url')
# Open $TOKEN in browser, get JWT

# 2. Upload
RESPONSE=$(curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer $JWT" \
  -F "file=@test.mp4" \
  -F "title=Test")

ASSET_ID=$(echo $RESPONSE | jq -r '.asset_uuid')

# 3. Check Status
curl http://localhost:3000/api/workflow/status/$ASSET_ID \
  -H "Authorization: Bearer $JWT"

# 4. Get Metadata (after processing)
curl http://localhost:3000/api/metadata/$ASSET_ID \
  -H "Authorization: Bearer $JWT"
```

---

## 📖 Full Documentation

See `COMPLETE_API_FLOW_GUIDE.md` for detailed step-by-step instructions.

---

**Ready to use!** 🚀

