use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::{DailySnapshot, DaySummary, Task, TaskKind, TaskStatus};

#[derive(Debug, Clone)]
pub struct ProgressionConfig {
    pub positive_day_threshold: i32,
    pub points_per_level: i32,
}

impl Default for ProgressionConfig {
    fn default() -> Self {
        Self {
            positive_day_threshold: 3,
            points_per_level: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProgressionSummary {
    pub balance_score: i32,
    pub level: String,
    pub rest_day_credits: i32,
}

pub struct ProgressionEngine {
    config: ProgressionConfig,
}

impl ProgressionEngine {
    pub fn new(config: ProgressionConfig) -> Self {
        Self { config }
    }

    pub fn summarize_tasks(&self, tasks: &[Task], date: NaiveDate) -> DaySummary {
        let mut summary = DaySummary {
            date,
            positive_weight: 0,
            negative_weight: 0,
            net_score: 0,
        };

        for task in tasks {
            if task.status != TaskStatus::Completed || task.scheduled_for != date {
                continue;
            }

            match task.kind {
                TaskKind::Positive => summary.positive_weight += task.weight,
                TaskKind::Negative => summary.negative_weight += task.weight,
            }
        }

        summary.net_score = summary.positive_weight - summary.negative_weight;
        summary
    }

    pub fn finalize_snapshot(
        &self,
        summary: DaySummary,
        previous: Option<&ProgressionSummary>,
        finalized_at: DateTime<Utc>,
    ) -> DailySnapshot {
        let previous_balance = previous
            .map(|state| state.balance_score)
            .unwrap_or_default();
        let previous_credits = previous
            .map(|state| state.rest_day_credits)
            .unwrap_or_default();

        let balance_score = previous_balance + summary.net_score;
        let mut rest_day_credits = previous_credits;

        if summary.positive_weight >= self.config.positive_day_threshold {
            rest_day_credits += 1;
        }

        if summary.net_score < 0 {
            rest_day_credits = rest_day_credits.saturating_sub(1);
        }

        DailySnapshot {
            id: uuid::Uuid::new_v4(),
            date: summary.date,
            summary,
            balance_score,
            level: self.level_from_balance(balance_score),
            rest_day_credits,
            finalized_at,
        }
    }

    pub fn current_from_history(&self, history: &[DailySnapshot]) -> ProgressionSummary {
        history
            .last()
            .map(|snapshot| ProgressionSummary {
                balance_score: snapshot.balance_score,
                level: snapshot.level.clone(),
                rest_day_credits: snapshot.rest_day_credits,
            })
            .unwrap_or_else(|| ProgressionSummary {
                balance_score: 0,
                level: self.level_from_balance(0),
                rest_day_credits: 0,
            })
    }

    pub fn level_from_balance(&self, balance_score: i32) -> String {
        let level = (balance_score.div_euclid(self.config.points_per_level)).clamp(0, 10);
        format!("x{level}")
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Utc};
    use uuid::Uuid;

    use crate::domain::{Task, TaskCadence, TaskKind, TaskStatus};

    use super::{ProgressionConfig, ProgressionEngine};

    fn sample_task(date: NaiveDate, kind: TaskKind, weight: i32) -> Task {
        Task {
            id: Uuid::new_v4(),
            profile_id: Uuid::new_v4(),
            title: "task".to_owned(),
            sphere_id: None,
            kind,
            weight,
            cadence: TaskCadence::Day,
            scheduled_for: date,
            status: TaskStatus::Completed,
            completed_at: None,
        }
    }

    #[test]
    fn awards_rest_day_credit_when_positive_threshold_is_met() {
        let date = NaiveDate::from_ymd_opt(2026, 6, 30).unwrap();
        let engine = ProgressionEngine::new(ProgressionConfig::default());
        let tasks = vec![
            sample_task(date, TaskKind::Positive, 2),
            sample_task(date, TaskKind::Positive, 1),
        ];

        let summary = engine.summarize_tasks(&tasks, date);
        let snapshot = engine.finalize_snapshot(summary.clone(), None, Utc::now());
        let current = engine.current_from_history(std::slice::from_ref(&snapshot));

        assert_eq!(summary.net_score, 3);
        assert_eq!(current.rest_day_credits, 1);
        assert_eq!(snapshot.rest_day_credits, 1);
    }

    #[test]
    fn negative_day_consumes_one_rest_day_credit() {
        let day_one = NaiveDate::from_ymd_opt(2026, 6, 29).unwrap();
        let day_two = NaiveDate::from_ymd_opt(2026, 6, 30).unwrap();
        let engine = ProgressionEngine::new(ProgressionConfig::default());

        let first = engine.finalize_snapshot(
            engine.summarize_tasks(&[sample_task(day_one, TaskKind::Positive, 3)], day_one),
            None,
            Utc::now(),
        );
        let previous = engine.current_from_history(std::slice::from_ref(&first));
        let second = engine.finalize_snapshot(
            engine.summarize_tasks(&[sample_task(day_two, TaskKind::Negative, 2)], day_two),
            Some(&previous),
            Utc::now(),
        );
        let current = engine.current_from_history(&[first.clone(), second.clone()]);

        assert_eq!(first.rest_day_credits, 1);
        assert_eq!(second.rest_day_credits, 0);
        assert_eq!(current.rest_day_credits, 0);
    }

    #[test]
    fn clamps_level_inside_x0_and_x10() {
        let date = NaiveDate::from_ymd_opt(2026, 6, 30).unwrap();
        let engine = ProgressionEngine::new(ProgressionConfig {
            positive_day_threshold: 3,
            points_per_level: 1,
        });

        let snapshot = engine.finalize_snapshot(
            engine.summarize_tasks(&[sample_task(date, TaskKind::Positive, 20)], date),
            None,
            Utc::now(),
        );
        let current = engine.current_from_history(&[snapshot]);

        assert_eq!(current.level, "x10");
    }
}
