use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex, MutexGuard},
};

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{Connection, ErrorCode, OptionalExtension, Transaction, params};
use uuid::Uuid;

use crate::{
    domain::{
        DayFinalization, Level, Profile, ProfileBalance, ProfileLevelState, ProfilePhoto, Sphere,
        Task, TaskCadence, TaskExecution, TaskKind, TaskStatus,
    },
    infrastructure::{Repository, RepositoryError},
};

const MIGRATIONS: [(&str, &str); 1] = [(
    "0001_v2_schema.sql",
    include_str!("../../migrations/0001_v2_schema.sql"),
)];

pub struct SqliteRepository {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteRepository {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, RepositoryError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                RepositoryError::Operation(format!("failed to create database directory: {error}"))
            })?;
        }

        let connection = Connection::open(path).map_err(map_sqlite_error)?;
        connection
            .execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(map_sqlite_error)?;
        apply_migrations(&connection)?;
        seed_default_spheres(&connection)?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    fn connection(&self) -> Result<MutexGuard<'_, Connection>, RepositoryError> {
        self.connection
            .lock()
            .map_err(|_| RepositoryError::Operation("database mutex poisoned".to_owned()))
    }
}

#[async_trait]
impl Repository for SqliteRepository {
    async fn initialize_profile(&self, profile: Profile) -> Result<Vec<Level>, RepositoryError> {
        let now = Utc::now();
        let levels = default_levels(profile.id, now);
        let mut connection = self.connection()?;
        let tx = connection.transaction().map_err(map_sqlite_error)?;

        tx.execute(
            "INSERT INTO profiles
             (id, full_name, birth_date, occupation, telegram, email, timezone, current_photo_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                profile.id,
                profile.full_name,
                profile.birth_date,
                profile.occupation,
                profile.telegram,
                profile.email,
                profile.timezone,
                profile.current_photo_id,
                now.to_rfc3339(),
                now.to_rfc3339()
            ],
        )
        .map_err(map_sqlite_error)?;

        for level in &levels {
            insert_level(&tx, level)?;
        }

        if let Some(first_level) = levels.iter().min_by_key(|level| level.ordinal) {
            tx.execute(
                "INSERT INTO profile_level_state (profile_id, current_level_id, last_balance_id, updated_at)
                 VALUES (?1, ?2, NULL, ?3)",
                params![profile.id, first_level.id, now.to_rfc3339()],
            )
            .map_err(map_sqlite_error)?;
        }

        tx.commit().map_err(map_sqlite_error)?;
        Ok(levels)
    }

    async fn get_profile(&self, profile_id: Uuid) -> Result<Option<Profile>, RepositoryError> {
        self.connection()?
            .query_row(
                "SELECT id, full_name, birth_date, occupation, telegram, email, timezone, current_photo_id
                 FROM profiles WHERE id = ?1",
                params![profile_id],
                map_profile_row,
            )
            .optional()
            .map_err(map_sqlite_error)
    }

    async fn update_profile(&self, profile: Profile) -> Result<(), RepositoryError> {
        self.connection()?
            .execute(
                "UPDATE profiles
                 SET full_name = ?2, birth_date = ?3, occupation = ?4, telegram = ?5, email = ?6,
                     timezone = ?7, current_photo_id = ?8, updated_at = ?9
                 WHERE id = ?1",
                params![
                    profile.id,
                    profile.full_name,
                    profile.birth_date,
                    profile.occupation,
                    profile.telegram,
                    profile.email,
                    profile.timezone,
                    profile.current_photo_id,
                    Utc::now().to_rfc3339()
                ],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn create_photo(&self, photo: ProfilePhoto) -> Result<(), RepositoryError> {
        self.connection()?
            .execute(
                "INSERT INTO profile_photos
                 (id, profile_id, storage_path, original_name, mime_type, size_bytes, width, height, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    photo.id,
                    photo.profile_id,
                    photo.storage_path,
                    photo.original_name,
                    photo.mime_type,
                    photo.size_bytes,
                    photo.width,
                    photo.height,
                    photo.created_at.to_rfc3339(),
                    photo.updated_at.to_rfc3339()
                ],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn list_photos(&self, profile_id: Uuid) -> Result<Vec<ProfilePhoto>, RepositoryError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare(
                "SELECT id, profile_id, storage_path, original_name, mime_type, size_bytes, width, height, created_at, updated_at
                 FROM profile_photos WHERE profile_id = ?1 ORDER BY created_at DESC",
            )
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map(params![profile_id], map_photo_row)
            .map_err(map_sqlite_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(map_sqlite_error)
    }

    async fn get_photo(&self, photo_id: Uuid) -> Result<Option<ProfilePhoto>, RepositoryError> {
        self.connection()?
            .query_row(
                "SELECT id, profile_id, storage_path, original_name, mime_type, size_bytes, width, height, created_at, updated_at
                 FROM profile_photos WHERE id = ?1",
                params![photo_id],
                map_photo_row,
            )
            .optional()
            .map_err(map_sqlite_error)
    }

    async fn delete_photo(&self, photo_id: Uuid) -> Result<(), RepositoryError> {
        self.connection()?
            .execute("DELETE FROM profile_photos WHERE id = ?1", params![photo_id])
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn set_current_photo(
        &self,
        profile_id: Uuid,
        photo_id: Option<Uuid>,
    ) -> Result<(), RepositoryError> {
        self.connection()?
            .execute(
                "UPDATE profiles SET current_photo_id = ?2, updated_at = ?3 WHERE id = ?1",
                params![profile_id, photo_id, Utc::now().to_rfc3339()],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn create_sphere(&self, sphere: Sphere) -> Result<(), RepositoryError> {
        let now = Utc::now().to_rfc3339();
        self.connection()?
            .execute(
                "INSERT INTO spheres (id, name, weight, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![sphere.id, sphere.name, sphere.weight, now, now],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn list_spheres(&self) -> Result<Vec<Sphere>, RepositoryError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare("SELECT id, name, weight FROM spheres ORDER BY name")
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(Sphere {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    weight: row.get(2)?,
                })
            })
            .map_err(map_sqlite_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(map_sqlite_error)
    }

    async fn get_sphere(&self, sphere_id: Uuid) -> Result<Option<Sphere>, RepositoryError> {
        self.connection()?
            .query_row(
                "SELECT id, name, weight FROM spheres WHERE id = ?1",
                params![sphere_id],
                |row| {
                    Ok(Sphere {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        weight: row.get(2)?,
                    })
                },
            )
            .optional()
            .map_err(map_sqlite_error)
    }

    async fn update_sphere(&self, sphere: Sphere) -> Result<(), RepositoryError> {
        self.connection()?
            .execute(
                "UPDATE spheres SET name = ?2, weight = ?3, updated_at = ?4 WHERE id = ?1",
                params![sphere.id, sphere.name, sphere.weight, Utc::now().to_rfc3339()],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn delete_sphere(&self, sphere_id: Uuid) -> Result<(), RepositoryError> {
        self.connection()?
            .execute("DELETE FROM spheres WHERE id = ?1", params![sphere_id])
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn create_task(&self, task: Task) -> Result<(), RepositoryError> {
        self.connection()?
            .execute(
                "INSERT INTO tasks
                 (id, profile_id, sphere_id, title, kind, planned_weight, planned_score, planned_rate, cadence, starts_on, status, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                params![
                    task.id,
                    task.profile_id,
                    task.sphere_id,
                    task.title,
                    task.kind.as_str(),
                    task.planned_weight,
                    task.planned_score,
                    task.planned_rate,
                    task.cadence.as_str(),
                    task.starts_on,
                    task.status.as_str(),
                    task.created_at.to_rfc3339(),
                    task.updated_at.to_rfc3339()
                ],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn list_tasks(&self, profile_id: Uuid) -> Result<Vec<Task>, RepositoryError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare(
                "SELECT id, profile_id, sphere_id, title, kind, planned_weight, planned_score, planned_rate,
                        cadence, starts_on, status, created_at, updated_at
                 FROM tasks
                 WHERE profile_id = ?1
                 ORDER BY planned_weight DESC, created_at DESC",
            )
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map(params![profile_id], map_task_row)
            .map_err(map_sqlite_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(map_sqlite_error)
    }

    async fn get_task(&self, task_id: Uuid) -> Result<Option<Task>, RepositoryError> {
        self.connection()?
            .query_row(
                "SELECT id, profile_id, sphere_id, title, kind, planned_weight, planned_score, planned_rate,
                        cadence, starts_on, status, created_at, updated_at
                 FROM tasks WHERE id = ?1",
                params![task_id],
                map_task_row,
            )
            .optional()
            .map_err(map_sqlite_error)
    }

    async fn update_task(&self, task: Task) -> Result<(), RepositoryError> {
        self.connection()?
            .execute(
                "UPDATE tasks
                 SET sphere_id = ?2, title = ?3, kind = ?4, planned_weight = ?5, planned_score = ?6,
                     planned_rate = ?7, cadence = ?8, starts_on = ?9, status = ?10, updated_at = ?11
                 WHERE id = ?1",
                params![
                    task.id,
                    task.sphere_id,
                    task.title,
                    task.kind.as_str(),
                    task.planned_weight,
                    task.planned_score,
                    task.planned_rate,
                    task.cadence.as_str(),
                    task.starts_on,
                    task.status.as_str(),
                    task.updated_at.to_rfc3339(),
                ],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn delete_task(&self, task_id: Uuid) -> Result<(), RepositoryError> {
        self.connection()?
            .execute("DELETE FROM tasks WHERE id = ?1", params![task_id])
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn create_execution(
        &self,
        task: &Task,
        actual_score: i32,
        actual_rate: i32,
        completed_at: DateTime<Utc>,
        period_start: NaiveDate,
        period_end: NaiveDate,
    ) -> Result<(TaskExecution, ProfileBalance), RepositoryError> {
        let mut connection = self.connection()?;
        let tx = connection.transaction().map_err(map_sqlite_error)?;
        let execution = TaskExecution {
            id: Uuid::new_v4(),
            task_id: task.id,
            profile_id: task.profile_id,
            actual_score,
            actual_rate,
            completed_at,
            period_start,
            period_end,
            created_at: Utc::now(),
        };

        tx.execute(
            "INSERT INTO task_executions
             (id, task_id, profile_id, actual_score, actual_rate, completed_at, period_start, period_end, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                execution.id,
                execution.task_id,
                execution.profile_id,
                execution.actual_score,
                execution.actual_rate,
                execution.completed_at.to_rfc3339(),
                execution.period_start,
                execution.period_end,
                execution.created_at.to_rfc3339()
            ],
        )
        .map_err(map_sqlite_error)?;

        let previous_balance = latest_balance_after(&tx, task.profile_id)?;
        let actual_weight = match task.kind {
            TaskKind::Positive => task.planned_weight,
            TaskKind::Negative => -task.planned_weight,
        };
        let balance = ProfileBalance {
            id: Uuid::new_v4(),
            profile_id: task.profile_id,
            task_execution_id: execution.id,
            actual_rate,
            actual_score,
            actual_weight,
            balance_after: previous_balance + actual_weight,
            created_at: execution.created_at,
        };

        tx.execute(
            "INSERT INTO profile_balances
             (id, profile_id, task_execution_id, actual_rate, actual_score, actual_weight, balance_after, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                balance.id,
                balance.profile_id,
                balance.task_execution_id,
                balance.actual_rate,
                balance.actual_score,
                balance.actual_weight,
                balance.balance_after,
                balance.created_at.to_rfc3339()
            ],
        )
        .map_err(map_sqlite_error)?;

        recalculate_level_state_in_tx(&tx, task.profile_id)?;
        tx.commit().map_err(map_sqlite_error)?;

        Ok((execution, balance))
    }

    async fn list_executions(
        &self,
        profile_id: Uuid,
    ) -> Result<Vec<TaskExecution>, RepositoryError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare(
                "SELECT id, task_id, profile_id, actual_score, actual_rate, completed_at, period_start, period_end, created_at
                 FROM task_executions WHERE profile_id = ?1 ORDER BY completed_at DESC",
            )
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map(params![profile_id], map_execution_row)
            .map_err(map_sqlite_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(map_sqlite_error)
    }

    async fn get_execution(
        &self,
        execution_id: Uuid,
    ) -> Result<Option<TaskExecution>, RepositoryError> {
        self.connection()?
            .query_row(
                "SELECT id, task_id, profile_id, actual_score, actual_rate, completed_at, period_start, period_end, created_at
                 FROM task_executions WHERE id = ?1",
                params![execution_id],
                map_execution_row,
            )
            .optional()
            .map_err(map_sqlite_error)
    }

    async fn delete_execution(
        &self,
        execution_id: Uuid,
    ) -> Result<Option<TaskExecution>, RepositoryError> {
        let mut connection = self.connection()?;
        let tx = connection.transaction().map_err(map_sqlite_error)?;
        let execution = tx
            .query_row(
                "SELECT id, task_id, profile_id, actual_score, actual_rate, completed_at, period_start, period_end, created_at
                 FROM task_executions WHERE id = ?1",
                params![execution_id],
                map_execution_row,
            )
            .optional()
            .map_err(map_sqlite_error)?;

        let Some(execution) = execution else {
            return Ok(None);
        };

        tx.execute(
            "DELETE FROM profile_balances WHERE task_execution_id = ?1",
            params![execution_id],
        )
        .map_err(map_sqlite_error)?;
        tx.execute("DELETE FROM task_executions WHERE id = ?1", params![execution_id])
            .map_err(map_sqlite_error)?;
        rebuild_balances_in_tx(&tx, execution.profile_id)?;
        recalculate_level_state_in_tx(&tx, execution.profile_id)?;
        tx.commit().map_err(map_sqlite_error)?;

        Ok(Some(execution))
    }

    async fn list_balances(
        &self,
        profile_id: Uuid,
    ) -> Result<Vec<ProfileBalance>, RepositoryError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare(
                "SELECT id, profile_id, task_execution_id, actual_rate, actual_score, actual_weight, balance_after, created_at
                 FROM profile_balances WHERE profile_id = ?1 ORDER BY created_at ASC",
            )
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map(params![profile_id], map_balance_row)
            .map_err(map_sqlite_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(map_sqlite_error)
    }

    async fn list_levels(&self, profile_id: Uuid) -> Result<Vec<Level>, RepositoryError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare(
                "SELECT id, profile_id, code, ordinal, min_balance, target_planned_score, target_planned_rate, created_at, updated_at
                 FROM levels WHERE profile_id = ?1 ORDER BY ordinal",
            )
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map(params![profile_id], map_level_row)
            .map_err(map_sqlite_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(map_sqlite_error)
    }

    async fn get_level(&self, level_id: Uuid) -> Result<Option<Level>, RepositoryError> {
        self.connection()?
            .query_row(
                "SELECT id, profile_id, code, ordinal, min_balance, target_planned_score, target_planned_rate, created_at, updated_at
                 FROM levels WHERE id = ?1",
                params![level_id],
                map_level_row,
            )
            .optional()
            .map_err(map_sqlite_error)
    }

    async fn create_level(&self, level: Level) -> Result<(), RepositoryError> {
        self.connection()
            .and_then(|connection| insert_level(&connection, &level))
    }

    async fn update_level(&self, level: Level) -> Result<(), RepositoryError> {
        self.connection()?
            .execute(
                "UPDATE levels
                 SET code = ?2, ordinal = ?3, min_balance = ?4, target_planned_score = ?5,
                     target_planned_rate = ?6, updated_at = ?7
                 WHERE id = ?1",
                params![
                    level.id,
                    level.code,
                    level.ordinal,
                    level.min_balance,
                    level.target_planned_score,
                    level.target_planned_rate,
                    level.updated_at.to_rfc3339()
                ],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn delete_level(&self, level_id: Uuid) -> Result<(), RepositoryError> {
        self.connection()?
            .execute("DELETE FROM levels WHERE id = ?1", params![level_id])
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn get_level_state(
        &self,
        profile_id: Uuid,
    ) -> Result<Option<ProfileLevelState>, RepositoryError> {
        self.connection()?
            .query_row(
                "SELECT profile_id, current_level_id, last_balance_id, updated_at
                 FROM profile_level_state WHERE profile_id = ?1",
                params![profile_id],
                map_level_state_row,
            )
            .optional()
            .map_err(map_sqlite_error)
    }

    async fn recalculate_level_state(
        &self,
        profile_id: Uuid,
    ) -> Result<Option<ProfileLevelState>, RepositoryError> {
        let mut connection = self.connection()?;
        let tx = connection.transaction().map_err(map_sqlite_error)?;
        let state = recalculate_level_state_in_tx(&tx, profile_id)?;
        tx.commit().map_err(map_sqlite_error)?;
        Ok(state)
    }

    async fn create_day_finalization(
        &self,
        finalization: DayFinalization,
    ) -> Result<(), RepositoryError> {
        self.connection()?
            .execute(
                "INSERT INTO day_finalizations (id, profile_id, date, note, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    finalization.id,
                    finalization.profile_id,
                    finalization.date,
                    finalization.note,
                    finalization.created_at.to_rfc3339()
                ],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn list_day_finalizations(
        &self,
        profile_id: Uuid,
    ) -> Result<Vec<DayFinalization>, RepositoryError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare(
                "SELECT id, profile_id, date, note, created_at
                 FROM day_finalizations WHERE profile_id = ?1 ORDER BY date DESC",
            )
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map(params![profile_id], map_day_finalization_row)
            .map_err(map_sqlite_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(map_sqlite_error)
    }
}

fn apply_migrations(connection: &Connection) -> Result<(), RepositoryError> {
    reset_legacy_schema_if_needed(connection)?;
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (name TEXT PRIMARY KEY, applied_at TEXT NOT NULL)",
            [],
        )
        .map_err(map_sqlite_error)?;

    for (name, sql) in MIGRATIONS {
        let already_applied = connection
            .query_row(
                "SELECT name FROM schema_migrations WHERE name = ?1",
                params![name],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(map_sqlite_error)?
            .is_some();

        if already_applied {
            continue;
        }

        connection.execute_batch(sql).map_err(map_sqlite_error)?;
        connection
            .execute(
                "INSERT INTO schema_migrations (name, applied_at) VALUES (?1, ?2)",
                params![name, Utc::now().to_rfc3339()],
            )
            .map_err(map_sqlite_error)?;
    }

    Ok(())
}

fn reset_legacy_schema_if_needed(connection: &Connection) -> Result<(), RepositoryError> {
    let has_legacy_profiles = connection
        .query_row(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'profiles'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(map_sqlite_error)?
        .is_some();

    if !has_legacy_profiles {
        return Ok(());
    }

    let mut has_current_photo_id = false;
    let mut statement = connection
        .prepare("PRAGMA table_info(profiles)")
        .map_err(map_sqlite_error)?;
    let mut rows = statement.query([]).map_err(map_sqlite_error)?;
    while let Some(row) = rows.next().map_err(map_sqlite_error)? {
        let column_name: String = row.get(1).map_err(map_sqlite_error)?;
        if column_name == "current_photo_id" {
            has_current_photo_id = true;
            break;
        }
    }

    if has_current_photo_id {
        return Ok(());
    }

    connection
        .execute_batch(
            "DROP TABLE IF EXISTS profile_level_state;
             DROP TABLE IF EXISTS day_finalizations;
             DROP TABLE IF EXISTS profile_balances;
             DROP TABLE IF EXISTS task_executions;
             DROP TABLE IF EXISTS levels;
             DROP TABLE IF EXISTS profile_photos;
             DROP TABLE IF EXISTS daily_snapshots;
             DROP TABLE IF EXISTS tasks;
             DROP TABLE IF EXISTS spheres;
             DROP TABLE IF EXISTS profiles;
             DROP TABLE IF EXISTS schema_migrations;",
        )
        .map_err(map_sqlite_error)?;

    Ok(())
}

fn default_levels(profile_id: Uuid, now: DateTime<Utc>) -> Vec<Level> {
    let definitions = [
        ("x1", 1, 0, 2, 20),
        ("x2", 2, 5, 3, 40),
        ("x3", 3, 10, 3, 50),
        ("x4", 4, 15, 4, 60),
        ("x5", 5, 20, 4, 70),
        ("x6", 6, 25, 4, 80),
        ("x7", 7, 30, 5, 85),
        ("x8", 8, 35, 5, 90),
        ("x9", 9, 40, 5, 95),
        ("x10", 10, 45, 5, 99),
    ];

    definitions
        .into_iter()
        .map(|(code, ordinal, min_balance, target_planned_score, target_planned_rate)| Level {
            id: Uuid::new_v4(),
            profile_id,
            code: code.to_owned(),
            ordinal,
            min_balance,
            target_planned_score,
            target_planned_rate,
            created_at: now,
            updated_at: now,
        })
        .collect()
}

fn insert_level<C>(connection: &C, level: &Level) -> Result<(), RepositoryError>
where
    C: std::ops::Deref<Target = Connection>,
{
    connection
        .execute(
            "INSERT INTO levels
             (id, profile_id, code, ordinal, min_balance, target_planned_score, target_planned_rate, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                level.id,
                level.profile_id,
                level.code,
                level.ordinal,
                level.min_balance,
                level.target_planned_score,
                level.target_planned_rate,
                level.created_at.to_rfc3339(),
                level.updated_at.to_rfc3339()
            ],
        )
        .map_err(map_sqlite_error)?;
    Ok(())
}

fn recalculate_level_state_in_tx(
    tx: &Transaction<'_>,
    profile_id: Uuid,
) -> Result<Option<ProfileLevelState>, RepositoryError> {
    let levels = {
        let mut statement = tx
            .prepare(
                "SELECT id, profile_id, code, ordinal, min_balance, target_planned_score, target_planned_rate, created_at, updated_at
                 FROM levels WHERE profile_id = ?1 ORDER BY ordinal",
            )
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map(params![profile_id], map_level_row)
            .map_err(map_sqlite_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(map_sqlite_error)?
    };

    let Some(current_level) = levels
        .iter()
        .filter(|level| latest_balance_after(tx, profile_id).unwrap_or_default() >= level.min_balance)
        .max_by_key(|level| level.ordinal)
        .or_else(|| levels.iter().min_by_key(|level| level.ordinal))
    else {
        return Ok(None);
    };

    let last_balance_id = tx
        .query_row(
            "SELECT id FROM profile_balances WHERE profile_id = ?1 ORDER BY created_at DESC LIMIT 1",
            params![profile_id],
            |row| row.get::<_, Uuid>(0),
        )
        .optional()
        .map_err(map_sqlite_error)?;
    let updated_at = Utc::now();

    tx.execute(
        "INSERT INTO profile_level_state (profile_id, current_level_id, last_balance_id, updated_at)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(profile_id)
         DO UPDATE SET current_level_id = excluded.current_level_id,
                       last_balance_id = excluded.last_balance_id,
                       updated_at = excluded.updated_at",
        params![
            profile_id,
            current_level.id,
            last_balance_id,
            updated_at.to_rfc3339()
        ],
    )
    .map_err(map_sqlite_error)?;

    Ok(Some(ProfileLevelState {
        profile_id,
        current_level_id: current_level.id,
        last_balance_id,
        updated_at,
    }))
}

fn latest_balance_after(tx: &Transaction<'_>, profile_id: Uuid) -> Result<i32, RepositoryError> {
    tx.query_row(
        "SELECT balance_after FROM profile_balances WHERE profile_id = ?1 ORDER BY created_at DESC LIMIT 1",
        params![profile_id],
        |row| row.get::<_, i32>(0),
    )
    .optional()
    .map(|value| value.unwrap_or_default())
    .map_err(map_sqlite_error)
}

fn rebuild_balances_in_tx(tx: &Transaction<'_>, profile_id: Uuid) -> Result<(), RepositoryError> {
    let entries = {
        let mut statement = tx
            .prepare(
                "SELECT id, actual_weight FROM profile_balances WHERE profile_id = ?1 ORDER BY created_at ASC, id ASC",
            )
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map(params![profile_id], |row| {
                Ok((row.get::<_, Uuid>(0)?, row.get::<_, i32>(1)?))
            })
            .map_err(map_sqlite_error)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(map_sqlite_error)?
    };

    let mut running_balance = 0;
    for (balance_id, actual_weight) in entries {
        running_balance += actual_weight;
        tx.execute(
            "UPDATE profile_balances SET balance_after = ?2 WHERE id = ?1",
            params![balance_id, running_balance],
        )
        .map_err(map_sqlite_error)?;
    }
    Ok(())
}

fn map_profile_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Profile> {
    Ok(Profile {
        id: row.get(0)?,
        full_name: row.get(1)?,
        birth_date: row.get(2)?,
        occupation: row.get(3)?,
        telegram: row.get(4)?,
        email: row.get(5)?,
        timezone: row.get(6)?,
        current_photo_id: row.get(7)?,
    })
}

fn map_photo_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProfilePhoto> {
    Ok(ProfilePhoto {
        id: row.get(0)?,
        profile_id: row.get(1)?,
        storage_path: row.get(2)?,
        original_name: row.get(3)?,
        mime_type: row.get(4)?,
        size_bytes: row.get(5)?,
        width: row.get(6)?,
        height: row.get(7)?,
        created_at: parse_datetime(&row.get::<_, String>(8)?).map_err(to_from_sql_error)?,
        updated_at: parse_datetime(&row.get::<_, String>(9)?).map_err(to_from_sql_error)?,
    })
}

fn map_task_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Task> {
    Ok(Task {
        id: row.get(0)?,
        profile_id: row.get(1)?,
        sphere_id: row.get(2)?,
        title: row.get(3)?,
        kind: TaskKind::from_db(&row.get::<_, String>(4)?),
        planned_weight: row.get(5)?,
        planned_score: row.get(6)?,
        planned_rate: row.get(7)?,
        cadence: TaskCadence::from_db(&row.get::<_, String>(8)?),
        starts_on: row.get(9)?,
        status: TaskStatus::from_db(&row.get::<_, String>(10)?),
        created_at: parse_datetime(&row.get::<_, String>(11)?).map_err(to_from_sql_error)?,
        updated_at: parse_datetime(&row.get::<_, String>(12)?).map_err(to_from_sql_error)?,
    })
}

fn map_execution_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<TaskExecution> {
    Ok(TaskExecution {
        id: row.get(0)?,
        task_id: row.get(1)?,
        profile_id: row.get(2)?,
        actual_score: row.get(3)?,
        actual_rate: row.get(4)?,
        completed_at: parse_datetime(&row.get::<_, String>(5)?).map_err(to_from_sql_error)?,
        period_start: row.get(6)?,
        period_end: row.get(7)?,
        created_at: parse_datetime(&row.get::<_, String>(8)?).map_err(to_from_sql_error)?,
    })
}

fn map_balance_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProfileBalance> {
    Ok(ProfileBalance {
        id: row.get(0)?,
        profile_id: row.get(1)?,
        task_execution_id: row.get(2)?,
        actual_rate: row.get(3)?,
        actual_score: row.get(4)?,
        actual_weight: row.get(5)?,
        balance_after: row.get(6)?,
        created_at: parse_datetime(&row.get::<_, String>(7)?).map_err(to_from_sql_error)?,
    })
}

fn map_level_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Level> {
    Ok(Level {
        id: row.get(0)?,
        profile_id: row.get(1)?,
        code: row.get(2)?,
        ordinal: row.get(3)?,
        min_balance: row.get(4)?,
        target_planned_score: row.get(5)?,
        target_planned_rate: row.get(6)?,
        created_at: parse_datetime(&row.get::<_, String>(7)?).map_err(to_from_sql_error)?,
        updated_at: parse_datetime(&row.get::<_, String>(8)?).map_err(to_from_sql_error)?,
    })
}

fn map_level_state_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ProfileLevelState> {
    Ok(ProfileLevelState {
        profile_id: row.get(0)?,
        current_level_id: row.get(1)?,
        last_balance_id: row.get(2)?,
        updated_at: parse_datetime(&row.get::<_, String>(3)?).map_err(to_from_sql_error)?,
    })
}

fn map_day_finalization_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DayFinalization> {
    Ok(DayFinalization {
        id: row.get(0)?,
        profile_id: row.get(1)?,
        date: row.get(2)?,
        note: row.get(3)?,
        created_at: parse_datetime(&row.get::<_, String>(4)?).map_err(to_from_sql_error)?,
    })
}

fn parse_datetime(value: &str) -> Result<DateTime<Utc>, String> {
    DateTime::parse_from_rfc3339(value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|error| error.to_string())
}

fn to_from_sql_error(error: String) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(
        0,
        rusqlite::types::Type::Text,
        Box::<dyn std::error::Error + Send + Sync>::from(error),
    )
}

fn map_sqlite_error(error: rusqlite::Error) -> RepositoryError {
    match error {
        rusqlite::Error::SqliteFailure(code, message)
            if code.code == ErrorCode::ConstraintViolation =>
        {
            RepositoryError::Conflict(message.unwrap_or_else(|| "constraint violated".to_owned()))
        }
        other => RepositoryError::Operation(other.to_string()),
    }
}

fn seed_default_spheres(connection: &Connection) -> Result<(), RepositoryError> {
    let defaults = ["health", "work", "family", "mind", "finance", "rest"];
    for name in defaults {
        connection
            .execute(
                "INSERT OR IGNORE INTO spheres (id, name, weight, created_at, updated_at) VALUES (?1, ?2, 1, ?3, ?4)",
                params![Uuid::new_v4(), name, Utc::now().to_rfc3339(), Utc::now().to_rfc3339()],
            )
            .map_err(map_sqlite_error)?;
    }
    Ok(())
}

impl TaskKind {
    fn as_str(self) -> &'static str {
        match self {
            TaskKind::Positive => "positive",
            TaskKind::Negative => "negative",
        }
    }

    fn from_db(value: &str) -> Self {
        match value {
            "negative" => Self::Negative,
            _ => Self::Positive,
        }
    }
}

impl TaskCadence {
    fn as_str(self) -> &'static str {
        match self {
            TaskCadence::Day => "day",
            TaskCadence::Week => "week",
            TaskCadence::Month => "month",
            TaskCadence::Year => "year",
        }
    }

    fn from_db(value: &str) -> Self {
        match value {
            "week" => Self::Week,
            "month" => Self::Month,
            "year" => Self::Year,
            _ => Self::Day,
        }
    }
}

impl TaskStatus {
    fn as_str(self) -> &'static str {
        match self {
            TaskStatus::Planned => "planned",
            TaskStatus::Archived => "archived",
            TaskStatus::Skipped => "skipped",
        }
    }

    fn from_db(value: &str) -> Self {
        match value {
            "archived" => Self::Archived,
            "skipped" => Self::Skipped,
            _ => Self::Planned,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Utc};

    use super::SqliteRepository;
    use crate::{
        domain::{Profile, Task, TaskCadence, TaskKind, TaskStatus},
        infrastructure::Repository,
    };

    #[tokio::test]
    async fn sqlite_repository_initializes_profile_levels_and_execution_balances() {
        let database = tempfile::NamedTempFile::new().unwrap();
        let repository = SqliteRepository::new(database.path()).unwrap();

        let profile = Profile {
            id: uuid::Uuid::new_v4(),
            full_name: "Test User".into(),
            birth_date: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            occupation: "Builder".into(),
            telegram: None,
            email: None,
            timezone: "Europe/Samara".into(),
            current_photo_id: None,
        };

        let levels = repository.initialize_profile(profile.clone()).await.unwrap();
        assert_eq!(levels.len(), 10);

        let task = Task {
            id: uuid::Uuid::new_v4(),
            profile_id: profile.id,
            title: "Run".into(),
            sphere_id: None,
            kind: TaskKind::Positive,
            planned_weight: 3,
            planned_score: 4,
            planned_rate: 80,
            cadence: TaskCadence::Day,
            starts_on: NaiveDate::from_ymd_opt(2026, 7, 2).unwrap(),
            status: TaskStatus::Planned,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repository.create_task(task.clone()).await.unwrap();

        let (_execution, balance) = repository
            .create_execution(
                &task,
                5,
                90,
                Utc::now(),
                task.starts_on,
                task.starts_on,
            )
            .await
            .unwrap();

        assert_eq!(balance.actual_weight, 3);
        assert_eq!(balance.balance_after, 3);
    }
}
