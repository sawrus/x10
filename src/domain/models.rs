use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Profile {
    pub id: Uuid,
    pub full_name: String,
    pub birth_date: NaiveDate,
    pub occupation: String,
    pub telegram: Option<String>,
    pub email: Option<String>,
    pub timezone: String,
    pub current_photo_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProfilePhoto {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub storage_path: String,
    pub original_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProfilePhotoSummary {
    pub id: Uuid,
    pub original_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub content_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Sphere {
    pub id: Uuid,
    pub name: String,
    pub weight: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskKind {
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Planned,
    Archived,
    Skipped,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskCadence {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Task {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub title: String,
    pub sphere_id: Option<Uuid>,
    pub kind: TaskKind,
    pub planned_weight: i32,
    pub planned_score: i32,
    pub planned_rate: i32,
    pub cadence: TaskCadence,
    pub starts_on: NaiveDate,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TaskExecution {
    pub id: Uuid,
    pub task_id: Uuid,
    pub profile_id: Uuid,
    pub actual_score: i32,
    pub actual_rate: i32,
    pub completed_at: DateTime<Utc>,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProfileBalance {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub task_execution_id: Uuid,
    pub actual_rate: i32,
    pub actual_score: i32,
    pub actual_weight: i32,
    pub balance_after: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Level {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub code: String,
    pub ordinal: i32,
    pub min_balance: i32,
    pub target_planned_score: i32,
    pub target_planned_rate: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProfileLevelState {
    pub profile_id: Uuid,
    pub current_level_id: Uuid,
    pub last_balance_id: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DayFinalization {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub date: NaiveDate,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProgressionSummary {
    pub balance_score: i32,
    pub current_level: String,
    pub current_level_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Dashboard {
    pub profile: Profile,
    pub current_photo: Option<ProfilePhotoSummary>,
    pub current: ProgressionSummary,
    pub tasks: Vec<Task>,
    pub execution_queue: Vec<Task>,
    pub recent_executions: Vec<TaskExecution>,
    pub balances: Vec<ProfileBalance>,
    pub levels: Vec<Level>,
    pub finalizations: Vec<DayFinalization>,
}
