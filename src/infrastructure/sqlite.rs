use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex, MutexGuard},
};

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{Connection, OptionalExtension, params};
use uuid::Uuid;

use crate::{
    domain::{DailySnapshot, DaySummary, Profile, Sphere, Task, TaskCadence, TaskKind, TaskStatus},
    infrastructure::{Repository, RepositoryError},
};

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
            .execute_batch(include_str!("../../migrations/0001_init.sql"))
            .map_err(map_sqlite_error)?;
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
    async fn create_profile(&self, profile: Profile) -> Result<(), RepositoryError> {
        let now = Utc::now().to_rfc3339();
        self.connection()?
            .execute(
                "INSERT INTO profiles (id, full_name, birth_date, occupation, telegram, email, timezone, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    profile.id,
                    profile.full_name,
                    profile.birth_date,
                    profile.occupation,
                    profile.telegram,
                    profile.email,
                    profile.timezone,
                    now,
                    now
                ],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn get_profile(&self, profile_id: Uuid) -> Result<Option<Profile>, RepositoryError> {
        self.connection()?
            .query_row(
                "SELECT id, full_name, birth_date, occupation, telegram, email, timezone
                 FROM profiles WHERE id = ?1",
                params![profile_id],
                |row| {
                    Ok(Profile {
                        id: row.get(0)?,
                        full_name: row.get(1)?,
                        birth_date: row.get(2)?,
                        occupation: row.get(3)?,
                        telegram: row.get(4)?,
                        email: row.get(5)?,
                        timezone: row.get(6)?,
                    })
                },
            )
            .optional()
            .map_err(map_sqlite_error)
    }

    async fn create_sphere(&self, sphere: Sphere) -> Result<(), RepositoryError> {
        self.connection()?
            .execute(
                "INSERT INTO spheres (id, name, created_at) VALUES (?1, ?2, ?3)",
                params![sphere.id, sphere.name, Utc::now().to_rfc3339()],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn list_spheres(&self) -> Result<Vec<Sphere>, RepositoryError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare("SELECT id, name FROM spheres ORDER BY name")
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(Sphere {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })
            .map_err(map_sqlite_error)?;

        rows.collect::<Result<Vec<_>, _>>()
            .map_err(map_sqlite_error)
    }

    async fn create_task(&self, task: Task) -> Result<(), RepositoryError> {
        let now = Utc::now().to_rfc3339();
        self.connection()?
            .execute(
                "INSERT INTO tasks
                 (id, profile_id, sphere_id, title, kind, weight, cadence, scheduled_for, status, completed_at, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                params![
                    task.id,
                    task.profile_id,
                    task.sphere_id,
                    task.title,
                    task.kind.as_str(),
                    task.weight,
                    task.cadence.as_str(),
                    task.scheduled_for,
                    task.status.as_str(),
                    task.completed_at.map(|value| value.to_rfc3339()),
                    now,
                    now
                ],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn get_task(&self, task_id: Uuid) -> Result<Option<Task>, RepositoryError> {
        self.connection()?
            .query_row(
                "SELECT id, profile_id, sphere_id, title, kind, weight, cadence, scheduled_for, status, completed_at
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
                 SET sphere_id = ?2, title = ?3, kind = ?4, weight = ?5, cadence = ?6,
                     scheduled_for = ?7, status = ?8, completed_at = ?9, updated_at = ?10
                 WHERE id = ?1",
                params![
                    task.id,
                    task.sphere_id,
                    task.title,
                    task.kind.as_str(),
                    task.weight,
                    task.cadence.as_str(),
                    task.scheduled_for,
                    task.status.as_str(),
                    task.completed_at.map(|value| value.to_rfc3339()),
                    Utc::now().to_rfc3339()
                ],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn list_tasks(&self, profile_id: Uuid) -> Result<Vec<Task>, RepositoryError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare(
                "SELECT id, profile_id, sphere_id, title, kind, weight, cadence, scheduled_for, status, completed_at
                 FROM tasks WHERE profile_id = ?1 ORDER BY scheduled_for, created_at",
            )
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map(params![profile_id], map_task_row)
            .map_err(map_sqlite_error)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(map_sqlite_error)
    }

    async fn list_tasks_for_date(
        &self,
        profile_id: Uuid,
        date: NaiveDate,
    ) -> Result<Vec<Task>, RepositoryError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare(
                "SELECT id, profile_id, sphere_id, title, kind, weight, cadence, scheduled_for, status, completed_at
                 FROM tasks WHERE profile_id = ?1 AND scheduled_for = ?2 ORDER BY created_at",
            )
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map(params![profile_id, date], map_task_row)
            .map_err(map_sqlite_error)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(map_sqlite_error)
    }

    async fn create_snapshot(
        &self,
        snapshot: DailySnapshot,
        profile_id: Uuid,
    ) -> Result<(), RepositoryError> {
        self.connection()?
            .execute(
                "INSERT INTO daily_snapshots
                 (id, profile_id, snapshot_date, positive_weight, negative_weight, net_score, balance_score, level, rest_day_credits, finalized_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    snapshot.id,
                    profile_id,
                    snapshot.date,
                    snapshot.summary.positive_weight,
                    snapshot.summary.negative_weight,
                    snapshot.summary.net_score,
                    snapshot.balance_score,
                    snapshot.level,
                    snapshot.rest_day_credits,
                    snapshot.finalized_at.to_rfc3339()
                ],
            )
            .map_err(map_sqlite_error)?;
        Ok(())
    }

    async fn get_snapshot(
        &self,
        profile_id: Uuid,
        date: NaiveDate,
    ) -> Result<Option<DailySnapshot>, RepositoryError> {
        self.connection()?
            .query_row(
                "SELECT id, snapshot_date, positive_weight, negative_weight, net_score, balance_score, level, rest_day_credits, finalized_at
                 FROM daily_snapshots WHERE profile_id = ?1 AND snapshot_date = ?2",
                params![profile_id, date],
                map_snapshot_row,
            )
            .optional()
            .map_err(map_sqlite_error)
    }

    async fn list_snapshots(
        &self,
        profile_id: Uuid,
    ) -> Result<Vec<DailySnapshot>, RepositoryError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare(
                "SELECT id, snapshot_date, positive_weight, negative_weight, net_score, balance_score, level, rest_day_credits, finalized_at
                 FROM daily_snapshots WHERE profile_id = ?1 ORDER BY snapshot_date",
            )
            .map_err(map_sqlite_error)?;
        let rows = statement
            .query_map(params![profile_id], map_snapshot_row)
            .map_err(map_sqlite_error)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(map_sqlite_error)
    }
}

fn map_task_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Task> {
    Ok(Task {
        id: row.get(0)?,
        profile_id: row.get(1)?,
        sphere_id: row.get(2)?,
        title: row.get(3)?,
        kind: TaskKind::from_db(&row.get::<_, String>(4)?),
        weight: row.get(5)?,
        cadence: TaskCadence::from_db(&row.get::<_, String>(6)?),
        scheduled_for: row.get(7)?,
        status: TaskStatus::from_db(&row.get::<_, String>(8)?),
        completed_at: row
            .get::<_, Option<String>>(9)?
            .map(|value| parse_datetime(&value))
            .transpose()
            .map_err(to_from_sql_error)?,
    })
}

fn map_snapshot_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DailySnapshot> {
    let date: NaiveDate = row.get(1)?;
    Ok(DailySnapshot {
        id: row.get(0)?,
        date,
        summary: DaySummary {
            date,
            positive_weight: row.get(2)?,
            negative_weight: row.get(3)?,
            net_score: row.get(4)?,
        },
        balance_score: row.get(5)?,
        level: row.get(6)?,
        rest_day_credits: row.get(7)?,
        finalized_at: parse_datetime(&row.get::<_, String>(8)?).map_err(to_from_sql_error)?,
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
    RepositoryError::Operation(error.to_string())
}

fn seed_default_spheres(connection: &Connection) -> Result<(), RepositoryError> {
    let defaults = ["health", "work", "family", "mind", "finance", "rest"];
    for name in defaults {
        connection
            .execute(
                "INSERT OR IGNORE INTO spheres (id, name, created_at) VALUES (?1, ?2, ?3)",
                params![Uuid::new_v4(), name, Utc::now().to_rfc3339()],
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
            TaskStatus::Completed => "completed",
            TaskStatus::Skipped => "skipped",
        }
    }

    fn from_db(value: &str) -> Self {
        match value {
            "completed" => Self::Completed,
            "skipped" => Self::Skipped,
            _ => Self::Planned,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SqliteRepository;
    use crate::infrastructure::Repository;

    #[tokio::test]
    async fn sqlite_repository_seeds_default_spheres() {
        let database = tempfile::NamedTempFile::new().unwrap();
        let repository = SqliteRepository::new(database.path()).unwrap();

        let spheres = repository.list_spheres().await.unwrap();

        assert!(!spheres.is_empty());
        assert!(spheres.iter().any(|sphere| sphere.name == "health"));
    }
}
