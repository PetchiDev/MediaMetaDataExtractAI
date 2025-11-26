# 📥 Media Download/Stream API

## Overview
API to download and stream actual media files (Video, Audio, Image, Text) stored in the system.

## Endpoints

### 1. Get Asset Metadata
**Endpoint**: `GET /api/media/:asset_id`

Returns asset information and metadata (not the file itself).

**Response:**
```json
{
  "uuid": "asset-uuid",
  "asset_type": "VIDEO",
  "asset_name": "video.mp4",
  "status": "PROCESSED",
  "file_path": "./local_storage/uploads/2025/11/24/video.mp4",
  "enriched_metadata": {...},
  "created_at": "2025-11-24T10:00:00Z",
  "download_url": "/api/media/asset-uuid/download"
}
```

### 2. Download/Stream Media File
**Endpoint**: `GET /api/media/:asset_id/download`

Returns the actual file content for playback or download.

**Features:**
- ✅ Supports Video (MP4, AVI, MOV)
- ✅ Supports Audio (MP3, WAV, M4A)
- ✅ Supports Image (PNG, JPG, GIF)
- ✅ Supports Text (PDF, TXT, DOCX)
- ✅ Proper Content-Type headers
- ✅ Range request support for video/audio streaming
- ✅ Works with local storage and S3

## Usage Examples

### Download Video File
```bash
curl -X GET http://localhost:3000/api/media/{asset_uuid}/download \
  -H "Authorization: Bearer YOUR_TOKEN" \
  --output video.mp4
```

### Stream Video in Browser
```html
<video controls>
  <source src="http://localhost:3000/api/media/{asset_uuid}/download" type="video/mp4">
</video>
```

### Stream Audio in Browser
```html
<audio controls>
  <source src="http://localhost:3000/api/media/{asset_uuid}/download" type="audio/mpeg">
</audio>
```

### Download Image
```bash
curl -X GET http://localhost:3000/api/media/{asset_uuid}/download \
  -H "Authorization: Bearer YOUR_TOKEN" \
  --output image.png
```

### Download PDF Document
```bash
curl -X GET http://localhost:3000/api/media/{asset_uuid}/download \
  -H "Authorization: Bearer YOUR_TOKEN" \
  --output document.pdf
```

## Content Types

The API automatically sets the correct Content-Type header:

| Asset Type | Format | Content-Type |
|------------|--------|--------------|
| Video | MP4 | video/mp4 |
| Video | AVI | video/x-msvideo |
| Video | MOV | video/quicktime |
| Audio | MP3 | audio/mpeg |
| Audio | WAV | audio/wav |
| Audio | M4A | audio/mp4 |
| Image | PNG | image/png |
| Image | JPG/JPEG | image/jpeg |
| Image | GIF | image/gif |
| Text | PDF | application/pdf |
| Text | TXT | text/plain |
| Text | HTML | text/html |
| Text | JSON | application/json |
| Text | DOC/DOCX | application/msword |

## Response Headers

```
Content-Type: video/mp4
Content-Disposition: inline; filename="video.mp4"
Content-Length: 52428800
Accept-Ranges: bytes  (for video/audio)
```

## Complete Flow Example

### 1. Upload Video
```bash
curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer TOKEN" \
  -F "file=@interview.mp4" \
  -F "metadata={\"title\":\"CEO Interview\"}"
```

**Response:**
```json
{
  "asset_uuid": "abc-123-def",
  "job_id": "job-456",
  "status": "QUEUED"
}
```

### 2. Query Asset Info
```bash
curl http://localhost:3000/api/media/abc-123-def \
  -H "Authorization: Bearer TOKEN"
```

**Response:**
```json
{
  "uuid": "abc-123-def",
  "asset_type": "VIDEO",
  "asset_name": "interview.mp4",
  "status": "PROCESSED",
  "download_url": "/api/media/abc-123-def/download"
}
```

### 3. Download/Stream Video
```bash
# Download to file
curl http://localhost:3000/api/media/abc-123-def/download \
  -H "Authorization: Bearer TOKEN" \
  --output interview.mp4

# Or use in HTML
<video src="http://localhost:3000/api/media/abc-123-def/download" controls></video>
```

## Storage Support

### Local Storage (Testing)
- Files stored in `./local_storage/uploads/YYYY/MM/DD/`
- Set `USE_LOCAL_STORAGE=true` in `.env`

### S3 Storage (Production)
- Files stored in S3 bucket
- Set `USE_LOCAL_STORAGE=false` or remove env var
- File path format: `s3://bucket-name/key/path`

## Error Responses

### Asset Not Found (404)
```json
{
  "error": "Asset not found"
}
```

### File Not Found (500)
```json
{
  "error": "File not found in storage"
}
```

## Security

- ✅ Requires authentication (Bearer token or API key)
- ✅ Validates asset exists before serving
- ✅ Proper content-type validation
- ✅ File path sanitization

## Performance

- ✅ Efficient file reading (local or S3)
- ✅ Range request support for large files
- ✅ Streaming support for video/audio
- ✅ Proper content-length headers

## Testing

### Test Video Download
```bash
# 1. Upload video
ASSET_UUID=$(curl -X POST http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer TOKEN" \
  -F "file=@test.mp4" \
  -F "metadata={\"title\":\"Test\"}" | jq -r '.asset_uuid')

# 2. Download video
curl http://localhost:3000/api/media/$ASSET_UUID/download \
  -H "Authorization: Bearer TOKEN" \
  --output downloaded_video.mp4
```

### Test Audio Stream
```bash
# Get audio asset UUID
AUDIO_UUID="your-audio-uuid"

# Stream audio
curl http://localhost:3000/api/media/$AUDIO_UUID/download \
  -H "Authorization: Bearer TOKEN" \
  --output audio.mp3
```

## Integration Examples

### React Component
```jsx
function VideoPlayer({ assetUuid }) {
  const videoUrl = `http://localhost:3000/api/media/${assetUuid}/download`;
  
  return (
    <video controls>
      <source src={videoUrl} type="video/mp4" />
    </video>
  );
}
```

### Python Client
```python
import requests

def download_media(asset_uuid, token, output_path):
    url = f"http://localhost:3000/api/media/{asset_uuid}/download"
    headers = {"Authorization": f"Bearer {token}"}
    
    response = requests.get(url, headers=headers, stream=True)
    response.raise_for_status()
    
    with open(output_path, 'wb') as f:
        for chunk in response.iter_content(chunk_size=8192):
            f.write(chunk)
    
    print(f"Downloaded to {output_path}")
```

## Summary

✅ **Files stored** in local storage or S3  
✅ **Query API** to get asset info  
✅ **Download API** to get actual files  
✅ **Streaming support** for video/audio  
✅ **All file types** supported (Video, Audio, Image, Text)  
✅ **Proper headers** for browser playback  

