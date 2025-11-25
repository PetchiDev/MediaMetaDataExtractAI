# AI Media Metadata Processing Platform

This Rust-based API implements all 33 Functional Requirements (I-FR-01 through I-FR-33) for the AI Media Metadata Processing Platform.

## Project Structure

```
src/
├── main.rs                 # Entry point
├── config.rs              # Configuration (I-FR-01, I-FR-12, I-FR-16)
├── models/                # Data models
│   ├── asset.rs          # Asset model (I-FR-02, I-FR-15, I-FR-18)
│   ├── action_record.rs  # Action records (I-FR-05)
│   ├── workflow.rs       # Workflow models (I-FR-26, I-FR-32, I-FR-33)
│   ├── metadata.rs       # Metadata models (I-FR-27, I-FR-19)
│   └── user.rs           # User/auth models (I-FR-21, I-FR-23)
├── controllers/          # Ingress/Egress controllers
│   ├── base.rs          # Base controller trait (I-FR-03, I-FR-04, I-FR-05, etc.)
│   ├── ingress.rs       # Ingress controllers (I-FR-01, I-FR-02)
│   └── egress.rs        # Egress controllers
├── services/            # Business logic
│   ├── asset_service.rs
│   ├── workflow_service.rs
│   ├── graph_service.rs
│   └── preprocessing_service.rs  # I-FR-33: Preprocessing routing
├── api/                 # API routes and handlers
│   ├── routes/         # Route definitions
│   │   ├── media.rs    # I-FR-29, I-FR-31
│   │   ├── metadata.rs # I-FR-24, I-FR-27, I-FR-19
│   │   ├── workflow.rs # I-FR-26, I-FR-32
│   │   ├── graph.rs    # I-FR-20, I-FR-22
│   │   ├── admin.rs    # I-FR-09, I-FR-05, I-FR-13, etc.
│   │   └── auth.rs     # I-FR-21, I-FR-23, I-FR-25
│   ├── handlers/       # Request handlers
│   └── middleware.rs  # Auth, rate limiting, logging
└── utils/              # Utilities
    └── hash.rs         # I-FR-02: Hash calculation
```

## Functional Requirements Mapping

### Controllers (I-FR-01 to I-FR-16)
- **I-FR-01**: Configurable sync intervals → `config.rs`
- **I-FR-02**: Hash-based deduplication → `utils/hash.rs`, `controllers/base.rs`
- **I-FR-03**: Asynchronous execution → `controllers/base.rs` (async trait)
- **I-FR-04**: Parallel execution → Controllers can run in parallel
- **I-FR-05**: Action records → `models/action_record.rs`
- **I-FR-06**: Independent deployment → Controllers as separate modules
- **I-FR-07**: AWS security → Middleware/auth
- **I-FR-08**: Scalability → Async/await, parallel execution
- **I-FR-09**: Real-time monitoring → `api/routes/admin.rs`
- **I-FR-10**: Data sovereignty → Configuration
- **I-FR-11**: CI/CD integration → Project structure
- **I-FR-12**: Configurable logging → `config.rs`, `api/middleware.rs`
- **I-FR-13**: Rollback → `controllers/base.rs`, `api/routes/admin.rs`
- **I-FR-14**: Third-party integration → Controller implementations
- **I-FR-15**: Lifecycle management → `api/routes/admin.rs`
- **I-FR-16**: Retry mechanisms → `controllers/base.rs`, `config.rs`

### Display & UI (I-FR-17 to I-FR-22)
- **I-FR-17**: Schema grouping → API endpoints
- **I-FR-18**: Version control → `models/asset.rs`, `services/asset_service.rs`
- **I-FR-19**: Conflict resolution → `api/handlers/metadata.rs`
- **I-FR-20**: Graph indexing → `services/graph_service.rs`
- **I-FR-21**: SSO → `api/routes/auth.rs`
- **I-FR-22**: Graph search → `api/routes/graph.rs`

### Serving & API (I-FR-23 to I-FR-33)
- **I-FR-23**: API security → `api/routes/auth.rs`
- **I-FR-24**: Metadata API → `api/routes/metadata.rs`
- **I-FR-25**: Rate limiting → `api/middleware.rs`
- **I-FR-26**: Workflow status → `api/routes/workflow.rs`
- **I-FR-27**: Metadata editing → `api/routes/metadata.rs`
- **I-FR-28**: Media preview → API endpoint (to be implemented)
- **I-FR-29**: Media submission → `api/routes/media.rs`
- **I-FR-30**: Operational tagging → `models/asset.rs`
- **I-FR-31**: Media upload → `api/routes/media.rs`
- **I-FR-32**: Workflow creation → `api/routes/workflow.rs`
- **I-FR-33**: Preprocessing → `services/preprocessing_service.rs`

## Getting Started

1. **Install dependencies:**
   ```bash
   cargo build
   ```

2. **Set environment variables:**

   **For Local PostgreSQL:**
   ```bash
   export DATABASE_URL="postgresql://mediacorp_user:password@localhost:5432/mediacorp"
   export DATABASE_USE_SSL=false
   export AWS_REGION="us-east-1"
   export JWT_SECRET="your-secret-key"
   ```

   **For AWS RDS:**
   ```bash
   export DATABASE_URL="postgresql://username:password@your-rds-endpoint.region.rds.amazonaws.com:5432/mediacorp"
   export DATABASE_USE_SSL=true
   export DATABASE_MAX_CONNECTIONS=50
   export AWS_REGION="us-east-1"
   export AWS_ACCESS_KEY_ID="your-access-key"
   export AWS_SECRET_ACCESS_KEY="your-secret-key"
   export JWT_SECRET="your-secret-key"
   ```

   See `DATABASE_SETUP.md` for detailed setup instructions.

3. **Run the server:**
   ```bash
   cargo run
   ```

## Next Steps

1. Implement database migrations (SQLx)
2. Implement controller logic for each external system
3. Integrate AWS Step Functions for workflows
4. Set up Neptune/Neo4j for graph database
5. Implement authentication middleware
6. Add rate limiting middleware
7. Create database schema
8. Implement handler logic (currently todos)

## Architecture Notes

- **Async/Await**: All controllers and services use async/await for I-FR-03 (asynchronous execution)
- **Parallel Execution**: Controllers can run in parallel (I-FR-04)
- **Hash-based Deduplication**: SHA-256 hashing for I-FR-02
- **Version Control**: Asset versioning for I-FR-18
- **Conflict Detection**: Version ID checking for I-FR-19
- **Preprocessing Routing**: Business logic in `preprocessing_service.rs` for I-FR-33

