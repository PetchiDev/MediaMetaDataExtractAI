# 🔄 Complete API Flow Documentation - Step by Step

## 📋 Table of Contents
1. [Authentication Flow](#1-authentication-flow)
2. [Media Upload/Submission Flow](#2-media-uploadsubmission-flow)
3. [Metadata Management Flow](#3-metadata-management-flow)
4. [Workflow Processing Flow](#4-workflow-processing-flow)
5. [Graph Search Flow](#5-graph-search-flow)
6. [Admin/Monitoring Flow](#6-adminmonitoring-flow)
7. [Ingress/Egress Controller Flow](#7-ingressegress-controller-flow)

---

## 1. Authentication Flow

### 1.1 SSO Login (I-FR-21)
**Endpoint**: `POST /api/auth/sso/login`

**Step-by-Step Flow:**
```
1. User clicks "Login with SSO" in UI
   ↓
2. Frontend calls POST /api/auth/sso/login
   ↓
3. Backend generates SSO redirect URL with RelayState
   ↓
4. Frontend redirects user to SSO provider URL
   ↓
5. User authenticates with SSO provider (SAML/OIDC)
   ↓
6. SSO provider redirects to /api/auth/sso/callback?SAMLResponse=...&RelayState=...
   ↓
7. Backend validates SAML assertion (TODO: Full SAML parsing in production)
   ↓
8. Backend extracts user attributes (email, name, sso_provider_id)
   ↓
9. Backend checks if user exists:
   SELECT * FROM users WHERE sso_provider_id = $1
   ↓
10. If user doesn't exist:
    - Create new user in `users` table
    - Default role: VIEWER
   ↓
11. Update last_login timestamp
   ↓
12. Backend generates JWT access token (24 hour expiry)
   ↓
13. Backend generates JWT refresh token (30 day expiry)
   ↓
14. Returns JWT tokens to frontend
   ↓
15. Frontend stores tokens and uses Bearer token for subsequent requests
```

**Request:**
```json
POST /api/auth/sso/login
{}
```

**Response:**
```json
{
  "redirect_url": "https://login.mediacorp.com/saml/sso?RelayState=uuid",
  "status": "redirect_required",
  "message": "Redirect user to SSO provider"
}
```

**Callback Response:**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "user": {
    "id": "user-uuid",
    "email": "user@mediacorp.com",
    "name": "Test User",
    "role": "VIEWER"
  },
  "status": "success"
}
```

**Note**: JWT tokens are signed with HS256 algorithm using JWT_SECRET from environment.

### 1.2 API Key Generation (I-FR-23)
**Endpoint**: `POST /api/access/keys`

**Step-by-Step Flow:**
```
1. Technical user (Developer) logs in via SSO
   ↓
2. User navigates to API Console
   ↓
3. User calls POST /api/access/keys with:
   {
     "key_name": "Production Integration Key",
     "permissions": ["submit:media", "read:metadata"]
   }
   ↓
4. Backend generates secure random key: "mc_sk_1234567890abcdef..."
   ↓
5. Backend hashes key using SHA-256
   ↓
6. Backend stores hash in `api_keys` table (NEVER stores plaintext)
   ↓
7. Backend returns key (ONLY TIME shown in plaintext)
   ↓
8. User must copy and store securely
```

**Response:**
```json
{
  "api_key": "mc_sk_1234567890abcdef...",
  "key_id": "key-uuid",
  "warning": "Store this key securely. It cannot be retrieved again."
}
```

---

## 2. Media Upload/Submission Flow

### 2.1 Technical User API Submission (I-FR-29)
**Endpoint**: `POST /api/media/submit`

**Step-by-Step Flow:**
```
1. Developer sends POST /api/media/submit with:
   - Multipart form data:
     * file: video.mp4
     * metadata: {"title": "Breaking News", "category": "news"}
     * operational_tags: {"broadcast_priority": "HIGH"}
   - Header: Authorization: ApiKey mc_sk_1234567890abcdef...
   ↓
2. Auth Middleware intercepts request:
   a. Extracts Authorization header
   b. Checks if starts with "ApiKey "
   c. Extracts API key: "mc_sk_1234567890abcdef..."
   d. Hashes key with SHA-256
   e. Queries database:
      SELECT * FROM api_keys WHERE key_hash = $1 AND status = 'ACTIVE'
   f. If found:
      - Updates last_used timestamp
      - Gets user from api_keys.user_id
      - Creates Claims object with user info
      - Adds Claims to request extensions
   g. If not found → Returns 401 Unauthorized
   ↓
3. Handler extracts file from multipart form
   ↓
3. Handler extracts file from multipart form
   ↓
4. Calculate SHA-256 hash of file (I-FR-02)
   ↓
5. Check for duplicate in database:
   SELECT * FROM assets WHERE file_hash = 'calculated_hash'
   ↓
6. If duplicate found:
   → Return 200 with existing asset UUID
   ↓
7. If new file:
   → Upload to S3: s3://mediacorp-ai-api-uploads/api-submissions/2025/11/24/uuid/video.mp4
   ↓
8. Determine asset type from filename (.mp4 = VIDEO, .png = IMAGE, etc.)
   ↓
9. Create asset record in `assets` table:
   INSERT INTO assets (
     uuid, asset_type, asset_name, source_system,
     file_path, file_hash, file_size, status, ...
   ) VALUES (...)
   ↓
10. Preprocessing Service determines workflow (I-FR-33):
    - Check metadata for "interview" → INTERVIEW_WORKFLOW
    - Check category "news" → NEWS_WORKFLOW
    - Check duration > 3600s → LONGFORM_WORKFLOW
    - Default → STANDARD_WORKFLOW
   ↓
11. Create processing job in `processing_jobs` table:
    INSERT INTO processing_jobs (
      job_id, asset_uuid, workflow_name, status, ...
    ) VALUES (...)
   ↓
12. TODO: Trigger AWS Step Functions workflow
   ↓
13. Return 202 Accepted with job_id and status URLs
```

**Response:**
```json
{
  "asset_uuid": "asset-uuid",
  "job_id": "job-uuid",
  "status": "QUEUED",
  "workflow": "NEWS_WORKFLOW",
  "estimated_time_minutes": 30,
  "status_url": "/api/jobs/job-uuid",
  "metadata_url": "/api/metadata/asset-uuid"
}
```

### 2.2 Naive User UI Upload (I-FR-31)
**Endpoint**: `POST /api/media/upload`

**Flow:**
```
Same as 2.1, but:
- User uploads via UI drag-and-drop
- Frontend automatically extracts metadata from form
- User doesn't need to provide operational_tags
- Returns 201 Created instead of 202 Accepted
```

---

## 3. Metadata Management Flow

### 3.1 Get Metadata (I-FR-24)
**Endpoint**: `GET /api/metadata/:asset_id`

**Step-by-Step Flow:**
```
1. User/System calls GET /api/metadata/{asset_uuid}
   Header: Authorization: Bearer eyJhbGciOiJIUzI1NiIs...
   ↓
2. Auth Middleware validates JWT token:
   a. Extracts "Bearer " token
   b. Decodes JWT using JWT_SECRET
   c. Validates signature and expiration
   d. If valid → Adds Claims to request extensions
   e. If invalid/expired → Returns 401 Unauthorized
   ↓
3. Handler extracts Claims from request extensions
   ↓
3. Handler queries database:
   SELECT * FROM assets WHERE uuid = asset_uuid
   ↓
4. If not found → Return 404
   ↓
5. If found → Return enriched_metadata JSONB field
```

**Response:**
```json
{
  "asset_uuid": "asset-uuid",
  "enriched_metadata": {
    "title": "Breaking News",
    "description": "...",
    "tags": ["news", "breaking"],
    "ai_keywords": ["election", "politics"],
    "sentiment": "POSITIVE",
    "speakers": [...]
  },
  "version": 2,
  "version_id": "version-uuid",
  "updated_at": "2025-11-24T10:00:00Z"
}
```

### 3.2 Update Metadata (I-FR-27, I-FR-19)
**Endpoint**: `PUT /api/metadata/:asset_id`

**Step-by-Step Flow:**
```
1. User calls PUT /api/metadata/{asset_uuid} with:
   {
     "title": "Updated Title",
     "description": "New description",
     "tags": ["tag1", "tag2"]
   }
   Header: X-Version-ID: previous-version-id
   ↓
2. Handler gets current asset from database
   ↓
3. Conflict Detection (I-FR-19):
   - Compare X-Version-ID header with current asset.version_id
   - If mismatch → Conflict detected!
   - Return 409 Conflict with conflict details
   ↓
4. If no conflict:
   - Create new version (I-FR-18):
     INSERT INTO asset_versions (
       asset_uuid, version, version_id, metadata_snapshot, ...
     ) VALUES (current_version_data)
   ↓
5. Merge metadata updates:
   updated_metadata = current_metadata.merge(new_updates)
   ↓
6. Update asset:
   UPDATE assets SET
     enriched_metadata = updated_metadata,
     version = version + 1,
     version_id = new_version_id,
     updated_at = NOW()
   WHERE uuid = asset_uuid
   ↓
7. Update graph database (I-FR-20):
   - Extract keywords from tags
   - Index in graph_nodes and asset_graph_nodes tables
   ↓
8. Return success with new version
```

**Conflict Response (409):**
```json
{
  "error": "Metadata conflict detected",
  "conflict_detected": true,
  "your_version": "old-version-id",
  "current_version": "new-version-id",
  "requires_manual_review": true
}
```

### 3.3 Resolve Conflict (I-FR-19)
**Endpoint**: `POST /api/metadata/:asset_id/resolve-conflict`

**Step-by-Step Flow:**
```
1. User receives conflict response (409)
   ↓
2. UI shows conflict resolution dialog
   ↓
3. User selects resolution strategy:
   - "merge" - Combine both changes
   - "yours" - Keep only user's changes
   - "theirs" - Keep only other user's changes
   ↓
4. User calls POST /api/metadata/{asset_uuid}/resolve-conflict with:
   {
     "resolution_strategy": "merge",
     "resolved_metadata": {...}
   }
   ↓
5. Handler creates new version with resolved metadata
   ↓
6. Updates asset with resolved data
   ↓
7. Returns success
```

---

## 4. Workflow Processing Flow

### 4.1 Get Workflow Status (I-FR-26)
**Endpoint**: `GET /api/workflow/status/:asset_id`

**Step-by-Step Flow:**
```
1. User calls GET /api/workflow/status/{asset_uuid}
   ↓
2. Handler queries processing_jobs table:
   SELECT * FROM processing_jobs 
   WHERE asset_uuid = asset_uuid 
   ORDER BY created_at DESC 
   LIMIT 1
   ↓
3. If job found:
   - Return job status, progress, completed capabilities
   ↓
4. If not found:
   - Return "NOT_FOUND"
```

**Response:**
```json
{
  "asset_uuid": "asset-uuid",
  "job_id": "job-uuid",
  "workflow_name": "NEWS_WORKFLOW",
  "status": "PROCESSING",
  "progress_percentage": 65,
  "capabilities_completed": ["SpeakerIndexing", "TextRecognition"],
  "capabilities_failed": [],
  "error_message": null
}
```

### 4.2 Retry Failed Job (I-FR-16)
**Endpoint**: `POST /api/jobs/:job_id/retry`

**Step-by-Step Flow:**
```
1. Admin sees failed job in dashboard
   ↓
2. Admin calls POST /api/jobs/{job_id}/retry with:
   {
     "strategy": "chunked",
     "chunk_size": 1800,
     "max_retries": 3
   }
   ↓
3. Handler updates job:
   UPDATE processing_jobs SET
     status = 'RETRYING',
     retry_count = retry_count + 1,
     retry_config = retry_config_json
   WHERE job_id = job_id
   ↓
4. TODO: Trigger Step Functions with retry config
   ↓
5. Return success
```

### 4.3 Create New Workflow (I-FR-32)
**Endpoint**: `POST /api/workflows`

**Step-by-Step Flow:**
```
1. Admin creates new workflow definition:
   POST /api/workflows
   {
     "workflow_name": "CUSTOM_WORKFLOW",
     "description": "Custom processing workflow",
     "step_functions_arn": "arn:aws:states:...",
     "preprocessing_logic": {
       "conditions": [...]
     },
     "ai_capabilities": ["ObjectDetection", "BrandDetection"]
   }
   ↓
2. Handler creates workflow definition:
   INSERT INTO workflow_definitions (
     workflow_id, workflow_name, step_functions_arn,
     preprocessing_logic, ai_capabilities, ...
   ) VALUES (...)
   ↓
3. Returns workflow_id
```

---

## 5. Graph Search Flow

### 5.1 Graph-Based Search (I-FR-20, I-FR-22)
**Endpoint**: `POST /api/graph/search`

**Step-by-Step Flow:**
```
1. User searches: "CEO strategy 2025"
   ↓
2. Frontend calls POST /api/graph/search with:
   {
     "query": "CEO strategy 2025",
     "filters": {
       "asset_type": "VIDEO",
       "relationship_type": "shared_keywords"
     },
     "max_depth": 2
   }
   ↓
3. Handler extracts keywords: ["CEO", "strategy", "2025"]
   ↓
4. Query graph database (PostgreSQL graph tables):
   SELECT a.*, related.*, tags, topics
   FROM assets a
   LEFT JOIN graph_relationships gr ON a.uuid = gr.source_asset_uuid
   LEFT JOIN asset_graph_nodes agn ON a.uuid = agn.asset_uuid
   LEFT JOIN graph_nodes gn ON agn.node_id = gn.node_id
   WHERE 
     (a.title LIKE '%CEO%' OR a.description LIKE '%CEO%')
     AND a.type = 'VIDEO'
   ↓
5. Find related assets through relationships:
   - Shared keywords
   - Shared topics
   - Temporal proximity
   - Same contributor
   ↓
6. Build graph structure:
   {
     "nodes": [
       {"id": "asset-1", "label": "Interview with CEO", "type": "VIDEO"},
       {"id": "asset-2", "label": "Q4 Results", "type": "TEXT"}
     ],
     "edges": [
       {"from": "asset-1", "to": "asset-2", "label": "shared_keyword"}
     ]
   }
   ↓
7. Return assets + graph visualization data
```

**Response:**
```json
{
  "assets": [
    {
      "uuid": "asset-uuid",
      "name": "Interview_with_CEO.mp4",
      "tags": ["CEO", "strategy"],
      "topics": ["Business Strategy"]
    }
  ],
  "graph": {
    "nodes": [...],
    "edges": [...]
  },
  "total_results": 5
}
```

---

## 6. Admin/Monitoring Flow

### 6.1 Controller Status (I-FR-09)
**Endpoint**: `GET /api/controllers/status`

**Step-by-Step Flow:**
```
1. Admin calls GET /api/controllers/status
   ↓
2. Handler queries action_records for each controller:
   SELECT 
     controller_name,
     COUNT(*) as total,
     COUNT(CASE WHEN status = 'SUCCESS' THEN 1 END) as success_count,
     AVG(processing_time_ms) as avg_time,
     MAX(created_at) as last_execution
   FROM action_records
   WHERE controller_name IN ('BrightcoveIngress', 'CloudinaryIngress', ...)
     AND created_at > NOW() - INTERVAL '24 hours'
   GROUP BY controller_name
   ↓
3. Calculate success rate: (success_count / total) * 100
   ↓
4. Determine health status:
   - success_rate < 90% → FAILED
   - success_rate < 95% → DEGRADED
   - else → ACTIVE
   ↓
5. Return status for each controller
```

**Response:**
```json
{
  "controllers": [
    {
      "controller_name": "BrightcoveIngress",
      "version": "v2.3.1",
      "status": "ACTIVE",
      "last_execution": "2025-11-24T10:15:00Z",
      "success_rate_24h": 99.2,
      "avg_processing_time_ms": 3200
    }
  ]
}
```

### 6.2 Action Records (I-FR-05)
**Endpoint**: `GET /api/audit/actions`

**Step-by-Step Flow:**
```
1. Admin calls GET /api/audit/actions?asset_id=uuid&limit=100
   ↓
2. Handler queries action_records:
   SELECT * FROM action_records
   WHERE asset_uuid = $1 (if provided)
   ORDER BY timestamp DESC
   LIMIT $2
   ↓
3. Returns audit trail
```

### 6.3 Rollback Asset (I-FR-13)
**Endpoint**: `POST /api/rollback/:asset_id/:version_id`

**Step-by-Step Flow:**
```
1. Admin selects asset and version to rollback
   ↓
2. Admin calls POST /api/rollback/{asset_uuid}/{version_id}
   ↓
3. Handler gets version snapshot:
   SELECT metadata_snapshot FROM asset_versions
   WHERE asset_uuid = $1 AND version_id = $2
   ↓
4. Create new version with current state
   ↓
5. Restore asset to previous version:
   UPDATE assets SET
     enriched_metadata = version_snapshot,
     version = version + 1,
     version_id = new_version_id
   WHERE uuid = asset_uuid
   ↓
6. Log rollback action in action_records
   ↓
7. Return success
```

---

## 7. Ingress/Egress Controller Flow

### 7.1 Ingress Controller (I-FR-01, I-FR-02, I-FR-05)
**Background Process (Runs every 15 minutes)**

**Step-by-Step Flow:**
```
1. Scheduled trigger (CloudWatch Events / Cron)
   ↓
2. Ingress Controller wakes up:
   - BrightcoveIngressController
   - CloudinaryIngressController
   - OmnystudioIngressController
   - etc.
   ↓
3. Get last sync timestamp from controller_configs:
   SELECT last_sync_at FROM controller_configs
   WHERE controller_name = 'BrightcoveIngress'
   ↓
4. Call external API to get new/modified assets:
   GET https://cms.api.brightcove.com/v1/videos?since={last_sync}
   ↓
5. For each asset:
   a. Download file from external system
   b. Calculate SHA-256 hash (I-FR-02)
   c. Check for duplicate:
      SELECT * FROM assets WHERE file_hash = hash
   d. If duplicate:
      - Check if metadata changed
      - If changed → Create new version
      - If not → Skip
   e. If new:
      - Upload to S3 staging bucket
      - Create asset record in database
   ↓
6. Generate action record (I-FR-05):
   INSERT INTO action_records (
     asset_uuid, action_type, direction,
     controller_name, controller_version,
     source_system, destination_system,
     status, timestamp, metadata
   ) VALUES ('INGRESS', 'INBOUND', ...)
   ↓
7. Trigger preprocessing workflow (I-FR-33)
   ↓
8. Update last_sync_at:
   UPDATE controller_configs SET
     last_sync_at = NOW()
   WHERE controller_name = 'BrightcoveIngress'
```

### 7.2 Egress Controller (I-FR-01, I-FR-05)
**Background Process (Runs every 15 minutes)**

**Step-by-Step Flow:**
```
1. Scheduled trigger
   ↓
2. Egress Controller wakes up:
   - BrightcoveEgressController
   - CloudinaryEgressController
   - etc.
   ↓
3. Query processed assets ready for egress:
   SELECT * FROM assets
   WHERE status = 'PROCESSED'
     AND source_system = 'BRIGHTCOVE'
     AND last_egress_sync < processing_completed_at
   ↓
4. For each asset:
   a. Format enriched metadata for external system
   b. Call external API to update:
      PATCH https://cms.api.brightcove.com/v1/videos/{source_id}
      Body: {
        "custom_fields": {
          "ai_keywords": "...",
          "ai_sentiment": "...",
          "ai_transcript": "..."
        }
      }
   c. If successful:
      - Update last_egress_sync timestamp
   ↓
5. Generate action record (I-FR-05):
   INSERT INTO action_records (
     action_type = 'EGRESS',
     direction = 'OUTBOUND',
     ...
   )
```

---

## 🔄 Complete End-to-End Flow Example

### Scenario: User Uploads Video → AI Processing → Metadata Update

```
Step 1: User Uploads Video
POST /api/media/upload
→ File uploaded to S3
→ Asset created in database (status: QUEUED)
→ Processing job created
→ Returns: asset_uuid, job_id

Step 2: Preprocessing Determines Workflow
→ Preprocessing service analyzes metadata
→ Routes to NEWS_WORKFLOW (category = "news")
→ Step Functions workflow triggered

Step 3: AI Processing (Parallel)
→ SpeakerIndexing Lambda runs
→ TextRecognition Lambda runs
→ SentimentAnalysis Lambda runs
→ All update processing_jobs table

Step 4: User Checks Status
GET /api/workflow/status/{asset_uuid}
→ Returns: status = "PROCESSING", progress = 65%

Step 5: Processing Completes
→ Step Functions updates job status = "COMPLETED"
→ Asset status updated to "PROCESSED"
→ Enriched metadata stored in assets.enriched_metadata

Step 6: User Views Metadata
GET /api/metadata/{asset_uuid}
→ Returns enriched metadata with AI results

Step 7: User Edits Metadata
PUT /api/metadata/{asset_uuid}
→ Version control creates new version
→ Graph database updated with new keywords

Step 8: Egress Syncs Back
→ Egress controller runs (every 15 min)
→ Pushes enriched metadata back to Brightcove
→ Action record logged
```

---

## 📊 API Endpoint Summary

### Authentication (7 endpoints) - **PUBLIC (No Auth Required)**
- `POST /api/auth/sso/login` - Initiate SSO login (returns redirect URL)
- `GET /api/auth/sso/callback` - SSO callback (validates SAML, returns JWT)
- `POST /api/access/keys` - Generate API key (requires auth)
- `GET /api/access/keys` - List API keys (requires auth)
- `DELETE /api/access/keys/:key_id` - Revoke API key (requires auth)
- `GET /api/access/permissions/:user_id` - Get user permissions (requires auth)
- `GET /api/ratelimit/config` - Get rate limit config (requires auth)
- `PUT /api/ratelimit/config` - Update rate limit config (requires auth)

### Media (3 endpoints) - **PROTECTED (Requires Auth)**
- `POST /api/media/submit` - Technical user submission
  - Auth: Bearer JWT or ApiKey
- `POST /api/media/upload` - Naive user upload
  - Auth: Bearer JWT (from SSO)
- `GET /api/media/:asset_id` - Get asset info
  - Auth: Bearer JWT or ApiKey

### Metadata (3 endpoints) - **PROTECTED (Requires Auth)**
- `GET /api/metadata/:asset_id` - Get metadata
  - Auth: Bearer JWT or ApiKey
- `PUT /api/metadata/:asset_id` - Update metadata
  - Auth: Bearer JWT or ApiKey
  - Header: X-Version-ID (for conflict detection)
- `POST /api/metadata/:asset_id/resolve-conflict` - Resolve conflict
  - Auth: Bearer JWT or ApiKey

### Workflow (5 endpoints) - **PROTECTED (Requires Auth)**
- `GET /api/workflow/status/:asset_id` - Get workflow status
  - Auth: Bearer JWT or ApiKey
- `GET /api/jobs/:job_id/status` - Get job status
  - Auth: Bearer JWT or ApiKey
- `POST /api/jobs/:job_id/retry` - Retry failed job
  - Auth: Bearer JWT (Admin role required)
- `POST /api/workflows` - Create workflow
  - Auth: Bearer JWT (Admin role required)
- `GET /api/workflows` - List workflows
  - Auth: Bearer JWT or ApiKey

### Graph (2 endpoints) - **PROTECTED (Requires Auth)**
- `POST /api/graph/search` - Graph search
  - Auth: Bearer JWT or ApiKey
- `POST /api/graph/relationships` - Get relationships
  - Auth: Bearer JWT or ApiKey

### Admin (10 endpoints) - **PROTECTED (Requires Auth - Admin Role)**
- `GET /api/controllers/status` - Controller status
  - Auth: Bearer JWT (Admin role)
- `GET /api/controllers/metrics` - Controller metrics
  - Auth: Bearer JWT (Admin role)
- `GET /api/audit/actions` - Action records
  - Auth: Bearer JWT (Admin role)
- `GET /api/audit/actions/:asset_id` - Asset actions
  - Auth: Bearer JWT (Admin role)
- `POST /api/rollback/:asset_id/:version_id` - Rollback
  - Auth: Bearer JWT (Admin role)
- `GET /api/config/sync-interval` - Get sync config
  - Auth: Bearer JWT (Admin role)
- `PUT /api/config/sync-interval` - Update sync config
  - Auth: Bearer JWT (Admin role)
- `PUT /api/config/logging-level` - Update logging
  - Auth: Bearer JWT (Admin role)
- `PUT /api/config/retry` - Update retry config
  - Auth: Bearer JWT (Admin role)
- `GET /api/lifecycle/rules` - Lifecycle rules
  - Auth: Bearer JWT (Admin role)
- `POST /api/lifecycle/rules` - Create lifecycle rule
  - Auth: Bearer JWT (Admin role)

## 🔐 Authentication Methods

### Method 1: JWT Bearer Token (SSO Users)
```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```
- Obtained via SSO login flow
- Valid for 24 hours
- Contains: user_id, email, role, exp

### Method 2: API Key (Technical Users)
```
Authorization: ApiKey mc_sk_1234567890abcdef...
```
- Generated via `/api/access/keys` endpoint
- Stored as SHA-256 hash in database
- Never returned again after generation
- Valid until revoked

**Total: 26 API Endpoints** 🎯
- **7 Public endpoints** (auth routes, swagger)
- **19 Protected endpoints** (require authentication)

