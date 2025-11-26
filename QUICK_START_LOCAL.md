# 🚀 Quick Start - Local Testing

## Complete Local File Upload & AI Processing Flow

### 1. Setup Environment

```bash
# Create .env file
cat > .env << EOF
DATABASE_URL=postgresql://postgres:password@localhost/mediacorp
USE_LOCAL_STORAGE=true
LOCAL_STORAGE_PATH=./local_storage
JWT_SECRET=local-testing-secret-key
EOF
```

### 2. Run Migrations

```bash
sqlx migrate run
```

### 3. Start Server

```bash
cargo run
```

### 4. Test Upload (All File Types)

#### Video Upload
```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -F "file=@test_video.mp4" \
  -F "metadata={\"title\":\"Test Video\",\"category\":\"news\"}"
```

#### Audio Upload
```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -F "file=@test_audio.mp3" \
  -F "metadata={\"title\":\"Test Audio\",\"category\":\"podcast\"}"
```

#### Text Upload
```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -F "file=@test_document.pdf" \
  -F "metadata={\"title\":\"Test Document\",\"category\":\"article\"}"
```

#### Image Upload
```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -F "file=@test_image.png" \
  -F "metadata={\"title\":\"Test Image\",\"category\":\"photo\"}"
```

### 5. What Happens Automatically

1. ✅ File saved to `./local_storage/uploads/YYYY/MM/DD/filename`
2. ✅ Asset created in `assets` table with:
   - Asset type (VIDEO/AUDIO/IMAGE/TEXT)
   - File hash (for deduplication)
   - File path
   - Metadata
3. ✅ Processing job created in `processing_jobs` table
4. ✅ AI processing runs in background:
   - **OCR extraction** (Video/Image)
   - **Transcript extraction** (Audio/Video)
   - **Sentiment analysis** (Audio/Video)
   - **Speaker detection** (Audio/Video)
5. ✅ Enriched metadata stored in `assets.enriched_metadata` JSONB field
6. ✅ Action record logged in `action_records` table
7. ✅ Keywords indexed in `graph_nodes` and `asset_graph_nodes` tables

### 6. Check Results

#### Get Asset Info
```bash
curl http://localhost:3000/api/media/{asset_uuid} \
  -H "Authorization: Bearer YOUR_TOKEN"
```

#### Get Enriched Metadata
```bash
curl http://localhost:3000/api/metadata/{asset_uuid} \
  -H "Authorization: Bearer YOUR_TOKEN"
```

#### Check Processing Status
```bash
curl http://localhost:3000/api/workflow/status/{asset_uuid} \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### 7. Database Queries

#### View All Assets
```sql
SELECT 
    uuid,
    asset_type,
    asset_name,
    status,
    enriched_metadata->'sentiment' as sentiment,
    enriched_metadata->'transcript' as transcript,
    enriched_metadata->'ocr_results' as ocr,
    created_at
FROM assets
ORDER BY created_at DESC;
```

#### View Processing Jobs
```sql
SELECT 
    job_id,
    asset_uuid,
    workflow_name,
    status,
    progress_percentage,
    capabilities_completed,
    created_at,
    completed_at
FROM processing_jobs
ORDER BY created_at DESC;
```

#### View Action Records
```sql
SELECT 
    action_type,
    direction,
    controller_name,
    status,
    timestamp
FROM action_records
ORDER BY timestamp DESC
LIMIT 10;
```

## All Data Stored in Database Tables ✅

- ✅ **assets** - File info, metadata, enriched data
- ✅ **processing_jobs** - Job status, progress
- ✅ **action_records** - Audit trail
- ✅ **graph_nodes** - Keywords, topics
- ✅ **asset_graph_nodes** - Asset-keyword relationships
- ✅ **asset_versions** - Version history

## AI Features Implemented ✅

- ✅ **OCR Extraction** - From video frames and images
- ✅ **Transcript Extraction** - From audio and video
- ✅ **Sentiment Analysis** - From transcript text
- ✅ **Speaker Detection** - From audio/video
- ✅ **Keyword Extraction** - From transcript and OCR

## Local Storage Structure

```
local_storage/
└── uploads/
    └── 2025/
        └── 11/
            └── 24/
                ├── video.mp4
                ├── audio.mp3
                ├── document.pdf
                └── image.png
```

All files stored locally, no S3 needed! 🎉

