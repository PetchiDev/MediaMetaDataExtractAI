// OpenAPI specification for Swagger UI
// Documents all API endpoints (I-FR-23 through I-FR-33)

use utoipa::OpenApi;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, PartialSchema,
};

use crate::models::asset::Asset;
use crate::models::metadata::{EnrichedMetadata, MetadataUpdate};
use crate::models::workflow::ProcessingJob;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth endpoints
        crate::api::handlers::auth::google_login,
        crate::api::handlers::auth::google_callback,
        crate::api::handlers::auth::generate_api_key,
        // Media endpoints
        crate::api::handlers::media::submit_media,
        crate::api::handlers::media::upload_media,
        crate::api::handlers::media::get_media,
        crate::api::handlers::media::download_media,
        // Metadata endpoints
        crate::api::handlers::metadata::get_metadata,
        crate::api::handlers::metadata::update_metadata,
        crate::api::handlers::metadata::resolve_conflict,
        // Workflow endpoints
        crate::api::handlers::workflow::get_workflow_status,
        // Graph endpoints
        crate::api::handlers::graph::search,
        // Admin endpoints
        crate::api::handlers::admin::get_controller_status,
    ),
    components(schemas(
        Asset,
        EnrichedMetadata,
        MetadataUpdate,
        ProcessingJob,
        MediaSubmitResponse,
        MediaUploadResponse,
        MetadataResponse,
        ConflictResolutionRequest,
        WorkflowStatusResponse,
        GraphSearchResponse,
        ControllerStatusResponse,
        ApiKeyResponse,
        GoogleLoginResponse,
        SSOCallbackResponse,
        ErrorResponse,
    )),
    modifiers(&SecurityAddon),
    tags(
        (name = "Media", description = "Media upload and submission endpoints"),
        (name = "Metadata", description = "Metadata query and editing endpoints"),
        (name = "Workflow", description = "AI workflow status and management"),
        (name = "Graph", description = "Graph-based content discovery"),
        (name = "Admin", description = "Administrative and monitoring endpoints"),
        (name = "Auth", description = "Authentication and API key management"),
    ),
    info(
        title = "AI Media Metadata Processing Platform API",
        description = "Complete API for media ingestion, AI processing, and metadata management",
        version = "1.0.0",
        contact(
            name = "MediaCorp AI Platform",
            email = "api-support@mediacorp.com"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server"),
        (url = "https://api-ai.mediacorp.com", description = "Production server")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            );
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

// Response schemas
#[derive(utoipa::ToSchema)]
pub struct MediaSubmitResponse {
    pub asset_uuid: String,
    pub job_id: String,
    pub status: String,
    pub estimated_time_minutes: i32,
    pub status_url: String,
    pub metadata_url: String,
}

#[derive(utoipa::ToSchema)]
pub struct MediaUploadResponse {
    pub asset_uuid: String,
    pub status: String,
    pub message: String,
}

#[derive(utoipa::ToSchema)]
pub struct MetadataResponse {
    pub asset_uuid: String,
    pub metadata: serde_json::Value,
    pub version: i32,
}

#[derive(utoipa::ToSchema)]
pub struct ConflictResolutionRequest {
    pub resolved_metadata: serde_json::Value,
    pub resolution_strategy: String,
}

#[derive(utoipa::ToSchema)]
pub struct WorkflowStatusResponse {
    pub job_id: String,
    pub status: String,
    pub progress_percentage: i32,
    pub capabilities_completed: Vec<String>,
}

#[derive(utoipa::ToSchema)]
pub struct GraphSearchResponse {
    pub assets: Vec<serde_json::Value>,
    pub graph: serde_json::Value,
    pub total_results: i32,
}

#[derive(utoipa::ToSchema)]
pub struct ControllerStatusResponse {
    pub controllers: Vec<serde_json::Value>,
    pub timestamp: String,
}

#[derive(utoipa::ToSchema)]
pub struct ApiKeyResponse {
    pub api_key: String,
    pub key_id: String,
    pub expires_in: String,
    pub warning: String,
}

#[derive(utoipa::ToSchema)]
pub struct GoogleLoginResponse {
    pub redirect_url: String,
    pub callback_url: String,
    pub status: String,
    pub message: String,
    pub instructions: String,
}

#[derive(utoipa::ToSchema)]
pub struct SSOCallbackResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: serde_json::Value,
    pub status: String,
}

#[derive(utoipa::ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: i32,
}

