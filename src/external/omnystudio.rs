// Omnystudio API client
// Audio ingress/egress integration

use reqwest::Client;
use anyhow::Result;
use serde_json::Value;

pub struct OmnystudioClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OmnystudioClient {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.omnystudio.com".to_string()),
        }
    }

    // Get audio content since timestamp (I-FR-01: Sync)
    pub async fn get_content_since(
        &self,
        since: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Value>> {
        let url = format!("{}/v1/content", self.base_url);
        let since_str = since.to_rfc3339();
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .query(&[("since", since_str)])
            .send()
            .await?;

        let content: Vec<Value> = response.json().await?;
        Ok(content)
    }

    // Update audio metadata (Egress)
    pub async fn update_content_metadata(
        &self,
        content_id: &str,
        metadata: Value,
    ) -> Result<()> {
        let url = format!("{}/v1/content/{}", self.base_url, content_id);
        
        self.client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&metadata)
            .send()
            .await?;

        Ok(())
    }
}
