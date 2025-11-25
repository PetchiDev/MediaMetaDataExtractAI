// S3 service for file storage
// I-FR-02: File storage with hash-based deduplication

use aws_sdk_s3::{Client as S3Client, primitives::ByteStream};
use aws_config::SdkConfig;
use anyhow::Result;
use std::path::Path;

pub struct S3Service {
    client: S3Client,
    bucket: String,
}

impl S3Service {
    pub fn new(config: &SdkConfig, bucket: String) -> Self {
        let client = S3Client::new(config);
        Self { client, bucket }
    }

    pub async fn upload_file(
        &self,
        key: &str,
        data: Vec<u8>,
    ) -> Result<String> {
        let body = ByteStream::from(data);
        
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .send()
            .await?;

        Ok(format!("s3://{}/{}", self.bucket, key))
    }

    pub async fn download_file(&self, key: &str) -> Result<Vec<u8>> {
        let response = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        let data = response.body.collect().await?.into_bytes().to_vec();
        Ok(data)
    }

    pub async fn delete_file(&self, key: &str) -> Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        Ok(())
    }

    pub fn generate_key(&self, prefix: &str, filename: &str) -> String {
        let timestamp = chrono::Utc::now().format("%Y/%m/%d");
        format!("{}/{}/{}", prefix, timestamp, filename)
    }
}
