use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::domain::{Level, ProfileBalance, ProfileLevelState, Task, TaskExecution, TaskKind};

#[derive(Debug, Clone, Copy)]
pub struct ProgressionConfig;

impl Default for ProgressionConfig {
    fn default() -> Self {
        Self
    }
}

pub struct ProgressionEngine {
    _config: ProgressionConfig,
}

impl ProgressionEngine {
    pub fn new(config: ProgressionConfig) -> Self {
        Self { _config: config }
    }

    pub fn signed_weight(&self, task: &Task) -> i32 {
        match task.kind {
            TaskKind::Positive => task.planned_weight,
            TaskKind::Negative => -task.planned_weight,
        }
    }

    pub fn period_for(
        &self,
        starts_on: NaiveDate,
        cadence: crate::domain::TaskCadence,
    ) -> (NaiveDate, NaiveDate) {
        match cadence {
            crate::domain::TaskCadence::Day => (starts_on, starts_on),
            crate::domain::TaskCadence::Week => (starts_on, starts_on + chrono::Days::new(6)),
            crate::domain::TaskCadence::Month => {
                let end = starts_on + chrono::Months::new(1) - chrono::Days::new(1);
                (starts_on, end)
            }
            crate::domain::TaskCadence::Year => {
                let end = starts_on + chrono::Months::new(12) - chrono::Days::new(1);
                (starts_on, end)
            }
        }
    }

    pub fn balance_from_history(&self, balances: &[ProfileBalance]) -> i32 {
        balances
            .last()
            .map(|entry| entry.balance_after)
            .unwrap_or_default()
    }

    pub fn current_level<'a>(&self, levels: &'a [Level], balance: i32) -> Option<&'a Level> {
        levels
            .iter()
            .filter(|level| balance >= level.min_balance)
            .max_by_key(|level| level.ordinal)
            .or_else(|| levels.iter().min_by_key(|level| level.ordinal))
    }

    pub fn create_balance_entry(
        &self,
        profile_id: Uuid,
        task: &Task,
        execution: &TaskExecution,
        previous_balance: i32,
        created_at: DateTime<Utc>,
    ) -> ProfileBalance {
        let actual_weight = self.signed_weight(task);

        ProfileBalance {
            id: Uuid::new_v4(),
            profile_id,
            task_execution_id: execution.id,
            actual_rate: execution.actual_rate,
            actual_score: execution.actual_score,
            actual_weight,
            balance_after: previous_balance + actual_weight,
            created_at,
        }
    }

    pub fn state_for_level(
        &self,
        profile_id: Uuid,
        current_level_id: Uuid,
        last_balance_id: Option<Uuid>,
        updated_at: DateTime<Utc>,
    ) -> ProfileLevelState {
        ProfileLevelState {
            profile_id,
            current_level_id,
            last_balance_id,
            updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Utc};
    use uuid::Uuid;

    use crate::domain::{Level, Task, TaskCadence, TaskExecution, TaskKind, TaskStatus};

    use super::{ProgressionConfig, ProgressionEngine};

    fn sample_task(kind: TaskKind, planned_weight: i32) -> Task {
        let now = Utc::now();
        Task {
            id: Uuid::new_v4(),
            profile_id: Uuid::new_v4(),
            title: "task".to_owned(),
            sphere_id: None,
            kind,
            planned_weight,
            planned_score: 3,
            planned_rate: 50,
            cadence: TaskCadence::Day,
            starts_on: NaiveDate::from_ymd_opt(2026, 7, 2).unwrap(),
            status: TaskStatus::Planned,
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn signed_weight_respects_task_kind() {
        let engine = ProgressionEngine::new(ProgressionConfig);

        assert_eq!(engine.signed_weight(&sample_task(TaskKind::Positive, 3)), 3);
        assert_eq!(
            engine.signed_weight(&sample_task(TaskKind::Negative, 3)),
            -3
        );
    }

    #[test]
    fn picks_highest_level_under_balance() {
        let engine = ProgressionEngine::new(ProgressionConfig);
        let profile_id = Uuid::new_v4();
        let now = Utc::now();
        let levels = vec![
            Level {
                id: Uuid::new_v4(),
                profile_id,
                code: "x1".into(),
                ordinal: 1,
                min_balance: 0,
                target_planned_score: 2,
                target_planned_rate: 20,
                created_at: now,
                updated_at: now,
            },
            Level {
                id: Uuid::new_v4(),
                profile_id,
                code: "x2".into(),
                ordinal: 2,
                min_balance: 10,
                target_planned_score: 3,
                target_planned_rate: 40,
                created_at: now,
                updated_at: now,
            },
        ];

        assert_eq!(engine.current_level(&levels, 15).unwrap().code, "x2");
    }

    #[test]
    fn balance_entry_accumulates_signed_weight() {
        let engine = ProgressionEngine::new(ProgressionConfig);
        let task = sample_task(TaskKind::Negative, 4);
        let execution = TaskExecution {
            id: Uuid::new_v4(),
            task_id: task.id,
            profile_id: task.profile_id,
            actual_score: 4,
            actual_rate: 80,
            completed_at: Utc::now(),
            period_start: task.starts_on,
            period_end: task.starts_on,
            created_at: Utc::now(),
        };

        let balance =
            engine.create_balance_entry(task.profile_id, &task, &execution, 7, Utc::now());

        assert_eq!(balance.balance_after, 3);
        assert_eq!(balance.actual_weight, -4);
    }
}
