// Brightcove API client
// Video ingress/egress integration

use reqwest::Client;
use anyhow::Result;
use serde_json::Value;

pub struct BrightcoveClient {
    client: Client,
    api_key: String,
    account_id: String,
    base_url: String,
}

impl BrightcoveClient {
    pub fn new(api_key: String, account_id: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            account_id: account_id.clone(),
            base_url: format!("https://cms.api.brightcove.com/v1/accounts/{}", account_id),
        }
    }

    // Get videos since timestamp (I-FR-01: Sync with interval)
    pub async fn get_videos_since(&self, since: chrono::DateTime<chrono::Utc>) -> Result<Vec<Value>> {
        let url = format!("{}/videos", self.base_url);
        let since_str = since.to_rfc3339();
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .query(&[("since", since_str)])
            .send()
            .await?;

        let videos: Vec<Value> = response.json().await?;
        Ok(videos)
    }

    // Update video metadata (Egress)
    pub async fn update_video_metadata(
        &self,
        video_id: &str,
        metadata: Value,
    ) -> Result<()> {
        let url = format!("{}/videos/{}", self.base_url, video_id);
        
        self.client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&metadata)
            .send()
            .await?;

        Ok(())
    }

    // Get video download URL
    pub async fn get_video_download_url(&self, video_id: &str) -> Result<String> {
        let url = format!("{}/videos/{}/sources", self.base_url, video_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        let sources: Vec<Value> = response.json().await?;
        if let Some(source) = sources.first() {
            if let Some(src) = source.get("src").and_then(|s| s.as_str()) {
                return Ok(src.to_string());
            }
        }

        anyhow::bail!("No download URL found for video {}", video_id)
    }
}
