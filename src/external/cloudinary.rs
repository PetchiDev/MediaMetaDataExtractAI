// Cloudinary API client
// Image ingress/egress integration

use reqwest::Client;
use anyhow::Result;
use serde_json::Value;

pub struct CloudinaryClient {
    client: Client,
    cloud_name: String,
    api_key: String,
    api_secret: String,
}

impl CloudinaryClient {
    pub fn new(cloud_name: String, api_key: String, api_secret: String) -> Self {
        Self {
            client: Client::new(),
            cloud_name,
            api_key,
            api_secret,
        }
    }

    // Get resources since timestamp (I-FR-01: Sync)
    pub async fn get_resources_since(
        &self,
        since: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Value>> {
        let url = format!("https://api.cloudinary.com/v1_1/{}/resources/image", self.cloud_name);
        let since_timestamp = since.timestamp();
        
        let response = self.client
            .get(&url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .query(&[("next_cursor", since_timestamp.to_string())])
            .send()
            .await?;

        let result: Value = response.json().await?;
        let resources = result.get("resources")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();

        Ok(resources)
    }

    // Update image metadata (Egress)
    pub async fn update_image_metadata(
        &self,
        public_id: &str,
        metadata: Value,
    ) -> Result<()> {
        let url = format!("https://api.cloudinary.com/v1_1/{}/resources/image/upload", self.cloud_name);
        
        let mut params = vec![
            ("public_id", public_id.to_string()),
        ];

        // Add metadata fields
        if let Some(tags) = metadata.get("tags") {
            if let Some(tags_str) = tags.as_array().map(|t| {
                t.iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            }) {
                params.push(("tags", tags_str));
            }
        }

        self.client
            .post(&url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .form(&params)
            .send()
            .await?;

        Ok(())
    }
}
