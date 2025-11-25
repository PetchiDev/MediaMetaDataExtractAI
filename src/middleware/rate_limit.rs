// Rate limiting middleware
// I-FR-25: Rate limiting and access throttling

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
struct RateLimitState {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
}

impl RateLimitState {
    fn new() -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn check_rate_limit(&self, key: &str, limit: u32, window: Duration) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();
        
        // Clean old requests outside the window
        if let Some(timestamps) = requests.get_mut(key) {
            timestamps.retain(|&time| now.duration_since(time) < window);
            
            if timestamps.len() >= limit as usize {
                return false; // Rate limit exceeded
            }
            
            timestamps.push(now);
        } else {
            requests.insert(key.to_string(), vec![now]);
        }
        
        true
    }
}

pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get rate limit config (default: 100 requests per minute)
    let limit = 100u32;
    let window = Duration::from_secs(60);
    
    // Get identifier (user ID from auth, or IP address)
    let identifier = request.headers()
        .get("x-user-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_else(|| {
            request.headers()
                .get("x-forwarded-for")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("unknown")
        });
    
    // TODO: Use shared state or Redis for distributed rate limiting
    // For now, using in-memory (not suitable for production with multiple instances)
    static RATE_LIMIT_STATE: std::sync::OnceLock<RateLimitState> = std::sync::OnceLock::new();
    let state = RATE_LIMIT_STATE.get_or_init(|| RateLimitState::new());
    
    if !state.check_rate_limit(identifier, limit, window) {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    Ok(next.run(request).await)
}
