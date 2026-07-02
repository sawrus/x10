PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS profiles (
    id TEXT PRIMARY KEY,
    full_name TEXT NOT NULL,
    birth_date TEXT NOT NULL,
    occupation TEXT NOT NULL,
    telegram TEXT,
    email TEXT,
    timezone TEXT NOT NULL,
    current_photo_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY(current_photo_id) REFERENCES profile_photos(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS profile_photos (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    storage_path TEXT NOT NULL,
    original_name TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    size_bytes INTEGER NOT NULL CHECK(size_bytes >= 0),
    width INTEGER,
    height INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_profile_photos_profile_created
    ON profile_photos(profile_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_profiles_current_photo
    ON profiles(current_photo_id);

CREATE TABLE IF NOT EXISTS spheres (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    weight INTEGER NOT NULL CHECK(weight > 0),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_spheres_name
    ON spheres(name);

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    sphere_id TEXT,
    title TEXT NOT NULL,
    kind TEXT NOT NULL CHECK(kind IN ('positive', 'negative')),
    planned_weight INTEGER NOT NULL CHECK(planned_weight > 0),
    planned_score INTEGER NOT NULL CHECK(planned_score BETWEEN 1 AND 5),
    planned_rate INTEGER NOT NULL CHECK(planned_rate BETWEEN 0 AND 100),
    cadence TEXT NOT NULL CHECK(cadence IN ('day', 'week', 'month', 'year')),
    starts_on TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('planned', 'archived', 'skipped')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
    FOREIGN KEY(sphere_id) REFERENCES spheres(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_tasks_profile_status_weight
    ON tasks(profile_id, status, planned_weight DESC, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_tasks_profile_starts_cadence
    ON tasks(profile_id, starts_on, cadence);

CREATE TABLE IF NOT EXISTS task_executions (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    actual_score INTEGER NOT NULL CHECK(actual_score BETWEEN 1 AND 5),
    actual_rate INTEGER NOT NULL CHECK(actual_rate BETWEEN 0 AND 100),
    completed_at TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY(task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_task_executions_profile_completed
    ON task_executions(profile_id, completed_at DESC);

CREATE INDEX IF NOT EXISTS idx_task_executions_task_completed
    ON task_executions(task_id, completed_at DESC);

CREATE INDEX IF NOT EXISTS idx_task_executions_profile_period
    ON task_executions(profile_id, period_start, period_end);

CREATE TABLE IF NOT EXISTS profile_balances (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    task_execution_id TEXT NOT NULL UNIQUE,
    actual_rate INTEGER NOT NULL CHECK(actual_rate BETWEEN 0 AND 100),
    actual_score INTEGER NOT NULL CHECK(actual_score BETWEEN 1 AND 5),
    actual_weight INTEGER NOT NULL,
    balance_after INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
    FOREIGN KEY(task_execution_id) REFERENCES task_executions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_profile_balances_profile_created
    ON profile_balances(profile_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_profile_balances_execution
    ON profile_balances(task_execution_id);

CREATE TABLE IF NOT EXISTS levels (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    code TEXT NOT NULL,
    ordinal INTEGER NOT NULL,
    min_balance INTEGER NOT NULL,
    target_planned_score INTEGER NOT NULL CHECK(target_planned_score BETWEEN 1 AND 5),
    target_planned_rate INTEGER NOT NULL CHECK(target_planned_rate BETWEEN 0 AND 100),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
    UNIQUE(profile_id, code),
    UNIQUE(profile_id, ordinal)
);

CREATE INDEX IF NOT EXISTS idx_levels_profile_ordinal
    ON levels(profile_id, ordinal);

CREATE INDEX IF NOT EXISTS idx_levels_profile_min_balance
    ON levels(profile_id, min_balance);

CREATE TABLE IF NOT EXISTS profile_level_state (
    profile_id TEXT PRIMARY KEY,
    current_level_id TEXT NOT NULL,
    last_balance_id TEXT,
    updated_at TEXT NOT NULL,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
    FOREIGN KEY(current_level_id) REFERENCES levels(id) ON DELETE RESTRICT,
    FOREIGN KEY(last_balance_id) REFERENCES profile_balances(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS day_finalizations (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    date TEXT NOT NULL,
    note TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
    UNIQUE(profile_id, date)
);

CREATE INDEX IF NOT EXISTS idx_day_finalizations_profile_date
    ON day_finalizations(profile_id, date);
