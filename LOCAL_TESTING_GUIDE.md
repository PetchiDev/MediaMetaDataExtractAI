# 🧪 Local Testing Guide - Complete Flow

## Overview
This guide shows how to test the complete flow locally:
1. Upload files (Audio, Video, Text, Image)
2. Store in local storage (no S3 needed)
3. AI processing (OCR, Sentiment Analysis)
4. Store all metadata in database
5. Ingress/Egress functionality

## Setup

### 1. Environment Variables
Create `.env` file:
```bash
# Database
DATABASE_URL=postgresql://user:password@localhost/mediacorp

# Use local storage instead of S3
USE_LOCAL_STORAGE=true
LOCAL_STORAGE_PATH=./local_storage

# JWT Secret
JWT_SECRET=your-secret-key-for-local-testing

# Optional: SSO (for testing)
SSO_PROVIDER_URL=https://login.mediacorp.com/saml/sso
```

### 2. Run Database Migrations
```bash
# Make sure PostgreSQL is running
sqlx migrate run
```

### 3. Start the Server
```bash
cargo run
```

Server will start on `http://localhost:3000`

## Testing Flow

### Step 1: Upload File via API

#### Upload Video File
```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -F "file=@/path/to/video.mp4" \
  -F "metadata={\"title\":\"Test Video\",\"category\":\"news\",\"description\":\"Test video for AI processing\"}"
```

#### Upload Audio File
```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -F "file=@/path/to/audio.mp3" \
  -F "metadata={\"title\":\"Test Audio\",\"category\":\"podcast\"}"
```

#### Upload Text File
```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -F "file=@/path/to/document.pdf" \
  -F "metadata={\"title\":\"Test Document\",\"category\":\"article\"}"
```

#### Upload Image File
```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -F "file=@/path/to/image.png" \
  -F "metadata={\"title\":\"Test Image\",\"category\":\"photo\"}"
```

**Response:**
```json
{
  "asset_uuid": "uuid-here",
  "job_id": "job-uuid",
  "status": "QUEUED",
  "workflow": "STANDARD_WORKFLOW",
  "estimated_time_minutes": 30,
  "status_url": "/api/jobs/job-uuid",
  "metadata_url": "/api/metadata/asset-uuid"
}
```

### Step 2: Check Processing Status

```bash
curl http://localhost:3000/api/workflow/status/{asset_uuid} \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

**Response:**
```json
{
  "asset_uuid": "uuid-here",
  "job_id": "job-uuid",
  "workflow_name": "STANDARD_WORKFLOW",
  "status": "PROCESSING",
  "progress_percentage": 50,
  "capabilities_completed": ["OCR", "Transcript"],
  "capabilities_failed": []
}
```

### Step 3: Get Enriched Metadata

```bash
curl http://localhost:3000/api/metadata/{asset_uuid} \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

**Response (Video/Audio):**
```json
{
  "asset_uuid": "uuid-here",
  "enriched_metadata": {
    "title": "Test Video",
    "category": "news",
    "transcript": "Welcome to today's interview...",
    "ocr_results": ["MediaCorp", "Breaking News", "2025"],
    "sentiment": {
      "overall": "POSITIVE",
      "score": 0.75,
      "segments": [...]
    },
    "speakers": {
      "count": 2,
      "segments": [...],
      "confidence": 0.92
    },
    "keywords": ["growth", "strategy", "2025"]
  },
  "version": 1,
  "version_id": "version-uuid"
}
```

### Step 4: Check Database Tables

#### Assets Table
```sql
SELECT 
    uuid,
    asset_type,
    asset_name,
    source_system,
    file_path,
    file_hash,
    status,
    enriched_metadata,
    created_at
FROM assets
ORDER BY created_at DESC;
```

#### Processing Jobs Table
```sql
SELECT 
    job_id,
    asset_uuid,
    workflow_name,
    status,
    progress_percentage,
    capabilities_completed,
    capabilities_failed,
    created_at,
    completed_at
FROM processing_jobs
ORDER BY created_at DESC;
```

#### Action Records Table
```sql
SELECT 
    record_id,
    asset_uuid,
    action_type,
    direction,
    controller_name,
    status,
    timestamp
FROM action_records
ORDER BY timestamp DESC;
```

## Local File Ingress Controller

### Setup Watch Directory
```bash
# Create directory to watch
mkdir -p ./watch_directory

# Set environment variable
export INGRESS_WATCH_DIR=./watch_directory
```

### Run Ingress Controller
```rust
// In your code or test script
use crate::controllers::local_ingress::LocalFileIngressController;

let controller = LocalFileIngressController::new(
    "./watch_directory".to_string(),
    db_pool.clone()
);

// Scan and ingest files
let result = controller.scan_and_ingest().await?;
println!("Ingested: {}, Skipped: {}, Errors: {}", 
    result.assets_processed, 
    result.assets_skipped, 
    result.errors
);
```

### Test Ingress
1. Copy files to watch directory:
```bash
cp /path/to/video.mp4 ./watch_directory/
cp /path/to/audio.mp3 ./watch_directory/
cp /path/to/document.pdf ./watch_directory/
```

2. Run ingress controller (it will automatically detect and process files)

## AI Processing Details

### OCR Extraction (Video/Image)
- Extracts text from video frames or images
- Results stored in `enriched_metadata.ocr_results`
- Simulated for local testing (production uses AWS Textract)

### Transcript Extraction (Audio/Video)
- Extracts speech-to-text from audio/video
- Results stored in `enriched_metadata.transcript`
- Simulated for local testing (production uses AWS Transcribe)

### Sentiment Analysis (Audio/Video)
- Analyzes sentiment from transcript
- Results stored in `enriched_metadata.sentiment`
- Includes:
  - Overall sentiment (POSITIVE/NEGATIVE/NEUTRAL)
  - Sentiment score (-1.0 to 1.0)
  - Segment-level sentiment

### Speaker Detection (Audio/Video)
- Detects multiple speakers
- Results stored in `enriched_metadata.speakers`
- Includes speaker segments with timestamps

## Complete Flow Example

### 1. Upload Video
```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer TOKEN" \
  -F "file=@interview.mp4" \
  -F "metadata={\"title\":\"CEO Interview\",\"category\":\"interview\"}"
```

### 2. Processing Happens Automatically
- File saved to `./local_storage/uploads/2025/11/24/interview.mp4`
- Asset created in `assets` table
- Job created in `processing_jobs` table
- AI processing runs in background:
  - OCR extraction
  - Transcript extraction
  - Sentiment analysis
  - Speaker detection

### 3. Check Status
```bash
curl http://localhost:3000/api/workflow/status/{asset_uuid} \
  -H "Authorization: Bearer TOKEN"
```

### 4. View Enriched Metadata
```bash
curl http://localhost:3000/api/metadata/{asset_uuid} \
  -H "Authorization: Bearer TOKEN"
```

### 5. All Data Stored in Database
- `assets` table: File info, metadata, enriched data
- `processing_jobs` table: Job status, progress
- `action_records` table: Audit trail
- `graph_nodes` table: Keywords, topics
- `asset_graph_nodes` table: Asset-keyword relationships

## File Storage Structure

```
local_storage/
├── uploads/
│   └── 2025/
│       └── 11/
│           └── 24/
│               ├── video.mp4
│               ├── audio.mp3
│               └── document.pdf
└── ingress/
    └── 2025/
        └── 11/
            └── 24/
                └── file_from_watch_dir.mp4
```

## Troubleshooting

### Files Not Processing
1. Check database connection
2. Check file permissions
3. Check logs for errors
4. Verify environment variables

### AI Processing Not Working
1. Check that `USE_LOCAL_STORAGE=true`
2. Verify file path is correct
3. Check database for processing_jobs status

### Database Issues
1. Run migrations: `sqlx migrate run`
2. Check PostgreSQL is running
3. Verify DATABASE_URL is correct

## Next Steps

1. **Production Setup**: Remove `USE_LOCAL_STORAGE` to use S3
2. **Real AI Services**: Replace simulated AI with AWS services
3. **Scheduled Ingress**: Set up cron job for local file ingress
4. **Monitoring**: Add logging and metrics

