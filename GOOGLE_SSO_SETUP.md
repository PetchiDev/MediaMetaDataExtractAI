# 🔐 Google Sign-In Setup Guide

## Overview
The platform now supports **Sign in with Google** using OAuth 2.0.

## Setup Steps

### 1. Create Google OAuth Credentials

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select existing one
3. Enable **Google+ API** or **Google Identity API**
4. Go to **Credentials** → **Create Credentials** → **OAuth 2.0 Client ID**
5. Configure OAuth consent screen:
   - Application name: MediaCorp AI Platform
   - User support email: your-email@mediacorp.com
   - Authorized domains: mediacorp.com
6. Create OAuth 2.0 Client ID:
   - Application type: **Web application**
   - Name: MediaCorp AI Platform
   - Authorized redirect URIs:
     - `http://localhost:3000/api/auth/google/callback` (for local testing)
     - `https://your-domain.com/api/auth/google/callback` (for production)
7. Copy **Client ID** and **Client Secret**

### 2. Set Environment Variables

Add to your `.env` file:

```bash
# Google OAuth
GOOGLE_CLIENT_ID=your-google-client-id.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=your-google-client-secret
GOOGLE_REDIRECT_URI=http://localhost:3000/api/auth/google/callback

# JWT Secret
JWT_SECRET=your-jwt-secret-key

# Database
DATABASE_URL=postgresql://user:password@localhost/mediacorp
```

### 3. Install Dependencies

The following dependencies are already added to `Cargo.toml`:
- `oauth2 = "4.4"`
- `url = "2.5"`

Run:
```bash
cargo build
```

## API Endpoints

### 1. Initiate Google Sign-In
**Endpoint**: `GET /api/auth/google/login`

**Description**: Returns Google OAuth authorization URL

**Response:**
```json
{
  "redirect_url": "https://accounts.google.com/o/oauth2/v2/auth?client_id=...&redirect_uri=...&response_type=code&scope=openid+email+profile&state=...",
  "status": "redirect_required",
  "message": "Redirect user to Google Sign-In"
}
```

### 2. Google OAuth Callback
**Endpoint**: `GET /api/auth/google/callback?code=...&state=...`

**Description**: Handles Google OAuth callback and returns JWT tokens

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

## Frontend Integration

### React Example

```jsx
function GoogleSignInButton() {
  const handleGoogleLogin = async () => {
    // Step 1: Get redirect URL
    const response = await fetch('http://localhost:3000/api/auth/google/login');
    const data = await response.json();
    
    // Step 2: Redirect to Google
    window.location.href = data.redirect_url;
  };
  
  return (
    <button onClick={handleGoogleLogin}>
      Sign in with Google
    </button>
  );
}

// Callback handler (in your callback route)
function GoogleCallback() {
  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    const code = params.get('code');
    
    if (code) {
      // The backend handles the callback automatically
      // Just redirect to your app
      window.location.href = '/dashboard';
    }
  }, []);
  
  return <div>Signing in...</div>;
}
```

### HTML Example

```html
<!DOCTYPE html>
<html>
<head>
    <title>Sign in with Google</title>
</head>
<body>
    <button id="google-signin">Sign in with Google</button>
    
    <script>
        document.getElementById('google-signin').addEventListener('click', async () => {
            // Get redirect URL
            const response = await fetch('http://localhost:3000/api/auth/google/login');
            const data = await response.json();
            
            // Redirect to Google
            window.location.href = data.redirect_url;
        });
    </script>
</body>
</html>
```

## Complete Flow

### Step 1: User Clicks "Sign in with Google"
```
Frontend → GET /api/auth/google/login
Backend → Returns Google OAuth URL
Frontend → Redirects to Google
```

### Step 2: User Authenticates with Google
```
User → Google OAuth page
User → Enters credentials
Google → Validates and redirects back
```

### Step 3: Google Redirects to Callback
```
Google → GET /api/auth/google/callback?code=...&state=...
Backend → Exchanges code for access token
Backend → Gets user info from Google
Backend → Creates/updates user in database
Backend → Generates JWT tokens
Backend → Returns tokens to frontend
```

### Step 4: Frontend Uses JWT Token
```
Frontend → Stores JWT token
Frontend → Uses token in Authorization header:
  Authorization: Bearer eyJhbGciOiJIUzI1NiIs...
```

## User Creation

### First Time Login
- User info fetched from Google:
  - Email
  - Name
  - Profile picture
  - Google ID
- New user created in `users` table:
  - Default role: `VIEWER`
  - `sso_provider_id`: `google_{google_id}`
  - Email and name from Google

### Subsequent Logins
- User found by `sso_provider_id`
- User info updated if changed
- Last login timestamp updated
- New JWT tokens generated

## Database Schema

### users Table
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'VIEWER',
    sso_provider_id VARCHAR(255),  -- Format: "google_123456789"
    created_at TIMESTAMPTZ NOT NULL,
    last_login TIMESTAMPTZ
);
```

## Security Features

✅ **OAuth 2.0 Flow** - Standard Google OAuth  
✅ **CSRF Protection** - State token validation  
✅ **JWT Tokens** - Secure token-based auth  
✅ **User Verification** - Google verified email  
✅ **Token Expiry** - 24 hour access tokens  
✅ **Refresh Tokens** - 30 day refresh tokens  

## Testing

### Local Testing

1. Set up Google OAuth credentials
2. Add to `.env`:
   ```bash
   GOOGLE_CLIENT_ID=your-client-id
   GOOGLE_CLIENT_SECRET=your-client-secret
   GOOGLE_REDIRECT_URI=http://localhost:3000/api/auth/google/callback
   ```

3. Start server:
   ```bash
   cargo run
   ```

4. Test login:
   ```bash
   # Get redirect URL
   curl http://localhost:3000/api/auth/google/login
   
   # Open redirect_url in browser
   # After Google auth, you'll be redirected to callback
   # Backend will return JWT tokens
   ```

### Production Setup

1. Update redirect URI in Google Console:
   - `https://your-domain.com/api/auth/google/callback`

2. Update environment variables:
   ```bash
   GOOGLE_REDIRECT_URI=https://your-domain.com/api/auth/google/callback
   ```

3. Deploy and test

## Troubleshooting

### Error: "GOOGLE_CLIENT_ID not set"
- Make sure `.env` file has `GOOGLE_CLIENT_ID`
- Restart server after adding env vars

### Error: "Invalid redirect_uri"
- Check redirect URI matches exactly in Google Console
- Must include protocol (http:// or https://)
- No trailing slash

### Error: "Failed to exchange code for token"
- Check `GOOGLE_CLIENT_SECRET` is correct
- Verify redirect URI matches
- Check code hasn't expired (codes expire quickly)

### User Not Created
- Check database connection
- Verify migrations ran
- Check logs for errors

## API Usage Examples

### Get Google Login URL
```bash
curl http://localhost:3000/api/auth/google/login
```

### Handle Callback (Automatic)
When Google redirects to:
```
http://localhost:3000/api/auth/google/callback?code=4/0A...&state=...
```

Backend automatically:
1. Exchanges code for token
2. Gets user info
3. Creates/updates user
4. Returns JWT tokens

## Summary

✅ **Google OAuth 2.0** implemented  
✅ **Sign in with Google** button ready  
✅ **User creation** automatic  
✅ **JWT tokens** generated  
✅ **Database storage** of user info  
✅ **Profile picture** included  

Ready to use! 🎉

