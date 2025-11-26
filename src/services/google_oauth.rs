// Google OAuth Service
// Sign in with Google implementation

use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl,
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use anyhow::Result;
use serde_json::Value;
use reqwest::Client;

pub struct GoogleOAuthService {
    client: BasicClient,
    http_client: Client,
}

impl GoogleOAuthService {
    pub fn new() -> Result<Self> {
        let client_id = ClientId::new(
            std::env::var("GOOGLE_CLIENT_ID")
                .map_err(|_| anyhow::anyhow!("GOOGLE_CLIENT_ID not set"))?
        );
        
        let client_secret = ClientSecret::new(
            std::env::var("GOOGLE_CLIENT_SECRET")
                .map_err(|_| anyhow::anyhow!("GOOGLE_CLIENT_SECRET not set"))?
        );
        
        let redirect_url = RedirectUrl::new(
            std::env::var("GOOGLE_REDIRECT_URI")
                .unwrap_or_else(|_| "http://localhost:3000/api/auth/google/callback".to_string())
        )?;
        
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())?;
        
        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url);
        
        Ok(Self {
            client,
            http_client: Client::new(),
        })
    }
    
    /// Generate Google OAuth authorization URL
    pub fn get_authorization_url(&self) -> (url::Url, oauth2::CsrfToken) {
        let mut request = self.client
            .authorize_url(oauth2::CsrfToken::new_random);
        
        // Request scopes
        request = request.add_scope(Scope::new("openid".to_string()));
        request = request.add_scope(Scope::new("email".to_string()));
        request = request.add_scope(Scope::new("profile".to_string()));
        
        request.url()
    }
    
    /// Exchange authorization code for access token
    pub async fn exchange_code(&self, code: String) -> Result<String> {
        let code = AuthorizationCode::new(code);
        
        let token_result = self.client
            .exchange_code(code)
            .request_async(async_http_client)
            .await?;
        
        Ok(token_result.access_token().secret().clone())
    }
    
    /// Get user info from Google using access token
    pub async fn get_user_info(&self, access_token: &str) -> Result<GoogleUserInfo> {
        let response = self.http_client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await?;
        
        let user_data: Value = response.json().await?;
        
        Ok(GoogleUserInfo {
            id: user_data["id"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Missing id"))?
                .to_string(),
            email: user_data["email"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Missing email"))?
                .to_string(),
            name: user_data["name"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            picture: user_data["picture"]
                .as_str()
                .map(|s| s.to_string()),
            verified_email: user_data["verified_email"]
                .as_bool()
                .unwrap_or(false),
        })
    }
}

#[derive(Debug, Clone)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
    pub verified_email: bool,
}

