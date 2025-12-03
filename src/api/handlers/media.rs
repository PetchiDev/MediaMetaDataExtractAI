// Media handlers - FULL IMPLEMENTATION
// I-FR-29: Media submission (API)
// I-FR-31: Media upload (UI)

use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap, HeaderValue},
    response::{Json, Response, IntoResponse},
    body::Bytes,
};
use axum_extra::extract::multipart::Multipart;
use uuid::Uuid;
use crate::db::DbPool;
use crate::api::openapi::{MediaSubmitResponse, MediaUploadResponse};
use crate::db::repositories::{asset_repository::AssetRepository, workflow_repository::WorkflowRepository};
use crate::models::asset::{Asset, AssetStatus, AssetType, SourceSystem};
use crate::utils::hash;
use crate::aws::s3::S3Service;
use crate::services::preprocessing_service;
use crate::services::local_storage::LocalStorageService;
use crate::services::ai_processing::AIProcessingService;
use chrono::Utc;
use serde_json::json;

/// Submit media for AI processing (Technical Users)
/// 
/// I-FR-29: Media submission and ingestion endpoint
/// 
/// Accepts multipart form data with:
/// - `file`: Media file (video, image, audio, or text)
/// - `metadata`: JSON metadata (optional)
/// - `operational_tags`: JSON operational tags for downstream processing (optional)
#[utoipa::path(
    post,
    path = "/api/media/submit",
    tag = "Media",
    request_body(content = String, description = "Multipart form data with file, metadata, and operational_tags", content_type = "multipart/form-data"),
    responses(
        (status = 202, description = "Media submitted successfully", body = MediaSubmitResponse),
        (status = 400, description = "Bad request - missing file or invalid data", body = ErrorResponse),
        (status = 409, description = "Duplicate asset detected", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("api_key" = []),
        ("bearer_auth" = [])
    )
)]
// I-FR-29: Technical user API submission
pub async fn submit_media(
    State(db_pool): State<DbPool>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut metadata_json: Option<String> = None;
    let mut operational_tags_json: Option<String> = None;
    let mut filename: Option<String> = None;

    // Parse multipart form
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Multipart parsing error: {:?}", e);
        StatusCode::BAD_REQUEST
    })? {
        let name = field.name().unwrap_or("");
        
        match name {
            "file" => {
                filename = field.file_name().map(|s| s.to_string());
                let data = field.bytes().await.map_err(|e| {
                    tracing::error!("Error reading file bytes: {:?}", e);
                    StatusCode::BAD_REQUEST
                })?;
                file_data = Some(data.to_vec());
            }
            "metadata" => {
                let data = field.text().await.map_err(|e| {
                    tracing::warn!("Error reading metadata field: {:?}", e);
                    StatusCode::BAD_REQUEST
                })?;
                metadata_json = Some(data);
            }
            "operational_tags" => {
                let data = field.text().await.map_err(|e| {
                    tracing::warn!("Error reading operational_tags field: {:?}", e);
                    StatusCode::BAD_REQUEST
                })?;
                operational_tags_json = Some(data);
            }
            _ => {
                tracing::debug!("Ignoring unknown field: {}", name);
            }
        }
    }

    let file_data = file_data.ok_or_else(|| {
        tracing::error!("Missing 'file' field in multipart form");
        StatusCode::BAD_REQUEST
    })?;
    let filename = filename.ok_or_else(|| {
        tracing::error!("Missing filename in file field");
        StatusCode::BAD_REQUEST
    })?;
    
    // I-FR-02: Calculate hash for deduplication
    let file_hash = hash::calculate_file_hash_async(&file_data).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check for duplicate
    if let Ok(Some(existing)) = AssetRepository::find_by_hash(&db_pool, &file_hash).await {
        return Ok(Json(json!({
            "asset_uuid": existing.uuid,
            "status": "DUPLICATE",
            "message": "Asset with same hash already exists"
        })));
    }

    // Upload to storage (local for testing, S3 for production)
    let file_size = file_data.len() as i64;
    let storage_path = if std::env::var("USE_LOCAL_STORAGE").is_ok() || std::env::var("USE_LOCAL_STORAGE").is_err() {
        // Use local storage (default for testing)
        let local_storage = LocalStorageService::new(
            std::env::var("LOCAL_STORAGE_PATH").ok()
        );
        local_storage.save_file("api-submissions", &filename, file_data.clone()).await
            .map_err(|e| {
                tracing::error!("Failed to save file to local storage: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    } else {
        // Use S3 for production
        let aws_config = aws_config::load_from_env().await;
        let s3_service = S3Service::new(&aws_config, "mediacorp-ai-api-uploads".to_string());
        let s3_key = s3_service.generate_key("api-submissions", &filename);
        s3_service.upload_file(&s3_key, file_data).await
            .map_err(|e| {
                tracing::error!("Failed to upload file to S3: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
    };
    
    tracing::info!("File saved to: {}", storage_path);

    // Parse metadata
    let metadata: serde_json::Value = metadata_json
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| json!({}));
    
    let operational_tags: Option<serde_json::Value> = operational_tags_json
        .and_then(|s| serde_json::from_str(&s).ok());

    // Determine asset type from filename extension (support all formats)
    let asset_type = {
        let ext = filename.split('.').last().unwrap_or("").to_lowercase();
        match ext.as_str() {
            // Video formats
            "mp4" | "avi" | "mov" | "mkv" | "webm" | "flv" | "wmv" | "m4v" | "3gp" => AssetType::Video,
            // Image formats
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" | "ico" | "tiff" | "tif" => AssetType::Image,
            // Audio formats
            "mp3" | "wav" | "m4a" | "aac" | "flac" | "ogg" | "wma" | "opus" | "amr" => AssetType::Audio,
            // Text formats
            "pdf" | "txt" | "doc" | "docx" | "html" | "json" | "xml" | "csv" | "md" | "rtf" | "odt" => AssetType::Text,
            _ => {
                tracing::warn!("Unknown file extension: {}, defaulting to Text", ext);
                AssetType::Text
            }
        }
    };

    // Create asset
    let asset_uuid = Uuid::new_v4();
    let asset_type_clone = asset_type.clone();
    let asset = Asset {
        uuid: asset_uuid,
        asset_type,
        asset_name: filename.clone(),
        source_system: SourceSystem::ApiSubmission,
        source_id: None,
        file_path: storage_path.clone(),
        file_hash,
        file_size,
        duration: metadata.get("duration_seconds").and_then(|d| d.as_i64()).map(|d| d as i32),
        format: filename.split('.').last().unwrap_or("unknown").to_uppercase(),
        status: AssetStatus::Queued,
        version: 1,
        version_id: Uuid::new_v4(),
        enriched_metadata: metadata.clone(),
        operational_tags,
        created_at: Utc::now(),
        updated_at: None,
        processing_completed_at: None,
        uploaded_by: None, // TODO: Get from auth middleware
    };

    AssetRepository::create(&db_pool, &asset).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // I-FR-33: Determine workflow based on preprocessing logic
    let workflow_name = preprocessing_service::determine_workflow(&asset)
        .unwrap_or_else(|_| "STANDARD_WORKFLOW".to_string());

    // Create processing job
    let job_id = Uuid::new_v4();
    let job = crate::models::workflow::ProcessingJob {
        job_id,
        asset_uuid,
        workflow_name: workflow_name.clone(),
        status: crate::models::workflow::JobStatus::Queued,
        progress_percentage: 0,
        capabilities_completed: vec![],
        capabilities_failed: vec![],
        error_message: None,
        created_at: Utc::now(),
        started_at: None,
        completed_at: None,
        estimated_completion: Some(Utc::now() + chrono::Duration::minutes(30)),
        retry_count: 0,
        retry_config: None,
    };

    WorkflowRepository::create_job(&db_pool, &job).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Trigger AI processing (local processing for testing)
    let db_pool_clone = db_pool.clone();
    let asset_uuid_clone = asset_uuid;
    let workflow_name_clone = workflow_name.clone();
    let asset_type_str = format!("{:?}", asset_type_clone);
    let storage_path_clone = storage_path.clone();
    tokio::spawn(async move {
        // Update job status to processing
        WorkflowRepository::update_job_status(
            &db_pool_clone,
            job_id,
            crate::models::workflow::JobStatus::Processing,
            Some(0), // Progress percentage
            None,
        ).await.ok();
        
        // Process with AI
        let ai_results = AIProcessingService::process_asset(
            &storage_path_clone,
            &asset_type_str,
        ).await;
        
        if let Ok(enriched_metadata) = ai_results {
            // Update asset with enriched metadata
            let current_asset = AssetRepository::get_by_uuid(&db_pool_clone, asset_uuid_clone).await;
            if let Ok(Some(mut asset)) = current_asset {
                // Merge AI results with existing metadata
                let mut metadata = asset.enriched_metadata.clone();
                for (key, value) in enriched_metadata.as_object().unwrap() {
                    metadata[key] = value.clone();
                }
                
                // Update asset
                AssetRepository::update_metadata(
                    &db_pool_clone,
                    asset_uuid_clone,
                    metadata,
                    asset.version,
                    asset.version_id,
                    uuid::Uuid::new_v4(), // TODO: Get from auth
                ).await.ok();
                
                    // Update asset status with completion timestamp
                    AssetRepository::update_status(
                        &db_pool_clone,
                        asset_uuid_clone,
                        AssetStatus::Processed,
                        Some(Utc::now())
                    ).await.ok();
                
                // Update job status
                WorkflowRepository::update_job_status(
                    &db_pool_clone,
                    job_id,
                    crate::models::workflow::JobStatus::Completed,
                    Some(100), // 100% complete
                    None, // No error
                ).await.ok();
            }
        }
    });

    Ok(Json(json!({
        "asset_uuid": asset_uuid,
        "job_id": job_id,
        "status": "QUEUED",
        "workflow": workflow_name,
        "estimated_time_minutes": 30,
        "status_url": format!("/api/jobs/{}", job_id),
        "metadata_url": format!("/api/metadata/{}", asset_uuid)
    })))
}

/// Upload media via UI (Naive Users)
/// 
/// I-FR-31: Media upload and ingestion
/// 
/// Similar to submit_media but designed for UI-based uploads
#[utoipa::path(
    post,
    path = "/api/media/upload",
    tag = "Media",
    request_body(content = String, description = "Multipart form data with file and basic metadata", content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Media uploaded successfully", body = MediaUploadResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("api_key" = []),
        ("bearer_auth" = [])
    )
)]
// I-FR-31: Naive user manual upload
pub async fn upload_media(
    State(db_pool): State<DbPool>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut tags: Option<String> = None;
    let mut category: Option<String> = None;

    // Parse multipart form (UI-friendly format)
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("");
        
        match name {
            "file" => {
                filename = field.file_name().map(|s| s.to_string());
                let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                file_data = Some(data.to_vec());
            }
            "title" => {
                title = field.text().await.ok();
            }
            "description" => {
                description = field.text().await.ok();
            }
            "tags" => {
                tags = field.text().await.ok();
            }
            "category" => {
                category = field.text().await.ok();
            }
            // Also support metadata as JSON (for advanced users)
            "metadata" => {
                // Will be handled below
            }
            _ => {}
        }
    }

    let file_data = file_data.ok_or(StatusCode::BAD_REQUEST)?;
    let filename = filename.ok_or(StatusCode::BAD_REQUEST)?;
    
    // I-FR-02: Calculate hash for deduplication
    let file_hash = hash::calculate_file_hash_async(&file_data).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check for duplicate
    if let Ok(Some(existing)) = AssetRepository::find_by_hash(&db_pool, &file_hash).await {
        return Ok(Json(json!({
            "asset_uuid": existing.uuid,
            "status": "DUPLICATE",
            "message": "Asset with same hash already exists"
        })));
    }

    // Upload to local storage (for local testing)
    let file_size = file_data.len() as i64;
    let storage_path = if std::env::var("USE_LOCAL_STORAGE").is_ok() {
        let local_storage = LocalStorageService::new(
            std::env::var("LOCAL_STORAGE_PATH").ok()
        );
        local_storage.save_file("uploads", &filename, file_data.clone()).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        // Use S3 for production
        let aws_config = aws_config::load_from_env().await;
        let s3_service = S3Service::new(&aws_config, "mediacorp-ai-api-uploads".to_string());
        let s3_key = s3_service.generate_key("user-uploads", &filename);
        s3_service.upload_file(&s3_key, file_data).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    // Build metadata from form fields
    let mut metadata = json!({});
    if let Some(t) = title {
        metadata["title"] = json!(t);
    }
    if let Some(d) = description {
        metadata["description"] = json!(d);
    }
    if let Some(tags_str) = tags {
        let tag_list: Vec<String> = tags_str.split(',').map(|s| s.trim().to_string()).collect();
        metadata["tags"] = json!(tag_list);
    }
    if let Some(c) = category {
        metadata["category"] = json!(c);
    }

    // Determine asset type from filename extension (support more formats)
    let asset_type = {
        let ext = filename.split('.').last().unwrap_or("").to_lowercase();
        match ext.as_str() {
            // Video formats
            "mp4" | "avi" | "mov" | "mkv" | "webm" | "flv" | "wmv" => AssetType::Video,
            // Image formats
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" | "ico" => AssetType::Image,
            // Audio formats
            "mp3" | "wav" | "m4a" | "aac" | "flac" | "ogg" | "wma" => AssetType::Audio,
            // Text formats
            "pdf" | "txt" | "doc" | "docx" | "html" | "json" | "xml" | "csv" | "md" => AssetType::Text,
            _ => AssetType::Text, // Default to text for unknown formats
        }
    };

    // Create asset record
    let asset_uuid = Uuid::new_v4();
    let asset_type_clone = asset_type.clone();
    let asset = Asset {
        uuid: asset_uuid,
        asset_type,
        asset_name: filename.clone(),
        source_system: SourceSystem::UserUpload,
        source_id: None,
        file_path: storage_path.clone(),
        file_hash,
        file_size,
        duration: metadata.get("duration_seconds").and_then(|d| d.as_i64()).map(|d| d as i32),
        format: filename.split('.').last().unwrap_or("unknown").to_uppercase(),
        status: AssetStatus::Queued,
        version: 1,
        version_id: Uuid::new_v4(),
        enriched_metadata: metadata.clone(),
        operational_tags: None,
        created_at: Utc::now(),
        updated_at: None,
        processing_completed_at: None,
        uploaded_by: None, // TODO: Get from auth middleware
    };

    // Save to database
    AssetRepository::create(&db_pool, &asset).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // I-FR-33: Determine workflow based on preprocessing logic
    let workflow_name = preprocessing_service::determine_workflow(&asset)
        .unwrap_or_else(|_| "STANDARD_WORKFLOW".to_string());

    // Create processing job
    let job_id = Uuid::new_v4();
    let job = crate::models::workflow::ProcessingJob {
        job_id,
        asset_uuid,
        workflow_name: workflow_name.clone(),
        status: crate::models::workflow::JobStatus::Queued,
        progress_percentage: 0,
        capabilities_completed: vec![],
        capabilities_failed: vec![],
        error_message: None,
        created_at: Utc::now(),
        started_at: None,
        completed_at: None,
        estimated_completion: Some(Utc::now() + chrono::Duration::minutes(30)),
        retry_count: 0,
        retry_config: None,
    };

    WorkflowRepository::create_job(&db_pool, &job).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Trigger AI processing in background
    let db_pool_clone = db_pool.clone();
    let asset_uuid_clone = asset_uuid;
    let workflow_name_clone = workflow_name.clone();
    let asset_type_str = format!("{:?}", asset_type_clone);
    let storage_path_clone = storage_path.clone();
    tokio::spawn(async move {
        // Update job status to processing
        WorkflowRepository::update_job_status(
            &db_pool_clone,
            job_id,
            crate::models::workflow::JobStatus::Processing,
            Some(0),
            None,
        ).await.ok();
        
        // Process with AI
        let ai_results = AIProcessingService::process_asset(
            &storage_path_clone,
            &asset_type_str,
        ).await;
        
        if let Ok(enriched_metadata) = ai_results {
            // Update asset with enriched metadata
            if let Ok(Some(mut asset)) = AssetRepository::get_by_uuid(&db_pool_clone, asset_uuid_clone).await {
                // Merge AI results with existing metadata
                let mut current_metadata = asset.enriched_metadata.clone();
                if let Some(obj) = current_metadata.as_object_mut() {
                    if let Some(ai_obj) = enriched_metadata.as_object() {
                        for (key, value) in ai_obj {
                            obj.insert(key.clone(), value.clone());
                        }
                    }
                }
                
                asset.enriched_metadata = current_metadata;
                asset.status = AssetStatus::Processed;
                asset.processing_completed_at = Some(Utc::now());
                
                AssetRepository::update(&db_pool_clone, &asset).await.ok();
            }
            
            // Update job status to completed
            WorkflowRepository::update_job_status(
                &db_pool_clone,
                job_id,
                crate::models::workflow::JobStatus::Completed,
                Some(100),
                None,
            ).await.ok();
        } else {
            // Update job status to failed
            WorkflowRepository::update_job_status(
                &db_pool_clone,
                job_id,
                crate::models::workflow::JobStatus::Failed,
                None,
                Some("AI processing failed".to_string()),
            ).await.ok();
        }
    });

    Ok(Json(json!({
        "asset_uuid": asset_uuid,
        "status": "PROCESSING",
        "message": "Upload successful. AI processing started.",
        "file_saved": true,
        "metadata_saved": true,
        "workflow": workflow_name,
        "job_id": job_id
    })))
}

/// Get media asset information
/// 
/// Retrieves asset details by UUID
#[utoipa::path(
    get,
    path = "/api/media/{asset_id}",
    tag = "Media",
    params(
        ("asset_id" = Uuid, Path, description = "Asset UUID")
    ),
    responses(
        (status = 200, description = "Asset found", body = Asset),
        (status = 404, description = "Asset not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("api_key" = []),
        ("bearer_auth" = [])
    )
)]
/// Get media asset information (metadata only)
pub async fn get_media(
    State(db_pool): State<DbPool>,
    Path(asset_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let asset = AssetRepository::get_by_uuid(&db_pool, asset_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(json!({
        "uuid": asset.uuid,
        "asset_type": format!("{:?}", asset.asset_type),
        "asset_name": asset.asset_name,
        "status": format!("{:?}", asset.status),
        "file_path": asset.file_path,
        "enriched_metadata": asset.enriched_metadata,
        "created_at": asset.created_at,
        "download_url": format!("/api/media/{}/download", asset_id),
    })))
}

/// Download/Stream media file (Video, Audio, Image, Text)
/// 
/// Returns the actual file content for playback/download
#[utoipa::path(
    get,
    path = "/api/media/{asset_id}/download",
    tag = "Media",
    params(
        ("asset_id" = Uuid, Path, description = "Asset UUID")
    ),
    responses(
        (status = 200, description = "File content", content_type = "video/mp4, audio/mpeg, image/png, application/pdf"),
        (status = 404, description = "Asset not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("api_key" = []),
        ("bearer_auth" = [])
    )
)]
pub async fn download_media(
    State(db_pool): State<DbPool>,
    Path(asset_id): Path<Uuid>,
) -> Result<Response, StatusCode> {
    // Get asset from database
    let asset = AssetRepository::get_by_uuid(&db_pool, asset_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Read file from storage
    let file_data = if std::env::var("USE_LOCAL_STORAGE").is_ok() {
        // Read from local storage
        let local_storage = LocalStorageService::new(
            std::env::var("LOCAL_STORAGE_PATH").ok()
        );
        local_storage.read_file(&asset.file_path).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        // Read from S3
        let aws_config = aws_config::load_from_env().await;
        let s3_service = S3Service::new(&aws_config, "mediacorp-ai-api-uploads".to_string());
        
        // Extract key from S3 path (format: s3://bucket/key)
        let key = asset.file_path
            .strip_prefix("s3://")
            .and_then(|s| s.splitn(2, '/').nth(1))
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        
        s3_service.download_file(key).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    // Determine content type based on asset type and format
    let content_type = match asset.asset_type {
        AssetType::Video => {
            match asset.format.as_str() {
                "MP4" => "video/mp4",
                "AVI" => "video/x-msvideo",
                "MOV" => "video/quicktime",
                _ => "video/mp4",
            }
        }
        AssetType::Audio => {
            match asset.format.as_str() {
                "MP3" => "audio/mpeg",
                "WAV" => "audio/wav",
                "M4A" => "audio/mp4",
                _ => "audio/mpeg",
            }
        }
        AssetType::Image => {
            match asset.format.as_str() {
                "PNG" => "image/png",
                "JPG" | "JPEG" => "image/jpeg",
                "GIF" => "image/gif",
                _ => "image/png",
            }
        }
        AssetType::Text => {
            match asset.format.as_str() {
                "PDF" => "application/pdf",
                "TXT" => "text/plain",
                "HTML" => "text/html",
                "JSON" => "application/json",
                "DOC" | "DOCX" => "application/msword",
                _ => "application/octet-stream",
            }
        }
    };

    // Set headers for file download/streaming
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_str(content_type).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("inline; filename=\"{}\"", asset.asset_name))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    headers.insert(
        axum::http::header::CONTENT_LENGTH,
        HeaderValue::from_str(&file_data.len().to_string())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    
    // For video/audio, add range support headers for streaming
    if matches!(asset.asset_type, AssetType::Video | AssetType::Audio) {
        headers.insert(
            axum::http::header::ACCEPT_RANGES,
            HeaderValue::from_static("bytes"),
        );
    }

    // Return file as response
    Ok((headers, Bytes::from(file_data)).into_response())
}