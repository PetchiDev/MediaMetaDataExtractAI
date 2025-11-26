# 🚀 Google Sign-In Quick Start

## Setup (5 minutes)

### 1. Get Google OAuth Credentials

1. Go to: https://console.cloud.google.com/apis/credentials
2. Create OAuth 2.0 Client ID
3. Add redirect URI: `http://localhost:3000/api/auth/google/callback`
4. Copy Client ID and Client Secret

### 2. Add to .env

```bash
GOOGLE_CLIENT_ID=your-client-id.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=your-client-secret
GOOGLE_REDIRECT_URI=http://localhost:3000/api/auth/google/callback
JWT_SECRET=your-secret-key
```

### 3. Build and Run

```bash
cargo build
cargo run
```

## Usage

### Step 1: Get Google Login URL

```bash
curl http://localhost:3000/api/auth/google/login
```

**Response:**
```json
{
  "redirect_url": "https://accounts.google.com/o/oauth2/v2/auth?client_id=...&redirect_uri=...&response_type=code&scope=openid+email+profile&state=...",
  "status": "redirect_required",
  "message": "Redirect user to Google Sign-In"
}
```

### Step 2: Open Redirect URL in Browser

Copy `redirect_url` and open in browser. Google will ask for permission.

### Step 3: Google Redirects Back

After authentication, Google redirects to:
```
http://localhost:3000/api/auth/google/callback?code=4/0A...&state=...
```

Backend automatically:
- ✅ Exchanges code for access token
- ✅ Gets user info from Google
- ✅ Creates/updates user in database
- ✅ Generates JWT tokens

**Response:**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "user": {
    "id": "user-uuid",
    "email": "user@gmail.com",
    "name": "John Doe",
    "role": "VIEWER",
    "picture": "https://lh3.googleusercontent.com/..."
  },
  "status": "success"
}
```

### Step 4: Use JWT Token

```bash
curl http://localhost:3000/api/media/upload \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIs..." \
  -F "file=@video.mp4"
```

## Frontend Example

### HTML Button

```html
<button onclick="signInWithGoogle()">Sign in with Google</button>

<script>
async function signInWithGoogle() {
    // Get redirect URL
    const response = await fetch('http://localhost:3000/api/auth/google/login');
    const data = await response.json();
    
    // Redirect to Google
    window.location.href = data.redirect_url;
}
</script>
```

### React Component

```jsx
function GoogleSignIn() {
    const handleSignIn = async () => {
        const response = await fetch('http://localhost:3000/api/auth/google/login');
        const data = await response.json();
        window.location.href = data.redirect_url;
    };
    
    return (
        <button onClick={handleSignIn}>
            Sign in with Google
        </button>
    );
}
```

## What Gets Stored

### Database - users Table
```sql
SELECT * FROM users WHERE email = 'user@gmail.com';
```

**Result:**
- `id`: UUID
- `email`: user@gmail.com
- `name`: John Doe
- `role`: VIEWER (default)
- `sso_provider_id`: google_123456789
- `created_at`: 2025-11-24 10:00:00
- `last_login`: 2025-11-24 10:00:00

## Complete Flow Diagram

```
1. User clicks "Sign in with Google"
   ↓
2. Frontend calls GET /api/auth/google/login
   ↓
3. Backend returns Google OAuth URL
   ↓
4. Frontend redirects to Google
   ↓
5. User authenticates with Google
   ↓
6. Google redirects to /api/auth/google/callback?code=...
   ↓
7. Backend exchanges code for access token
   ↓
8. Backend gets user info from Google
   ↓
9. Backend creates/updates user in database
   ↓
10. Backend generates JWT tokens
   ↓
11. Backend returns tokens to frontend
   ↓
12. Frontend stores token and uses for API calls
```

## Testing

### Test Complete Flow

```bash
# 1. Get login URL
LOGIN_URL=$(curl -s http://localhost:3000/api/auth/google/login | jq -r '.redirect_url')

# 2. Open in browser (or use curl with browser)
echo "Open this URL: $LOGIN_URL"

# 3. After Google auth, callback will return JWT tokens
# Check browser network tab or server logs
```

## Environment Variables

```bash
# Required
GOOGLE_CLIENT_ID=your-client-id
GOOGLE_CLIENT_SECRET=your-client-secret
GOOGLE_REDIRECT_URI=http://localhost:3000/api/auth/google/callback

# Optional (defaults shown)
JWT_SECRET=change-me-in-production
DATABASE_URL=postgresql://user:pass@localhost/mediacorp
```

## Summary

✅ **Google OAuth 2.0** implemented  
✅ **Sign in with Google** ready  
✅ **User auto-creation** from Google profile  
✅ **JWT tokens** generated  
✅ **Database storage** of user info  

Just add Google credentials and you're ready! 🎉

