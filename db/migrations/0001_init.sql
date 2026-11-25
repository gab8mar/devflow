CREATE TABLE schema_migrations (
    version     INTEGER PRIMARY KEY,
    applied_at  INTEGER NOT NULL,
    checksum    TEXT NOT NULL
);

CREATE TABLE projects (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL CHECK (length(trim(name)) > 0),
    code        TEXT NOT NULL CHECK (length(trim(code)) > 0),
    external_id TEXT,
    color       TEXT NOT NULL DEFAULT '#2F81F7',
    is_archived INTEGER NOT NULL DEFAULT 0 CHECK (is_archived IN (0, 1)),
    sort_order  INTEGER NOT NULL DEFAULT 0,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL,
    deleted_at  INTEGER,
    CHECK (deleted_at IS NULL OR deleted_at >= created_at)
);

CREATE UNIQUE INDEX ux_projects_code ON projects (code);
CREATE UNIQUE INDEX ux_projects_external ON projects (external_id) WHERE external_id IS NOT NULL;
CREATE INDEX ix_projects_active ON projects (is_archived, sort_order);

CREATE TABLE tasks (
    id               TEXT PRIMARY KEY,
    project_id       TEXT NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    title            TEXT NOT NULL CHECK (length(trim(title)) > 0),
    external_id      TEXT,
    task_type        TEXT NOT NULL DEFAULT 'task'
                     CHECK (task_type IN ('task', 'bug', 'feature', 'support', 'meeting', 'other')),
    status           TEXT NOT NULL DEFAULT 'active'
                     CHECK (status IN ('active', 'closed', 'blocked')),
    estimate_minutes INTEGER CHECK (estimate_minutes IS NULL OR estimate_minutes >= 0),
    sort_order       INTEGER NOT NULL DEFAULT 0,
    created_at       INTEGER NOT NULL,
    updated_at       INTEGER NOT NULL,
    deleted_at       INTEGER,
    CHECK (deleted_at IS NULL OR deleted_at >= created_at)
);

CREATE UNIQUE INDEX ux_tasks_external ON tasks (project_id, external_id) WHERE external_id IS NOT NULL;
CREATE INDEX ix_tasks_project ON tasks (project_id, status, sort_order);
CREATE INDEX ix_tasks_updated ON tasks (updated_at DESC);

CREATE TABLE time_entries (
    id                  TEXT PRIMARY KEY,
    task_id             TEXT NOT NULL REFERENCES tasks (id) ON DELETE RESTRICT,
    status              TEXT NOT NULL CHECK (status IN ('running', 'paused', 'closed')),
    started_at          INTEGER NOT NULL,
    last_resumed_at     INTEGER,
    ended_at            INTEGER,
    accumulated_seconds INTEGER NOT NULL DEFAULT 0 CHECK (accumulated_seconds >= 0),
    duration_seconds    INTEGER CHECK (duration_seconds IS NULL OR duration_seconds >= 0),
    last_heartbeat_at   INTEGER,
    source              TEXT NOT NULL DEFAULT 'manual'
                        CHECK (source IN ('manual', 'autoswitch', 'idle_pause', 'import', 'recovered')),
    note                TEXT CHECK (note IS NULL OR length(note) <= 500),
    billable            INTEGER NOT NULL DEFAULT 1 CHECK (billable IN (0, 1)),
    sync_state          TEXT NOT NULL DEFAULT 'local'
                        CHECK (sync_state IN ('local', 'pending', 'synced', 'failed', 'conflict')),
    external_id         TEXT,
    idempotency_key     TEXT NOT NULL UNIQUE,
    created_at          INTEGER NOT NULL,
    updated_at          INTEGER NOT NULL,
    CHECK (ended_at IS NULL OR ended_at >= started_at),
    CHECK (status <> 'running' OR last_resumed_at IS NOT NULL),
    CHECK (status <> 'paused' OR last_resumed_at IS NULL),
    CHECK (status <> 'closed' OR last_resumed_at IS NULL),
    CHECK (status <> 'closed' OR (ended_at IS NOT NULL AND duration_seconds IS NOT NULL)),
    CHECK (status <> 'closed' OR duration_seconds = accumulated_seconds)
);

CREATE UNIQUE INDEX ux_time_entries_running ON time_entries (status) WHERE status = 'running';
CREATE INDEX ix_time_entries_task_started ON time_entries (task_id, started_at DESC);
CREATE INDEX ix_time_entries_started ON time_entries (started_at DESC);
CREATE INDEX ix_time_entries_sync ON time_entries (sync_state)
    WHERE sync_state IN ('pending', 'failed', 'conflict');

CREATE TABLE user_preferences (
    key        TEXT PRIMARY KEY,
    value_json TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE activity_history (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    occurred_at   INTEGER NOT NULL,
    event_type    TEXT NOT NULL CHECK (event_type IN (
                      'app_start', 'app_stop', 'task_switch', 'idle_start', 'idle_end',
                      'focus_change', 'context_suggested', 'context_dismissed', 'pause',
                      'resume', 'entry_closed', 'sync_attempt', 'sync_result', 'recovered_entry')),
    time_entry_id TEXT REFERENCES time_entries (id) ON DELETE SET NULL,
    payload_json  TEXT,
    created_at    INTEGER NOT NULL
);

CREATE INDEX ix_activity_occurred ON activity_history (occurred_at DESC);
CREATE INDEX ix_activity_type ON activity_history (event_type, occurred_at DESC);
CREATE INDEX ix_activity_entry ON activity_history (time_entry_id) WHERE time_entry_id IS NOT NULL;

CREATE TABLE sync_outbox (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    entity_type     TEXT NOT NULL CHECK (entity_type IN ('time_entry', 'task', 'project')),
    entity_id       TEXT NOT NULL,
    operation       TEXT NOT NULL CHECK (operation IN ('create', 'update', 'delete', 'submit')),
    payload_json    TEXT,
    idempotency_key TEXT NOT NULL UNIQUE,
    attempts        INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    next_attempt_at INTEGER NOT NULL,
    last_error      TEXT,
    state           TEXT NOT NULL DEFAULT 'queued'
                    CHECK (state IN ('queued', 'in_flight', 'done', 'dead')),
    created_at      INTEGER NOT NULL,
    updated_at      INTEGER NOT NULL
);

CREATE INDEX ix_outbox_ready ON sync_outbox (state, next_attempt_at);
CREATE INDEX ix_outbox_entity ON sync_outbox (entity_type, entity_id);

CREATE TABLE integration_accounts (
    id             TEXT PRIMARY KEY,
    provider       TEXT NOT NULL CHECK (provider IN ('entra', 'ado', 'pwa_timesheet', 'custom_rest')),
    tenant_id      TEXT,
    account_upn    TEXT,
    credential_ref TEXT NOT NULL,
    scopes         TEXT,
    expires_at     INTEGER,
    created_at     INTEGER NOT NULL,
    updated_at     INTEGER NOT NULL,
    UNIQUE (provider, account_upn)
);

CREATE TABLE external_mappings (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    local_entity TEXT NOT NULL CHECK (local_entity IN ('project', 'task', 'time_entry')),
    local_id     TEXT NOT NULL,
    provider     TEXT NOT NULL,
    external_id  TEXT NOT NULL,
    payload_hash TEXT,
    updated_at   INTEGER NOT NULL,
    UNIQUE (local_entity, local_id, provider),
    UNIQUE (provider, external_id)
);

CREATE VIEW v_running_entry AS
SELECT
    te.id                  AS entry_id,
    te.task_id             AS task_id,
    t.project_id           AS project_id,
    t.title                AS task_title,
    p.name                 AS project_name,
    p.code                 AS project_code,
    p.color                AS project_color,
    te.started_at          AS started_at,
    te.last_resumed_at     AS last_resumed_at,
    te.accumulated_seconds AS accumulated_seconds
FROM time_entries te
JOIN tasks t ON t.id = te.task_id
JOIN projects p ON p.id = t.project_id
WHERE te.status = 'running';

CREATE VIEW v_daily_totals AS
SELECT
    date(te.started_at / 1000, 'unixepoch', 'localtime') AS local_day,
    p.id   AS project_id,
    p.name AS project_name,
    p.code AS project_code,
    t.id   AS task_id,
    t.title AS task_title,
    SUM(COALESCE(te.duration_seconds, 0)) AS total_seconds,
    COUNT(*) AS entry_count
FROM time_entries te
JOIN tasks t ON t.id = te.task_id
JOIN projects p ON p.id = t.project_id
WHERE te.status = 'closed'
GROUP BY local_day, p.id, t.id;

PRAGMA user_version = 1;
