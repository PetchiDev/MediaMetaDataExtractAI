// Preprocessing service
// I-FR-33: Preprocessing AI workflow
// Routes assets to appropriate workflows based on business logic

use crate::models::asset::Asset;
use anyhow::Result;

pub fn determine_workflow(asset: &Asset) -> Result<String> {
    // I-FR-33: Business logic-based preprocessing
    // Analyze pre-existing metadata to route to correct workflow
    
    let metadata = &asset.enriched_metadata;
    
    // Example routing logic:
    // - If title contains "interview" -> INTERVIEW_WORKFLOW
    // - If category is "news" -> NEWS_WORKFLOW
    // - If duration > 1 hour -> LONGFORM_WORKFLOW
    // - Default -> STANDARD_WORKFLOW
    
    if let Some(title) = metadata.get("title").and_then(|t| t.as_str()) {
        if title.to_lowercase().contains("interview") {
            return Ok("INTERVIEW_WORKFLOW".to_string());
        }
    }
    
    if let Some(category) = metadata.get("category").and_then(|c| c.as_str()) {
        if category.to_lowercase() == "news" {
            return Ok("NEWS_WORKFLOW".to_string());
        }
    }
    
    if let Some(duration) = asset.duration {
        if duration > 3600 {
            return Ok("LONGFORM_WORKFLOW".to_string());
        }
    }
    
    Ok("STANDARD_WORKFLOW".to_string())
}
