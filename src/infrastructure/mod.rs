mod sqlite;

use async_trait::async_trait;
use chrono::NaiveDate;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::{DailySnapshot, Profile, Sphere, Task};

pub use sqlite::SqliteRepository;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository operation failed: {0}")]
    Operation(String),
}

#[async_trait]
pub trait Repository: Send + Sync + 'static {
    async fn create_profile(&self, profile: Profile) -> Result<(), RepositoryError>;
    async fn get_profile(&self, profile_id: Uuid) -> Result<Option<Profile>, RepositoryError>;
    async fn create_sphere(&self, sphere: Sphere) -> Result<(), RepositoryError>;
    async fn list_spheres(&self) -> Result<Vec<Sphere>, RepositoryError>;
    async fn create_task(&self, task: Task) -> Result<(), RepositoryError>;
    async fn get_task(&self, task_id: Uuid) -> Result<Option<Task>, RepositoryError>;
    async fn update_task(&self, task: Task) -> Result<(), RepositoryError>;
    async fn list_tasks(&self, profile_id: Uuid) -> Result<Vec<Task>, RepositoryError>;
    async fn list_tasks_for_date(
        &self,
        profile_id: Uuid,
        date: NaiveDate,
    ) -> Result<Vec<Task>, RepositoryError>;
    async fn create_snapshot(
        &self,
        snapshot: DailySnapshot,
        profile_id: Uuid,
    ) -> Result<(), RepositoryError>;
    async fn get_snapshot(
        &self,
        profile_id: Uuid,
        date: NaiveDate,
    ) -> Result<Option<DailySnapshot>, RepositoryError>;
    async fn list_snapshots(&self, profile_id: Uuid)
    -> Result<Vec<DailySnapshot>, RepositoryError>;
}
