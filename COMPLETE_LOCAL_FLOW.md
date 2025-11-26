# ✅ Complete Local File Upload & Download Flow

## 🎯 What's Implemented

### 1. File Upload API ✅
- **Endpoint**: `POST /api/media/upload`
- Supports: Video (MP4, AVI), Audio (MP3, WAV), Image (PNG, JPG), Text (PDF, TXT, DOCX)
- Files stored in `./local_storage/uploads/YYYY/MM/DD/`
- All metadata stored in database

### 2. File Download/Stream API ✅
- **Endpoint**: `GET /api/media/:asset_id/download`
- Returns actual file content (Video, Audio, Image, Text)
- Proper Content-Type headers
- Streaming support for video/audio
- Works with local storage

### 3. Query Asset Info API ✅
- **Endpoint**: `GET /api/media/:asset_id`
- Returns asset metadata and info
- Includes download URL

### 4. AI Processing ✅
- OCR extraction (Video/Image)
- Transcript extraction (Audio/Video)
- Sentiment analysis (Audio/Video)
- Speaker detection (Audio/Video)
- All results stored in database

### 5. Database Storage ✅
All data stored in tables:
- `assets` - File info, metadata, enriched data
- `processing_jobs` - Job status
- `action_records` - Audit trail
- `graph_nodes` - Keywords
- `asset_graph_nodes` - Relationships

## 🔄 Complete Flow

### Step 1: Upload File
```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer TOKEN" \
  -F "file=@video.mp4" \
  -F "metadata={\"title\":\"Test Video\",\"category\":\"news\"}"
```

**What Happens:**
1. File saved to `./local_storage/uploads/2025/11/24/video.mp4`
2. Asset created in `assets` table
3. Processing job created
4. AI processing runs automatically
5. Enriched metadata stored in database

### Step 2: Query Asset
```bash
curl http://localhost:3000/api/media/{asset_uuid} \
  -H "Authorization: Bearer TOKEN"
```

**Response:**
```json
{
  "uuid": "asset-uuid",
  "asset_type": "VIDEO",
  "asset_name": "video.mp4",
  "status": "PROCESSED",
  "enriched_metadata": {
    "transcript": "...",
    "sentiment": {...},
    "ocr_results": [...],
    "speakers": {...}
  },
  "download_url": "/api/media/asset-uuid/download"
}
```

### Step 3: Download/Stream File
```bash
# Download video
curl http://localhost:3000/api/media/{asset_uuid}/download \
  -H "Authorization: Bearer TOKEN" \
  --output video.mp4

# Or use in HTML
<video src="http://localhost:3000/api/media/{asset_uuid}/download" controls></video>
```

**What Happens:**
1. API reads file from local storage
2. Returns file with proper Content-Type
3. Browser can play/stream the file

## 📊 Database Tables

### assets Table
```sql
SELECT 
    uuid,
    asset_type,        -- VIDEO, AUDIO, IMAGE, TEXT
    asset_name,        -- filename
    file_path,         -- local storage path
    file_hash,         -- SHA-256 hash
    status,            -- QUEUED, PROCESSING, PROCESSED
    enriched_metadata, -- JSONB with AI results
    created_at
FROM assets;
```

### processing_jobs Table
```sql
SELECT 
    job_id,
    asset_uuid,
    workflow_name,
    status,
    progress_percentage,
    capabilities_completed,  -- ["OCR", "Transcript", "Sentiment"]
    created_at,
    completed_at
FROM processing_jobs;
```

### action_records Table
```sql
SELECT 
    record_id,
    asset_uuid,
    action_type,       -- INGRESS, EGRESS, USER_UPLOAD
    direction,         -- INBOUND, OUTBOUND
    controller_name,
    status,
    timestamp
FROM action_records;
```

## 🎬 Example: Complete Video Flow

### 1. Upload Video
```bash
ASSET_UUID=$(curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer TOKEN" \
  -F "file=@interview.mp4" \
  -F "metadata={\"title\":\"CEO Interview\",\"category\":\"interview\"}" \
  | jq -r '.asset_uuid')

echo "Asset UUID: $ASSET_UUID"
```

### 2. Wait for Processing (or check status)
```bash
# Check status
curl http://localhost:3000/api/workflow/status/$ASSET_UUID \
  -H "Authorization: Bearer TOKEN"
```

### 3. Get Enriched Metadata
```bash
curl http://localhost:3000/api/metadata/$ASSET_UUID \
  -H "Authorization: Bearer TOKEN"
```

**Response includes:**
- Transcript (speech-to-text)
- Sentiment analysis
- OCR results (text from video frames)
- Speaker detection
- Keywords

### 4. Download/Stream Video
```bash
# Download
curl http://localhost:3000/api/media/$ASSET_UUID/download \
  -H "Authorization: Bearer TOKEN" \
  --output interview.mp4

# Or stream in browser
# <video src="http://localhost:3000/api/media/$ASSET_UUID/download" controls></video>
```

## 🎵 Example: Complete Audio Flow

### 1. Upload Audio
```bash
AUDIO_UUID=$(curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer TOKEN" \
  -F "file=@podcast.mp3" \
  -F "metadata={\"title\":\"Tech Podcast\",\"category\":\"podcast\"}" \
  | jq -r '.asset_uuid')
```

### 2. Get Results
```bash
curl http://localhost:3000/api/metadata/$AUDIO_UUID \
  -H "Authorization: Bearer TOKEN"
```

**Response includes:**
- Transcript
- Sentiment analysis
- Speaker detection
- Keywords

### 3. Download/Stream Audio
```bash
# Download
curl http://localhost:3000/api/media/$AUDIO_UUID/download \
  -H "Authorization: Bearer TOKEN" \
  --output podcast.mp3

# Or stream
# <audio src="http://localhost:3000/api/media/$AUDIO_UUID/download" controls></audio>
```

## 📄 Example: Complete Text Flow

### 1. Upload Document
```bash
DOC_UUID=$(curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer TOKEN" \
  -F "file=@article.pdf" \
  -F "metadata={\"title\":\"News Article\",\"category\":\"news\"}" \
  | jq -r '.asset_uuid')
```

### 2. Download Document
```bash
curl http://localhost:3000/api/media/$DOC_UUID/download \
  -H "Authorization: Bearer TOKEN" \
  --output article.pdf
```

## 🖼️ Example: Complete Image Flow

### 1. Upload Image
```bash
IMG_UUID=$(curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer TOKEN" \
  -F "file=@photo.png" \
  -F "metadata={\"title\":\"Event Photo\",\"category\":\"photo\"}" \
  | jq -r '.asset_uuid')
```

### 2. Get OCR Results
```bash
curl http://localhost:3000/api/metadata/$IMG_UUID \
  -H "Authorization: Bearer TOKEN"
```

**Response includes:**
- OCR results (text extracted from image)

### 3. Download Image
```bash
curl http://localhost:3000/api/media/$IMG_UUID/download \
  -H "Authorization: Bearer TOKEN" \
  --output photo.png
```

## 🔍 Query All Assets

### Get All Videos
```sql
SELECT uuid, asset_name, status, enriched_metadata
FROM assets
WHERE asset_type = 'VIDEO'
ORDER BY created_at DESC;
```

### Get All Processed Assets
```sql
SELECT uuid, asset_type, asset_name, enriched_metadata->'sentiment' as sentiment
FROM assets
WHERE status = 'PROCESSED'
ORDER BY created_at DESC;
```

### Search by Keyword
```sql
SELECT a.uuid, a.asset_name, a.enriched_metadata
FROM assets a
WHERE a.enriched_metadata->'keywords' @> '["growth"]'::jsonb;
```

## ✅ Summary

**Upload API**: ✅ Files stored locally  
**Query API**: ✅ Get asset info and metadata  
**Download API**: ✅ Get actual video/audio/image/text files  
**AI Processing**: ✅ OCR, Sentiment, Transcript, Speakers  
**Database Storage**: ✅ All data in tables  
**Local Testing**: ✅ No S3 needed  

Everything works locally! 🎉

