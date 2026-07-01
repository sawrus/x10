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
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Sphere {
    pub id: Uuid,
    pub name: String,
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
    Completed,
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
    pub weight: i32,
    pub cadence: TaskCadence,
    pub scheduled_for: NaiveDate,
    pub status: TaskStatus,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DaySummary {
    pub date: NaiveDate,
    pub positive_weight: i32,
    pub negative_weight: i32,
    pub net_score: i32,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DailySnapshot {
    pub id: Uuid,
    pub date: NaiveDate,
    pub summary: DaySummary,
    pub balance_score: i32,
    pub level: String,
    pub rest_day_credits: i32,
    pub finalized_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Dashboard {
    pub profile_id: Uuid,
    pub current: ProgressionSummary,
    pub today: DaySummary,
    pub history: Vec<DailySnapshot>,
}

use crate::domain::scoring::ProgressionSummary;
