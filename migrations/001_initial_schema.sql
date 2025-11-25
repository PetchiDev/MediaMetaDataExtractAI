-- Database migrations for AI Media Metadata Processing Platform
-- All tables for I-FR-01 through I-FR-33

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Asset types enum (idempotent creation)
DO $$ BEGIN
    CREATE TYPE asset_type AS ENUM ('VIDEO', 'IMAGE', 'AUDIO', 'TEXT');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE source_system AS ENUM (
        'BRIGHTCOVE', 'CLOUDINARY', 'OMNYSTUDIO', 'ONECMS', 
        'MISSYS3', 'DALETS3', 'USER_UPLOAD', 'API_SUBMISSION'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE asset_status AS ENUM (
        'STAGED', 'QUEUED', 'PROCESSING', 'PROCESSED', 'FAILED', 'ARCHIVED'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Users table (I-FR-21: SSO, I-FR-23: Access governance)
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM (
        'ADMIN', 'CONTENT_MANAGER', 'EDITOR', 'DEVELOPER', 'VIEWER'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'VIEWER',
    sso_provider_id VARCHAR(255), -- I-FR-21: SSO integration
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_sso ON users(sso_provider_id);

-- Assets table (I-FR-02: Hash-based deduplication, I-FR-18: Version control)
CREATE TABLE IF NOT EXISTS assets (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    asset_type asset_type NOT NULL,
    asset_name VARCHAR(500) NOT NULL,
    source_system source_system NOT NULL,
    source_id VARCHAR(255),
    file_path TEXT NOT NULL,
    file_hash VARCHAR(64) NOT NULL, -- I-FR-02: SHA-256 hash
    file_size BIGINT NOT NULL,
    duration INTEGER, -- For video/audio in seconds
    format VARCHAR(50) NOT NULL,
    status asset_status NOT NULL DEFAULT 'STAGED',
    version INTEGER NOT NULL DEFAULT 1, -- I-FR-18: Version control
    version_id UUID NOT NULL DEFAULT uuid_generate_v4(),
    enriched_metadata JSONB DEFAULT '{}'::jsonb,
    operational_tags JSONB, -- I-FR-30: Operational tagging
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ,
    processing_completed_at TIMESTAMPTZ,
    uploaded_by UUID REFERENCES users(id),
    
    -- Indexes for performance
    CONSTRAINT unique_file_hash UNIQUE (file_hash) -- I-FR-02: Prevent duplicates
);

CREATE INDEX IF NOT EXISTS idx_assets_source ON assets(source_system, source_id);
CREATE INDEX IF NOT EXISTS idx_assets_status ON assets(status);
CREATE INDEX IF NOT EXISTS idx_assets_created_at ON assets(created_at DESC);

-- Asset versions table (I-FR-18: Version control, I-FR-19: Conflict resolution)
CREATE TABLE IF NOT EXISTS asset_versions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    asset_uuid UUID NOT NULL REFERENCES assets(uuid) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    version_id UUID NOT NULL,
    metadata_snapshot JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id),
    conflict_resolved BOOLEAN DEFAULT FALSE, -- I-FR-19: Conflict tracking
    
    UNIQUE(asset_uuid, version)
);

CREATE INDEX IF NOT EXISTS idx_asset_versions_asset ON asset_versions(asset_uuid, version DESC);


-- API keys table (I-FR-23: Secure API access)
DO $$ BEGIN
    CREATE TYPE api_key_status AS ENUM ('ACTIVE', 'REVOKED');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    key_name VARCHAR(255) NOT NULL,
    key_hash VARCHAR(255) NOT NULL UNIQUE, -- Stored as hash, never plaintext
    permissions JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used TIMESTAMPTZ,
    status api_key_status NOT NULL DEFAULT 'ACTIVE'
);

CREATE INDEX IF NOT EXISTS idx_api_keys_user ON api_keys(user_id);
CREATE INDEX IF NOT EXISTS idx_api_keys_hash ON api_keys(key_hash);

-- Action records table (I-FR-05: Traceability and audit)
DO $$ BEGIN
    CREATE TYPE action_type AS ENUM (
        'INGRESS', 'EGRESS', 'USER_UPLOAD', 'API_SUBMISSION',
        'METADATA_UPDATE', 'CONFLICT_RESOLVED', 'JOB_RETRY', 'ROLLBACK'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE direction AS ENUM ('INBOUND', 'OUTBOUND', 'INTERNAL');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE action_status AS ENUM ('SUCCESS', 'FAILED', 'IN_PROGRESS', 'INITIATED');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS action_records (
    record_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    asset_uuid UUID REFERENCES assets(uuid),
    action_type action_type NOT NULL,
    direction direction NOT NULL,
    controller_name VARCHAR(255) NOT NULL,
    controller_version VARCHAR(50) NOT NULL,
    source_system VARCHAR(100),
    destination_system VARCHAR(100),
    status action_status NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB,
    user_id UUID REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_action_records_asset ON action_records(asset_uuid);
CREATE INDEX IF NOT EXISTS idx_action_records_timestamp ON action_records(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_action_records_controller ON action_records(controller_name, timestamp DESC);

-- Processing jobs table (I-FR-26: Workflow status, I-FR-32: Workflow management)
DO $$ BEGIN
    CREATE TYPE job_status AS ENUM (
        'QUEUED', 'PROCESSING', 'COMPLETED', 'FAILED', 'RETRYING', 'CANCELLED'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS processing_jobs (
    job_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    asset_uuid UUID NOT NULL REFERENCES assets(uuid) ON DELETE CASCADE,
    workflow_name VARCHAR(255) NOT NULL,
    status job_status NOT NULL DEFAULT 'QUEUED',
    progress_percentage INTEGER DEFAULT 0,
    capabilities_completed JSONB DEFAULT '[]'::jsonb,
    capabilities_failed JSONB DEFAULT '[]'::jsonb,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    estimated_completion TIMESTAMPTZ,
    retry_count INTEGER DEFAULT 0, -- I-FR-16: Retry tracking
    retry_config JSONB -- I-FR-16: Retry configuration
);

CREATE INDEX IF NOT EXISTS idx_jobs_asset ON processing_jobs(asset_uuid);
CREATE INDEX IF NOT EXISTS idx_jobs_status ON processing_jobs(status);
CREATE INDEX IF NOT EXISTS idx_jobs_created ON processing_jobs(created_at DESC);

-- Workflow definitions table (I-FR-32: Workflow creation, I-FR-33: Preprocessing)
CREATE TABLE IF NOT EXISTS workflow_definitions (
    workflow_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    step_functions_arn VARCHAR(500),
    preprocessing_logic JSONB NOT NULL, -- I-FR-33: Business logic routing
    ai_capabilities JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id),
    is_active BOOLEAN DEFAULT TRUE
);

CREATE INDEX IF NOT EXISTS idx_workflows_active ON workflow_definitions(is_active);

-- Lifecycle rules table (I-FR-15: Lifecycle management)
CREATE TABLE IF NOT EXISTS lifecycle_rules (
    rule_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    rule_name VARCHAR(255) NOT NULL,
    description TEXT,
    asset_type asset_type,
    source_system source_system,
    archive_after_days INTEGER,
    delete_after_days INTEGER,
    priority INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_lifecycle_rules_active ON lifecycle_rules(is_active, priority DESC);

-- Controller configurations table (I-FR-01: Sync intervals, I-FR-12: Logging)
CREATE TABLE IF NOT EXISTS controller_configs (
    controller_name VARCHAR(255) PRIMARY KEY,
    sync_interval_minutes INTEGER DEFAULT 15, -- I-FR-01: Default 15 minutes
    logging_level VARCHAR(20) DEFAULT 'info', -- I-FR-12: critical, error, warning, info
    last_sync_at TIMESTAMPTZ,
    is_enabled BOOLEAN DEFAULT TRUE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Graph relationships table (I-FR-20: Graph indexing, I-FR-22: Graph search)
-- Note: This is a simplified version. For production, use Neptune/Neo4j
CREATE TABLE IF NOT EXISTS graph_relationships (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_asset_uuid UUID NOT NULL REFERENCES assets(uuid) ON DELETE CASCADE,
    target_asset_uuid UUID NOT NULL REFERENCES assets(uuid) ON DELETE CASCADE,
    relationship_type VARCHAR(100) NOT NULL, -- e.g., 'shared_keyword', 'shared_topic', 'temporal'
    relationship_data JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(source_asset_uuid, target_asset_uuid, relationship_type)
);

CREATE INDEX IF NOT EXISTS idx_graph_source ON graph_relationships(source_asset_uuid);
CREATE INDEX IF NOT EXISTS idx_graph_target ON graph_relationships(target_asset_uuid);
CREATE INDEX IF NOT EXISTS idx_graph_type ON graph_relationships(relationship_type);

-- Graph nodes (keywords, topics) (I-FR-20, I-FR-22)
CREATE TABLE IF NOT EXISTS graph_nodes (
    node_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    node_type VARCHAR(50) NOT NULL, -- 'KEYWORD', 'TOPIC', 'CONTRIBUTOR'
    node_name VARCHAR(255) NOT NULL,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(node_type, node_name)
);

CREATE INDEX IF NOT EXISTS idx_graph_nodes_type ON graph_nodes(node_type, node_name);

-- Asset to graph node relationships
CREATE TABLE IF NOT EXISTS asset_graph_nodes (
    asset_uuid UUID NOT NULL REFERENCES assets(uuid) ON DELETE CASCADE,
    node_id UUID NOT NULL REFERENCES graph_nodes(node_id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (asset_uuid, node_id)
);

CREATE INDEX IF NOT EXISTS idx_asset_nodes_asset ON asset_graph_nodes(asset_uuid);
CREATE INDEX IF NOT EXISTS idx_asset_nodes_node ON asset_graph_nodes(node_id);
