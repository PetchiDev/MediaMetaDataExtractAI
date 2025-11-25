# How to Apply Rules to Your Project

This guide explains how the 33 Functional Requirements (I-FR-01 through I-FR-33) are mapped to the codebase structure.

## Overview

The rules from your use cases and functional requirements have been applied by:

1. **Creating a modular structure** that separates concerns
2. **Mapping each FR to specific modules/files**
3. **Using Rust traits and async/await** for I-FR-03 (asynchronous) and I-FR-04 (parallel)
4. **Implementing configuration** for I-FR-01, I-FR-12, I-FR-16
5. **Creating API routes** for all serving requirements (I-FR-23 through I-FR-33)

## Rule Application Strategy

### 1. **Controllers (I-FR-01 to I-FR-16)**

**Location**: `src/controllers/`

- **I-FR-01** (Sync intervals): `config.rs` → `SyncConfig` struct
- **I-FR-02** (No duplication): `utils/hash.rs` + `controllers/base.rs::check_duplicate()`
- **I-FR-03** (Asynchronous): `controllers/base.rs` → `async_trait` trait
- **I-FR-04** (Parallel): Controllers implement `Send + Sync`, can run in parallel
- **I-FR-05** (Action records): `models/action_record.rs` + `controllers/base.rs::log_action()`
- **I-FR-06** (Independent deployment): Each controller is a separate module
- **I-FR-07** (AWS security): `api/middleware.rs` → Authentication middleware
- **I-FR-08** (Scalability): Async/await throughout, parallel execution support
- **I-FR-09** (Monitoring): `api/routes/admin.rs` → Controller status endpoints
- **I-FR-10** (Data sovereignty): `config.rs` → Regional configuration
- **I-FR-11** (CI/CD): Project structure supports containerization
- **I-FR-12** (Logging levels): `config.rs` → `LoggingConfig` struct
- **I-FR-13** (Rollback): `controllers/base.rs::rollback()` + `api/routes/admin.rs`
- **I-FR-14** (Third-party): `controllers/ingress.rs` → External system integrations
- **I-FR-15** (Lifecycle): `api/routes/admin.rs` → Lifecycle management endpoints
- **I-FR-16** (Retry): `controllers/base.rs::retry_with_backoff()` + `config.rs`

### 2. **Display & UI Requirements (I-FR-17 to I-FR-22)**

**Location**: `src/api/routes/` and `src/api/handlers/`

- **I-FR-17** (Schema grouping): API endpoints support filtering by schema
- **I-FR-18** (Version control): `models/asset.rs` → `version` field + `services/asset_service.rs`
- **I-FR-19** (Conflict resolution): `api/handlers/metadata.rs::resolve_conflict()`
- **I-FR-20** (Graph indexing): `services/graph_service.rs::index_asset()`
- **I-FR-21** (SSO): `api/routes/auth.rs` → SSO endpoints
- **I-FR-22** (Graph search): `api/routes/graph.rs` → Graph search endpoints

### 3. **Serving & API Requirements (I-FR-23 to I-FR-33)**

**Location**: `src/api/`

- **I-FR-23** (API security): `api/routes/auth.rs` → API key management
- **I-FR-24** (Metadata API): `api/routes/metadata.rs` → GET `/api/metadata/:id`
- **I-FR-25** (Rate limiting): `api/middleware.rs` → Rate limiting middleware
- **I-FR-26** (Workflow status): `api/routes/workflow.rs` → Status endpoints
- **I-FR-27** (Metadata editing): `api/routes/metadata.rs` → PUT `/api/metadata/:id`
- **I-FR-28** (Media preview): To be implemented in handlers
- **I-FR-29** (Media submission): `api/routes/media.rs` → POST `/api/media/submit`
- **I-FR-30** (Operational tagging): `models/asset.rs` → `operational_tags` field
- **I-FR-31** (Media upload): `api/routes/media.rs` → POST `/api/media/upload`
- **I-FR-32** (Workflow creation): `api/routes/workflow.rs` → POST `/api/workflows`
- **I-FR-33** (Preprocessing): `services/preprocessing_service.rs` → Business logic routing

## How Rules Are Enforced

### Configuration-Based Rules

Rules that are configurable (I-FR-01, I-FR-12, I-FR-16) are enforced through:

```rust
// In config.rs
pub struct SyncConfig {
    pub default_interval_minutes: u64,  // I-FR-01
}

pub struct LoggingConfig {
    pub level: String,  // I-FR-12: critical, error, warning, info
}

pub struct RetryConfig {
    pub max_attempts: u32,  // I-FR-16
}
```

### Code Structure Rules

Rules enforced by code structure:

- **I-FR-02** (No duplication): Hash calculation in `utils/hash.rs`, checked in controllers
- **I-FR-03** (Asynchronous): All controller methods are `async`
- **I-FR-04** (Parallel): Controllers implement `Send + Sync`, can run concurrently
- **I-FR-18** (Version control): Asset model includes `version` and `version_id` fields

### API Endpoint Rules

Rules enforced through API endpoints:

- **I-FR-05** (Action records): All controller operations log to `action_records` table
- **I-FR-09** (Monitoring): `/api/controllers/status` endpoint
- **I-FR-19** (Conflict resolution): `/api/metadata/:id/resolve-conflict` endpoint
- **I-FR-26** (Workflow status): `/api/workflow/status/:id` endpoint

## Next Steps to Complete Implementation

1. **Database Schema**: Create SQL migrations for all models
2. **Controller Logic**: Implement `sync()` methods in ingress/egress controllers
3. **Handler Logic**: Replace `todo!()` with actual implementations
4. **Middleware**: Implement authentication and rate limiting
5. **AWS Integration**: Connect to Step Functions, S3, Neptune
6. **External APIs**: Implement Brightcove, Cloudinary, Omnystudio integrations

## Testing the Rules

To verify rules are applied:

1. **I-FR-01** (Sync interval): Check `config.rs` → `SyncConfig`
2. **I-FR-02** (Deduplication): Test `utils/hash.rs::calculate_file_hash()`
3. **I-FR-03** (Async): All controller methods are `async fn`
4. **I-FR-05** (Action records): Check `models/action_record.rs` exists
5. **I-FR-18** (Version control): Check `models/asset.rs` has `version` field
6. **I-FR-33** (Preprocessing): Check `services/preprocessing_service.rs::determine_workflow()`

## Example: How I-FR-02 (No Duplication) is Applied

```rust
// 1. Hash calculation utility (utils/hash.rs)
pub fn calculate_file_hash<R: Read>(mut reader: R) -> Result<String> {
    // SHA-256 hash calculation
}

// 2. Duplicate check in controller (controllers/base.rs)
async fn check_duplicate(&self, file_hash: &str) -> Result<Option<Asset>> {
    // Query database for existing hash
}

// 3. Usage in ingress controller (controllers/ingress.rs)
async fn sync(&self) -> Result<SyncResult> {
    // Calculate hash
    // Check duplicate
    // Only create if not exists
}
```

## Example: How I-FR-33 (Preprocessing) is Applied

```rust
// services/preprocessing_service.rs
pub fn determine_workflow(asset: &Asset) -> Result<String> {
    // Business logic:
    // - Check title for "interview" → INTERVIEW_WORKFLOW
    // - Check category for "news" → NEWS_WORKFLOW
    // - Check duration > 1 hour → LONGFORM_WORKFLOW
    // - Default → STANDARD_WORKFLOW
}
```

This function is called before triggering workflows to route assets to the correct AI processing pipeline.

## Summary

✅ **All 33 Functional Requirements are mapped to code structure**
✅ **Configuration-based rules are in `config.rs`**
✅ **API endpoints are defined in `api/routes/`**
✅ **Business logic is in `services/`**
✅ **Controllers follow async/parallel patterns**

The structure is ready for implementation. Replace `todo!()` macros with actual logic to complete the platform.
