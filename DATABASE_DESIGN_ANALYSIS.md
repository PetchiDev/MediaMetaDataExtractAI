# Database Design Analysis - AI Media Metadata Processing Platform

## Overview
This document provides a comprehensive analysis of the PostgreSQL database schema, explaining the purpose and functionality of each table in the AI Media Metadata Processing Platform.

---

## Table of Contents
1. [Core Tables](#core-tables)
2. [User & Authentication Tables](#user--authentication-tables)
3. [Workflow & Processing Tables](#workflow--processing-tables)
4. [Configuration & Management Tables](#configuration--management-tables)
5. [Graph Database Tables](#graph-database-tables)
6. [Enums & Types](#enums--types)

---

## Core Tables

### 1. `assets` Table
**Purpose:** Central table storing all media assets (videos, images, audio, text files) in the system.

**Key Functionality:**
- **I-FR-02 (Hash-based Deduplication)**: Stores `file_hash` (SHA-256) with UNIQUE constraint to prevent duplicate files
- **I-FR-18 (Version Control)**: Tracks `version` and `version_id` for metadata changes
- **I-FR-30 (Operational Tagging)**: Stores `operational_tags` JSONB for downstream business operations
- **Asset Lifecycle**: Tracks status from STAGED → QUEUED → PROCESSING → PROCESSED/FAILED
- **Source Tracking**: Links assets to their origin system (Brightcove, Cloudinary, etc.) via `source_system` and `source_id`
- **AI Enrichment**: Stores `enriched_metadata` JSONB containing all AI-processed data (speakers, transcripts, sentiment, etc.)

**Important Columns:**
- `uuid`: Primary key, unique identifier for each asset
- `file_hash`: SHA-256 hash for duplicate detection
- `enriched_metadata`: JSONB storing AI processing results
- `operational_tags`: JSONB for business-specific metadata (broadcast_priority, target_platforms, etc.)
- `status`: Current processing state
- `version` & `version_id`: For conflict detection and rollback

**Relationships:**
- References `users` table (uploaded_by)
- Referenced by `asset_versions`, `processing_jobs`, `action_records`, `graph_relationships`

---

### 2. `asset_versions` Table
**Purpose:** Maintains historical snapshots of asset metadata for version control and rollback capabilities.

**Key Functionality:**
- **I-FR-18 (Version Control)**: Stores complete metadata snapshot for each version
- **I-FR-19 (Conflict Resolution)**: Tracks `conflict_resolved` flag when conflicts are manually resolved
- **I-FR-13 (Rollback)**: Enables reverting assets to previous versions
- **Audit Trail**: Records who created each version and when

**Important Columns:**
- `asset_uuid`: Foreign key to assets table
- `version`: Version number (1, 2, 3, etc.)
- `version_id`: Unique identifier for this specific version
- `metadata_snapshot`: Complete JSONB copy of enriched_metadata at this version
- `conflict_resolved`: Boolean flag indicating if this version resolved a conflict

**Use Cases:**
- When user edits metadata, current version is archived here
- When conflict detected, both versions stored for comparison
- Admin can rollback to any previous version
- UI shows version history timeline

**Relationships:**
- References `assets` table (ON DELETE CASCADE)
- References `users` table (created_by)

---

## User & Authentication Tables

### 3. `users` Table
**Purpose:** Stores user accounts with role-based access control and SSO integration.

**Key Functionality:**
- **I-FR-21 (SSO Enablement)**: Links users to SSO provider via `sso_provider_id`
- **I-FR-23 (Access Governance)**: Role-based permissions (ADMIN, CONTENT_MANAGER, EDITOR, DEVELOPER, VIEWER)
- **User Management**: Tracks user creation and last login timestamps

**Important Columns:**
- `id`: Primary key (UUID)
- `email`: Unique identifier for login
- `role`: Determines permissions and access levels
- `sso_provider_id`: External SSO system identifier

**Relationships:**
- Referenced by `assets` (uploaded_by)
- Referenced by `api_keys` (user_id)
- Referenced by `asset_versions` (created_by)
- Referenced by `action_records` (user_id)

---

### 4. `api_keys` Table
**Purpose:** Manages API keys for technical users (developers, system integrators) to access the platform programmatically.

**Key Functionality:**
- **I-FR-23 (Secure API Access)**: Stores hashed API keys (never plaintext)
- **Permission Management**: JSONB array of permissions per key (e.g., ["submit:media", "read:metadata"])
- **Key Lifecycle**: Tracks creation, last usage, and revocation status
- **Security**: Uses `key_hash` instead of storing plaintext keys

**Important Columns:**
- `id`: Primary key
- `user_id`: Owner of the API key
- `key_name`: Human-readable name for the key
- `key_hash`: SHA-256 hash of the actual API key (plaintext shown only once at creation)
- `permissions`: JSONB array of allowed operations
- `status`: ACTIVE or REVOKED

**Security Notes:**
- API keys are hashed using SHA-256 before storage
- Plaintext key shown only once during generation
- Keys can be revoked without deletion (soft delete via status)

**Relationships:**
- References `users` table (ON DELETE CASCADE)

---

## Workflow & Processing Tables

### 5. `processing_jobs` Table
**Purpose:** Tracks AI workflow execution status for each asset, providing real-time visibility into processing progress.

**Key Functionality:**
- **I-FR-26 (Workflow Visibility)**: Shows which AI capabilities completed/failed
- **I-FR-16 (Retry Mechanisms)**: Tracks retry_count and retry_config for failed jobs
- **Progress Tracking**: Stores progress_percentage and timestamps for each stage
- **Error Handling**: Captures error_message when workflows fail

**Important Columns:**
- `job_id`: Primary key, unique job identifier
- `asset_uuid`: Asset being processed
- `workflow_name`: Which workflow is executing (INTERVIEW_WORKFLOW, NEWS_WORKFLOW, etc.)
- `status`: QUEUED → PROCESSING → COMPLETED/FAILED/RETRYING
- `capabilities_completed`: JSONB array of completed AI steps (e.g., ["SpeakerIndexing", "TextRecognition"])
- `capabilities_failed`: JSONB array of failed steps
- `retry_count`: Number of retry attempts
- `retry_config`: JSONB storing retry strategy (interval, max_attempts, etc.)

**Workflow States:**
- `QUEUED`: Job created, waiting to start
- `PROCESSING`: Currently executing AI capabilities
- `COMPLETED`: All capabilities finished successfully
- `FAILED`: One or more capabilities failed
- `RETRYING`: Job being retried after failure
- `CANCELLED`: Manually cancelled by admin

**Relationships:**
- References `assets` table (ON DELETE CASCADE)

---

### 6. `workflow_definitions` Table
**Purpose:** Stores AI workflow templates that define which AI capabilities to run and how to route assets to workflows.

**Key Functionality:**
- **I-FR-32 (Workflow Creation)**: Admins can create new workflows via API/UI
- **I-FR-33 (Preprocessing Routing)**: Stores `preprocessing_logic` JSONB that defines routing rules
- **AWS Integration**: Links to AWS Step Functions via `step_functions_arn`
- **Capability Management**: Defines which AI capabilities each workflow includes

**Important Columns:**
- `workflow_id`: Primary key
- `workflow_name`: Unique name (e.g., "INTERVIEW_WORKFLOW", "NEWS_WORKFLOW")
- `step_functions_arn`: AWS Step Functions state machine ARN
- `preprocessing_logic`: JSONB containing routing rules:
  ```json
  {
    "conditions": [
      {"field": "title", "contains": "interview", "workflow": "INTERVIEW_WORKFLOW"},
      {"field": "category", "equals": "news", "workflow": "NEWS_WORKFLOW"}
    ]
  }
  ```
- `ai_capabilities`: JSONB array of capabilities (e.g., ["SpeakerIndexing", "TextRecognition", "Sentiment"])
- `is_active`: Whether workflow is currently enabled

**Use Cases:**
- Preprocessing service reads this table to determine which workflow to assign
- Admin creates new workflows via Workflow Studio UI
- Workflows can be enabled/disabled without deletion

**Relationships:**
- References `users` table (created_by)

---

## Configuration & Management Tables

### 7. `action_records` Table
**Purpose:** Comprehensive audit trail logging all system operations for traceability and compliance.

**Key Functionality:**
- **I-FR-05 (Action Record Generation)**: Every ingress/egress operation creates a record
- **Audit Trail**: Complete history of all asset migrations, updates, and system actions
- **Controller Tracking**: Records which controller performed the action and its version
- **Compliance**: Supports regulatory requirements and incident investigation

**Important Columns:**
- `record_id`: Primary key
- `asset_uuid`: Optional link to specific asset (some actions are system-wide)
- `action_type`: INGRESS, EGRESS, USER_UPLOAD, API_SUBMISSION, METADATA_UPDATE, etc.
- `direction`: INBOUND (from external), OUTBOUND (to external), INTERNAL (within system)
- `controller_name`: Which controller executed (e.g., "BrightcoveIngressController")
- `controller_version`: Version of controller (e.g., "v2.3.1")
- `source_system` & `destination_system`: External systems involved
- `status`: SUCCESS, FAILED, IN_PROGRESS, INITIATED
- `metadata`: JSONB storing additional context (processing_time_ms, batch_size, etc.)

**Action Types:**
- `INGRESS`: Asset imported from external system
- `EGRESS`: Enriched metadata pushed to external system
- `USER_UPLOAD`: Manual file upload via UI
- `API_SUBMISSION`: Media submitted via API
- `METADATA_UPDATE`: User edited metadata
- `CONFLICT_RESOLVED`: Conflict manually resolved
- `JOB_RETRY`: Failed job retried
- `ROLLBACK`: Asset rolled back to previous version

**Use Cases:**
- Admin dashboard shows recent activity
- Troubleshooting failed operations
- Compliance audits
- Performance monitoring (query by controller_name, timestamp)

**Relationships:**
- References `assets` table (optional, nullable)
- References `users` table (optional, nullable)

---

### 8. `lifecycle_rules` Table
**Purpose:** Defines automated rules for archiving and deleting assets based on age, type, or source system.

**Key Functionality:**
- **I-FR-15 (Lifecycle Management)**: Automates asset archiving and deletion
- **Rule-Based**: Supports multiple rules with priority ordering
- **Flexible Criteria**: Can target specific asset types or source systems
- **Time-Based**: Archive after X days, delete after Y days

**Important Columns:**
- `rule_id`: Primary key
- `rule_name`: Human-readable name
- `asset_type`: Optional filter by type (VIDEO, IMAGE, AUDIO, TEXT)
- `source_system`: Optional filter by source (BRIGHTCOVE, CLOUDINARY, etc.)
- `archive_after_days`: Days until asset moves to ARCHIVED status
- `delete_after_days`: Days until asset is permanently deleted
- `priority`: Rule execution order (higher priority = executed first)
- `is_active`: Whether rule is currently enabled

**Example Rules:**
- Rule 1: Archive all videos from Brightcove after 365 days
- Rule 2: Delete archived assets after 730 days (2 years)
- Rule 3: Archive user uploads after 90 days

**Relationships:**
- References `users` table (created_by)

---

### 9. `controller_configs` Table
**Purpose:** Stores configuration settings for each ingress/egress controller (sync intervals, logging levels).

**Key Functionality:**
- **I-FR-01 (Sync Intervals)**: Configurable sync frequency per controller (default 15 minutes)
- **I-FR-12 (Logging Levels)**: Per-controller logging configuration (critical, error, warning, info)
- **Controller Management**: Enable/disable controllers without code changes
- **Last Sync Tracking**: Records when each controller last executed

**Important Columns:**
- `controller_name`: Primary key (e.g., "BrightcoveIngressController")
- `sync_interval_minutes`: How often controller runs (default 15)
- `logging_level`: critical, error, warning, or info
- `last_sync_at`: Timestamp of last successful sync
- `is_enabled`: Whether controller is active

**Use Cases:**
- Admin changes sync interval via API/UI
- Temporarily disable problematic controllers
- Monitor controller health via last_sync_at
- Adjust logging verbosity for troubleshooting

---

## Graph Database Tables

### 10. `graph_relationships` Table
**Purpose:** Stores relationships between assets for graph-based content discovery and relationship traversal.

**Key Functionality:**
- **I-FR-20 (Graph Indexing)**: Creates searchable relationship network
- **I-FR-22 (Graph Search)**: Enables finding related assets via relationships
- **Relationship Types**: Supports multiple relationship types (shared_keyword, shared_topic, temporal, etc.)
- **Bidirectional**: Relationships can be queried from either direction

**Important Columns:**
- `id`: Primary key
- `source_asset_uuid`: Starting asset
- `target_asset_uuid`: Related asset
- `relationship_type`: Type of relationship:
  - `shared_keyword`: Both assets contain same keyword
  - `shared_topic`: Both belong to same topic
  - `temporal`: Created around same time
  - `same_contributor`: Same uploader/creator
  - `related_to`: General relationship
- `relationship_data`: JSONB storing additional context (confidence score, reason, etc.)

**Use Cases:**
- User searches "CEO interview" → finds related assets via graph traversal
- Content discovery: "Show me all videos related to this image"
- Relationship visualization in UI
- Recommendation engine: "Users who viewed this also viewed..."

**Relationships:**
- References `assets` table (source_asset_uuid, ON DELETE CASCADE)
- References `assets` table (target_asset_uuid, ON DELETE CASCADE)

**Note:** For production scale, consider migrating to AWS Neptune or Neo4j for better graph query performance.

---

### 11. `graph_nodes` Table
**Purpose:** Stores graph nodes representing keywords, topics, contributors, and other entities that connect assets.

**Key Functionality:**
- **I-FR-20 (Graph Indexing)**: Indexes keywords and topics extracted from assets
- **Entity Management**: Centralized storage of keywords, topics, contributors
- **Deduplication**: Prevents duplicate nodes via UNIQUE constraint on (node_type, node_name)

**Important Columns:**
- `node_id`: Primary key
- `node_type`: Type of node:
  - `KEYWORD`: Extracted keywords (e.g., "growth", "innovation")
  - `TOPIC`: Content topics (e.g., "Business Strategy", "Technology")
  - `CONTRIBUTOR`: Content creators/uploaders
  - `BRAND`: Detected brands
  - `SPEAKER`: Identified speakers (for audio/video)
- `node_name`: Name/value of the node
- `metadata`: JSONB storing additional properties (frequency, confidence, etc.)

**Use Cases:**
- AI processing extracts keywords → creates KEYWORD nodes
- Assets linked to nodes via `asset_graph_nodes` junction table
- Graph search queries nodes to find related assets
- Analytics: "Which topics are most common?"

---

### 12. `asset_graph_nodes` Table
**Purpose:** Junction table linking assets to graph nodes (many-to-many relationship).

**Key Functionality:**
- **I-FR-20 (Graph Indexing)**: Creates connections between assets and keywords/topics
- **Many-to-Many**: One asset can have multiple keywords, one keyword can belong to multiple assets
- **Relationship Creation**: When AI processes asset, creates links to relevant nodes

**Important Columns:**
- `asset_uuid`: Foreign key to assets
- `node_id`: Foreign key to graph_nodes
- Composite primary key: (asset_uuid, node_id)

**Use Cases:**
- Asset "Interview_with_CEO.mp4" → linked to nodes: "CEO" (KEYWORD), "Business Strategy" (TOPIC), "growth" (KEYWORD)
- Graph query: Find all assets connected to "Business Strategy" topic
- When asset deleted, all relationships automatically removed (ON DELETE CASCADE)

**Relationships:**
- References `assets` table (ON DELETE CASCADE)
- References `graph_nodes` table (ON DELETE CASCADE)

---

## Enums & Types

### Database Enums

#### `asset_type`
- **Values:** `VIDEO`, `IMAGE`, `AUDIO`, `TEXT`
- **Purpose:** Categorizes media assets by type
- **Used in:** `assets` table

#### `source_system`
- **Values:** `BRIGHTCOVE`, `CLOUDINARY`, `OMNYSTUDIO`, `ONECMS`, `MISSYS3`, `DALETS3`, `USER_UPLOAD`, `API_SUBMISSION`
- **Purpose:** Identifies where asset originated
- **Used in:** `assets` table, `lifecycle_rules` table

#### `asset_status`
- **Values:** `STAGED`, `QUEUED`, `PROCESSING`, `PROCESSED`, `FAILED`, `ARCHIVED`
- **Purpose:** Tracks asset lifecycle state
- **Used in:** `assets` table

#### `user_role`
- **Values:** `ADMIN`, `CONTENT_MANAGER`, `EDITOR`, `DEVELOPER`, `VIEWER`
- **Purpose:** Role-based access control
- **Used in:** `users` table

#### `api_key_status`
- **Values:** `ACTIVE`, `REVOKED`
- **Purpose:** API key lifecycle management
- **Used in:** `api_keys` table

#### `action_type`
- **Values:** `INGRESS`, `EGRESS`, `USER_UPLOAD`, `API_SUBMISSION`, `METADATA_UPDATE`, `CONFLICT_RESOLVED`, `JOB_RETRY`, `ROLLBACK`
- **Purpose:** Categorizes audit trail actions
- **Used in:** `action_records` table

#### `direction`
- **Values:** `INBOUND`, `OUTBOUND`, `INTERNAL`
- **Purpose:** Indicates data flow direction
- **Used in:** `action_records` table

#### `action_status`
- **Values:** `SUCCESS`, `FAILED`, `IN_PROGRESS`, `INITIATED`
- **Purpose:** Tracks action execution state
- **Used in:** `action_records` table

#### `job_status`
- **Values:** `QUEUED`, `PROCESSING`, `COMPLETED`, `FAILED`, `RETRYING`, `CANCELLED`
- **Purpose:** Tracks AI workflow execution state
- **Used in:** `processing_jobs` table

---

## Database Relationships Summary

```
users
  ├── assets (uploaded_by)
  ├── api_keys (user_id) [CASCADE]
  ├── asset_versions (created_by)
  ├── action_records (user_id)
  ├── workflow_definitions (created_by)
  └── lifecycle_rules (created_by)

assets (CORE TABLE)
  ├── asset_versions (asset_uuid) [CASCADE]
  ├── processing_jobs (asset_uuid) [CASCADE]
  ├── action_records (asset_uuid)
  ├── graph_relationships (source_asset_uuid, target_asset_uuid) [CASCADE]
  └── asset_graph_nodes (asset_uuid) [CASCADE]

graph_nodes
  └── asset_graph_nodes (node_id) [CASCADE]
```

---

## Key Design Patterns

### 1. **Soft Deletes**
- API keys use `status` field (REVOKED) instead of deletion
- Workflows use `is_active` flag
- Lifecycle rules use `is_active` flag

### 2. **JSONB for Flexibility**
- `enriched_metadata`: Stores variable AI processing results
- `operational_tags`: Business-specific metadata
- `preprocessing_logic`: Configurable routing rules
- `metadata` in action_records: Flexible audit data

### 3. **Version Control**
- `assets.version` + `assets.version_id` for optimistic locking
- `asset_versions` table for historical snapshots
- Enables conflict detection and rollback

### 4. **Cascade Deletes**
- When asset deleted, all related records automatically removed
- Prevents orphaned records
- Maintains referential integrity

### 5. **Indexing Strategy**
- Unique indexes on `file_hash` (deduplication)
- Composite indexes on frequently queried columns
- Timestamp indexes for time-based queries
- Foreign key indexes for join performance

---

## Functional Requirements Coverage

| Table | Functional Requirements |
|-------|------------------------|
| `assets` | I-FR-02, I-FR-15, I-FR-18, I-FR-30 |
| `asset_versions` | I-FR-18, I-FR-19, I-FR-13 |
| `users` | I-FR-21, I-FR-23 |
| `api_keys` | I-FR-23 |
| `action_records` | I-FR-05, I-FR-09 |
| `processing_jobs` | I-FR-26, I-FR-16 |
| `workflow_definitions` | I-FR-32, I-FR-33 |
| `lifecycle_rules` | I-FR-15 |
| `controller_configs` | I-FR-01, I-FR-12 |
| `graph_relationships` | I-FR-20, I-FR-22 |
| `graph_nodes` | I-FR-20, I-FR-22 |
| `asset_graph_nodes` | I-FR-20, I-FR-22 |

---

## Performance Considerations

1. **Partitioning**: Consider partitioning `action_records` by timestamp for large-scale deployments
2. **Full-Text Search**: Add GIN indexes on JSONB columns for metadata search
3. **Graph Database**: For production, migrate graph tables to AWS Neptune or Neo4j
4. **Materialized Views**: Create materialized views for dashboard metrics
5. **Connection Pooling**: Use connection pooling (sqlx Pool) for concurrent access

---

## Next Steps

1. **Add Missing Indexes**: Create indexes on frequently queried columns
2. **Add Constraints**: Add check constraints for data validation
3. **Add Triggers**: Automate lifecycle rule execution
4. **Add Views**: Create views for common queries
5. **Add Functions**: Create stored procedures for complex operations

