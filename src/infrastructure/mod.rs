mod sqlite;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::{
    DayFinalization, Level, Profile, ProfileBalance, ProfileLevelState, ProfilePhoto, Sphere,
    Task, TaskExecution,
};

pub use sqlite::SqliteRepository;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository conflict: {0}")]
    Conflict(String),
    #[error("repository operation failed: {0}")]
    Operation(String),
}

#[async_trait]
pub trait Repository: Send + Sync + 'static {
    async fn initialize_profile(&self, profile: Profile) -> Result<Vec<Level>, RepositoryError>;
    async fn get_profile(&self, profile_id: Uuid) -> Result<Option<Profile>, RepositoryError>;
    async fn update_profile(&self, profile: Profile) -> Result<(), RepositoryError>;

    async fn create_photo(&self, photo: ProfilePhoto) -> Result<(), RepositoryError>;
    async fn list_photos(&self, profile_id: Uuid) -> Result<Vec<ProfilePhoto>, RepositoryError>;
    async fn get_photo(&self, photo_id: Uuid) -> Result<Option<ProfilePhoto>, RepositoryError>;
    async fn delete_photo(&self, photo_id: Uuid) -> Result<(), RepositoryError>;
    async fn set_current_photo(
        &self,
        profile_id: Uuid,
        photo_id: Option<Uuid>,
    ) -> Result<(), RepositoryError>;

    async fn create_sphere(&self, sphere: Sphere) -> Result<(), RepositoryError>;
    async fn list_spheres(&self) -> Result<Vec<Sphere>, RepositoryError>;
    async fn get_sphere(&self, sphere_id: Uuid) -> Result<Option<Sphere>, RepositoryError>;
    async fn update_sphere(&self, sphere: Sphere) -> Result<(), RepositoryError>;
    async fn delete_sphere(&self, sphere_id: Uuid) -> Result<(), RepositoryError>;

    async fn create_task(&self, task: Task) -> Result<(), RepositoryError>;
    async fn list_tasks(&self, profile_id: Uuid) -> Result<Vec<Task>, RepositoryError>;
    async fn get_task(&self, task_id: Uuid) -> Result<Option<Task>, RepositoryError>;
    async fn update_task(&self, task: Task) -> Result<(), RepositoryError>;
    async fn delete_task(&self, task_id: Uuid) -> Result<(), RepositoryError>;

    async fn create_execution(
        &self,
        task: &Task,
        actual_score: i32,
        actual_rate: i32,
        completed_at: DateTime<Utc>,
        period_start: NaiveDate,
        period_end: NaiveDate,
    ) -> Result<(TaskExecution, ProfileBalance), RepositoryError>;
    async fn list_executions(
        &self,
        profile_id: Uuid,
    ) -> Result<Vec<TaskExecution>, RepositoryError>;
    async fn get_execution(
        &self,
        execution_id: Uuid,
    ) -> Result<Option<TaskExecution>, RepositoryError>;
    async fn delete_execution(
        &self,
        execution_id: Uuid,
    ) -> Result<Option<TaskExecution>, RepositoryError>;

    async fn list_balances(
        &self,
        profile_id: Uuid,
    ) -> Result<Vec<ProfileBalance>, RepositoryError>;

    async fn list_levels(&self, profile_id: Uuid) -> Result<Vec<Level>, RepositoryError>;
    async fn get_level(&self, level_id: Uuid) -> Result<Option<Level>, RepositoryError>;
    async fn create_level(&self, level: Level) -> Result<(), RepositoryError>;
    async fn update_level(&self, level: Level) -> Result<(), RepositoryError>;
    async fn delete_level(&self, level_id: Uuid) -> Result<(), RepositoryError>;
    async fn get_level_state(
        &self,
        profile_id: Uuid,
    ) -> Result<Option<ProfileLevelState>, RepositoryError>;
    async fn recalculate_level_state(
        &self,
        profile_id: Uuid,
    ) -> Result<Option<ProfileLevelState>, RepositoryError>;

    async fn create_day_finalization(
        &self,
        finalization: DayFinalization,
    ) -> Result<(), RepositoryError>;
    async fn list_day_finalizations(
        &self,
        profile_id: Uuid,
    ) -> Result<Vec<DayFinalization>, RepositoryError>;
}
