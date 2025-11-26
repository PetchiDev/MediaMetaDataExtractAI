// Local file storage service
// For local testing without S3 dependency

use anyhow::Result;
use chrono::Datelike;
use std::fs;
use std::path::{Path, PathBuf};

pub struct LocalStorageService {
    base_path: PathBuf,
}

impl LocalStorageService {
    pub fn new(base_path: Option<String>) -> Self {
        let path = base_path
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("./local_storage"));
        
        // Create directory if it doesn't exist
        fs::create_dir_all(&path).ok();
        
        Self { base_path: path }
    }
    
    /// Generate storage path for file
    pub fn generate_path(&self, category: &str, filename: &str) -> PathBuf {
        let date = chrono::Utc::now();
        let date_path = format!("{}/{:02}/{:02}", date.year(), date.month(), date.day());
        
        self.base_path
            .join(category)
            .join(date_path)
            .join(filename)
    }
    
    /// Save file to local storage
    pub async fn save_file(&self, category: &str, filename: &str, data: Vec<u8>) -> Result<String> {
        let file_path = self.generate_path(category, filename);
        
        // Create parent directories
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Write file
        fs::write(&file_path, data)?;
        
        // Return path as string
        Ok(file_path.to_string_lossy().to_string())
    }
    
    /// Read file from local storage
    pub async fn read_file(&self, file_path: &str) -> Result<Vec<u8>> {
        let data = fs::read(file_path)?;
        Ok(data)
    }
    
    /// Delete file from local storage
    pub async fn delete_file(&self, file_path: &str) -> Result<()> {
        fs::remove_file(file_path)?;
        Ok(())
    }
    
    /// Get file size
    pub async fn get_file_size(&self, file_path: &str) -> Result<u64> {
        let metadata = fs::metadata(file_path)?;
        Ok(metadata.len())
    }
}

