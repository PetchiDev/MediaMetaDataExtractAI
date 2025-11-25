// Metadata models
// I-FR-27: Metadata editing and unified input
// I-FR-19: Conflict resolution

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub category: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedMetadata {
    pub speakers: Option<SpeakerMetadata>,
    pub text_recognition: Option<TextRecognitionMetadata>,
    pub sentiment: Option<SentimentMetadata>,
    pub object_detection: Option<ObjectDetectionMetadata>,
    pub brand_detection: Option<BrandDetectionMetadata>,
    pub keywords: Vec<String>,
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerMetadata {
    pub count: i32,
    pub segments: Vec<SpeakerSegment>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerSegment {
    pub speaker_id: String,
    pub start_time: f64,
    pub end_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRecognitionMetadata {
    pub transcript: Option<String>,
    pub ocr_results: Vec<String>,
    pub keywords: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentMetadata {
    pub overall: String,
    pub score: f64,
    pub segments: Vec<SentimentSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentSegment {
    pub start_time: f64,
    pub end_time: f64,
    pub sentiment: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectDetectionMetadata {
    pub objects: Vec<DetectedObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedObject {
    pub label: String,
    pub confidence: f64,
    pub bounding_box: BoundingBox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandDetectionMetadata {
    pub brands_detected: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolution {
    pub conflict_detected: bool,
    pub your_version: Option<String>,
    pub current_version: String,
    pub your_changes: serde_json::Value,
    pub their_changes: serde_json::Value,
    pub conflicting_fields: Vec<String>,
    pub requires_manual_review: bool,
}
