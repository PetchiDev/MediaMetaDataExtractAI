# 🚀 Complete API Flow Guide - Step by Step

## Overview

This guide shows you how to use **ALL** API endpoints in the correct order.

## 📋 Table of Contents

1. [Authentication Flow](#1-authentication-flow)
2. [Media Upload Flow](#2-media-upload-flow)
3. [Metadata Management Flow](#3-metadata-management-flow)
4. [Workflow Status Flow](#4-workflow-status-flow)
5. [Graph Search Flow](#5-graph-search-flow)
6. [Admin Monitoring Flow](#6-admin-monitoring-flow)

---

## 1. Authentication Flow

### Step 1.1: Google Sign-In (Get Redirect URL)

**Endpoint**: `GET /api/auth/google/login`

**Description**: Get Google OAuth redirect URL

**Request**:
```bash
curl http://localhost:3000/api/auth/google/login
```

**Response**:
```json
{
  "redirect_url": "https://accounts.google.com/o/oauth2/v2/auth?...",
  "status": "redirect_required",
  "message": "Redirect user to Google Sign-In"
}
```

**Action**: Open `redirect_url` in browser

---

### Step 1.2: Google Callback (Get JWT Token)

**Endpoint**: `GET /api/auth/google/callback?code=...&state=...`

**Description**: Google redirects here after user signs in. Returns JWT token.

**Request**: (Automatic redirect from Google)
```
http://localhost:3000/api/auth/google/callback?code=AUTHORIZATION_CODE&state=CSRF_TOKEN
```

**Response**:
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "user": {
    "id": "user-uuid",
    "email": "user@gmail.com",
    "name": "User Name",
    "role": "VIEWER",
    "picture": "https://..."
  },
  "status": "success"
}
```

**Save**: Copy `access_token` for next steps

---

### Step 1.3: Generate API Key (Alternative to JWT)

**Endpoint**: `POST /api/auth/api-keys`

**Description**: Generate API key for technical users

**Request**:
```bash
curl -X POST http://localhost:3000/api/auth/api-keys \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "key_name": "My API Key",
    "permissions": ["submit:media", "read:metadata"]
  }'
```

**Response**:
```json
{
  "api_key": "mc_sk_1234567890abcdef...",
  "key_id": "key-uuid",
  "expires_in": "never",
  "warning": "Store this key securely. It cannot be retrieved again."
}
```

**Save**: Copy `api_key` (shown only once!)

---

## 2. Media Upload Flow

### Option A: Technical User (API Submission)

### Step 2.1: Submit Media via API

**Endpoint**: `POST /api/media/submit`

**Description**: Submit media file with metadata for AI processing

**Request**:
```bash
curl -X POST http://localhost:3000/api/media/submit \
  -H "Authorization: ApiKey YOUR_API_KEY" \
  -F "file=@video.mp4" \
  -F "metadata={\"title\":\"My Video\",\"description\":\"Test video\",\"category\":\"news\"}" \
  -F "operational_tags={\"broadcast_priority\":\"HIGH\",\"target_platforms\":[\"web\",\"mobile\"]}"
```

**Response**:
```json
{
  "asset_uuid": "asset-uuid",
  "job_id": "job-uuid",
  "status": "QUEUED",
  "estimated_time_minutes": 30,
  "status_url": "/api/workflow/status/job-uuid",
  "metadata_url": "/api/metadata/asset-uuid"
}
```

**Save**: `asset_uuid` and `job_id`

---

### Option B: Naive User (UI Upload)

### Step 2.2: Upload Media via UI

**Endpoint**: `POST /api/media/upload`

**Description**: Simple upload for non-technical users

**Request**:
```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -F "file=@image.jpg" \
  -F "title=My Image" \
  -F "description=Test image" \
  -F "tags=tag1,tag2" \
  -F "category=news"
```

**Response**:
```json
{
  "asset_uuid": "asset-uuid",
  "status": "PROCESSING",
  "message": "Upload successful. AI processing started."
}
```

---

### Step 2.3: Get Media Information

**Endpoint**: `GET /api/media/{asset_id}`

**Description**: Get media asset details

**Request**:
```bash
curl http://localhost:3000/api/media/ASSET_UUID \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

**Response**:
```json
{
  "uuid": "asset-uuid",
  "asset_type": "VIDEO",
  "asset_name": "video.mp4",
  "source_system": "API_SUBMISSION",
  "status": "PROCESSED",
  "file_path": "s3://bucket/path/video.mp4",
  "created_at": "2025-01-20T10:00:00Z"
}
```

---

### Step 2.4: Download Media File

**Endpoint**: `GET /api/media/{asset_id}/download`

**Description**: Download or stream media file

**Request**:
```bash
curl http://localhost:3000/api/media/ASSET_UUID/download \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  --output video.mp4
```

**Response**: Binary file stream

---

## 3. Metadata Management Flow

### Step 3.1: Get Enriched Metadata

**Endpoint**: `GET /api/metadata/{asset_id}`

**Description**: Get all AI-enriched metadata

**Request**:
```bash
curl http://localhost:3000/api/metadata/ASSET_UUID \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

**Response**:
```json
{
  "asset_uuid": "asset-uuid",
  "metadata": {
    "title": "My Video",
    "description": "Test video",
    "enriched_metadata": {
      "ocr_text": ["Text detected"],
      "transcript": "Full transcript...",
      "sentiment": {
        "overall": "POSITIVE",
        "score": 0.78
      },
      "speakers": [
        {"speaker_id": "Speaker_1", "segments": [[0, 120]]}
      ],
      "keywords": ["keyword1", "keyword2"]
    }
  },
  "version": 1
}
```

---

### Step 3.2: Update Metadata

**Endpoint**: `PUT /api/metadata/{asset_id}`

**Description**: Update asset metadata (with conflict detection)

**Request**:
```bash
curl -X PUT http://localhost:3000/api/metadata/ASSET_UUID \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -H "X-Version-ID: version-id" \
  -d '{
    "title": "Updated Title",
    "description": "Updated description",
    "tags": ["tag1", "tag2", "tag3"]
  }'
```

**Response** (Success):
```json
{
  "status": "success",
  "version": 2,
  "conflict_detected": false,
  "message": "Metadata updated successfully"
}
```

**Response** (Conflict):
```json
{
  "error": "Conflict detected",
  "conflict_detected": true,
  "conflicts": {
    "your_version": "version-v1",
    "current_version": "version-v2",
    "your_changes": {...},
    "their_changes": {...}
  }
}
```

---

### Step 3.3: Resolve Metadata Conflict

**Endpoint**: `POST /api/metadata/{asset_id}/resolve-conflict`

**Description**: Resolve concurrent metadata updates

**Request**:
```bash
curl -X POST http://localhost:3000/api/metadata/ASSET_UUID/resolve-conflict \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "resolved_metadata": {
      "title": "Resolved Title",
      "description": "Resolved description"
    },
    "resolution_strategy": "MERGE"
  }'
```

**Response**:
```json
{
  "status": "success",
  "version": 3,
  "message": "Conflict resolved successfully"
}
```

---

## 4. Workflow Status Flow

### Step 4.1: Check Workflow Status

**Endpoint**: `GET /api/workflow/status/{asset_id}`

**Description**: Get AI processing workflow status

**Request**:
```bash
curl http://localhost:3000/api/workflow/status/ASSET_UUID \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

**Response**:
```json
{
  "job_id": "job-uuid",
  "asset_uuid": "asset-uuid",
  "status": "PROCESSING",
  "progress_percentage": 65,
  "workflow_name": "STANDARD_WORKFLOW",
  "capabilities_completed": [
    "TextRecognition",
    "SentimentAnalysis"
  ],
  "capabilities_in_progress": [
    "SpeakerIndexing"
  ],
  "capabilities_pending": [
    "BrandDetection"
  ],
  "estimated_completion": "2025-01-20T10:30:00Z"
}
```

**Status Values**:
- `QUEUED` - Waiting to start
- `PROCESSING` - Currently processing
- `COMPLETED` - All AI tasks done
- `FAILED` - Processing failed

---

## 5. Graph Search Flow

### Step 5.1: Search Assets Using Graph

**Endpoint**: `POST /api/graph/search`

**Description**: Search assets using graph-based relationships

**Request**:
```bash
curl -X POST http://localhost:3000/api/graph/search \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "CEO strategy 2025",
    "filters": {
      "asset_type": "VIDEO",
      "relationship_type": "shared_keywords"
    },
    "include_relationships": true,
    "max_depth": 2
  }'
```

**Response**:
```json
{
  "assets": [
    {
      "uuid": "asset-uuid-1",
      "name": "Interview_with_CEO.mp4",
      "type": "VIDEO",
      "title": "CEO Interview",
      "tags": ["CEO", "strategy", "2025"],
      "topics": ["Business Strategy"]
    }
  ],
  "graph": {
    "nodes": [
      {"id": "asset-uuid-1", "label": "CEO Interview", "type": "VIDEO"}
    ],
    "edges": [
      {"from": "asset-uuid-1", "to": "asset-uuid-2", "label": "shared_keyword"}
    ]
  },
  "total_results": 1
}
```

---

## 6. Admin Monitoring Flow

### Step 6.1: Get Controller Status

**Endpoint**: `GET /api/admin/controllers/status`

**Description**: Monitor ingress/egress controller health

**Request**:
```bash
curl http://localhost:3000/api/admin/controllers/status \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

**Response**:
```json
{
  "controllers": [
    {
      "controller_name": "BrightcoveIngress",
      "version": "v2.3.1",
      "status": "ACTIVE",
      "last_execution": "2025-01-20T10:00:00Z",
      "success_rate_24h": 99.2,
      "avg_processing_time_ms": 3200
    },
    {
      "controller_name": "CloudinaryEgress",
      "status": "DEGRADED",
      "success_rate_24h": 87.5,
      "avg_processing_time_ms": 12400
    }
  ],
  "timestamp": "2025-01-20T10:15:00Z"
}
```

---

## 🔄 Complete End-to-End Flow Example

### Scenario: Upload Video → Process → Get Metadata → Search

```bash
# 1. Authenticate
TOKEN=$(curl -s http://localhost:3000/api/auth/google/login | jq -r '.redirect_url')
# Open $TOKEN in browser, get JWT from callback

# 2. Upload Video
UPLOAD_RESPONSE=$(curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -F "file=@video.mp4" \
  -F "title=Test Video" \
  -F "category=news")

ASSET_UUID=$(echo $UPLOAD_RESPONSE | jq -r '.asset_uuid')

# 3. Check Workflow Status
curl http://localhost:3000/api/workflow/status/$ASSET_UUID \
  -H "Authorization: Bearer $JWT_TOKEN"

# 4. Wait for Processing (poll every 30 seconds)
while true; do
  STATUS=$(curl -s http://localhost:3000/api/workflow/status/$ASSET_UUID \
    -H "Authorization: Bearer $JWT_TOKEN" | jq -r '.status')
  
  if [ "$STATUS" = "COMPLETED" ]; then
    break
  fi
  sleep 30
done

# 5. Get Enriched Metadata
curl http://localhost:3000/api/metadata/$ASSET_UUID \
  -H "Authorization: Bearer $JWT_TOKEN"

# 6. Search Related Assets
curl -X POST http://localhost:3000/api/graph/search \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"query\": \"test video\"}"
```

---

## 🔐 Authentication Methods

### Method 1: JWT Token (Recommended)
```bash
Authorization: Bearer eyJhbGciOiJIUzI1NiIs...
```

### Method 2: API Key
```bash
Authorization: ApiKey mc_sk_1234567890abcdef...
```

---

## 📝 Quick Reference

| Endpoint | Method | Auth | Purpose |
|----------|--------|------|---------|
| `/api/auth/google/login` | GET | No | Get Google login URL |
| `/api/auth/google/callback` | GET | No | Get JWT token |
| `/api/auth/api-keys` | POST | JWT | Generate API key |
| `/api/media/submit` | POST | API Key | Submit media (API) |
| `/api/media/upload` | POST | JWT | Upload media (UI) |
| `/api/media/{id}` | GET | JWT/API Key | Get media info |
| `/api/media/{id}/download` | GET | JWT/API Key | Download media |
| `/api/metadata/{id}` | GET | JWT/API Key | Get metadata |
| `/api/metadata/{id}` | PUT | JWT/API Key | Update metadata |
| `/api/metadata/{id}/resolve-conflict` | POST | JWT/API Key | Resolve conflict |
| `/api/workflow/status/{id}` | GET | JWT/API Key | Get workflow status |
| `/api/graph/search` | POST | JWT/API Key | Graph search |
| `/api/admin/controllers/status` | GET | JWT | Controller health |

---

## 🎯 Next Steps

1. **Start Server**: `cargo run`
2. **Open Swagger UI**: `http://localhost:3000/swagger-ui`
3. **Test Authentication**: Use Google Sign-In
4. **Upload Media**: Test file upload
5. **Check Status**: Monitor workflow progress
6. **Explore Metadata**: View enriched data

---

**All endpoints are now documented in Swagger UI!** 🎉

