PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS profiles (
    id TEXT PRIMARY KEY,
    full_name TEXT NOT NULL,
    birth_date TEXT NOT NULL,
    occupation TEXT NOT NULL,
    telegram TEXT,
    email TEXT,
    timezone TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS spheres (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    sphere_id TEXT,
    title TEXT NOT NULL,
    kind TEXT NOT NULL CHECK(kind IN ('positive', 'negative')),
    weight INTEGER NOT NULL CHECK(weight > 0),
    cadence TEXT NOT NULL CHECK(cadence IN ('day', 'week', 'month', 'year')),
    scheduled_for TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('planned', 'completed', 'skipped')),
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
    FOREIGN KEY(sphere_id) REFERENCES spheres(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_tasks_profile_scheduled_for
    ON tasks(profile_id, scheduled_for);

CREATE TABLE IF NOT EXISTS daily_snapshots (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    snapshot_date TEXT NOT NULL,
    positive_weight INTEGER NOT NULL,
    negative_weight INTEGER NOT NULL,
    net_score INTEGER NOT NULL,
    balance_score INTEGER NOT NULL,
    level TEXT NOT NULL,
    rest_day_credits INTEGER NOT NULL,
    finalized_at TEXT NOT NULL,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
    UNIQUE(profile_id, snapshot_date)
);

CREATE INDEX IF NOT EXISTS idx_daily_snapshots_profile_date
    ON daily_snapshots(profile_id, snapshot_date);
