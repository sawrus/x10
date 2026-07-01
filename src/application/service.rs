use std::sync::Arc;

use chrono::{NaiveDate, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    domain::{
        DailySnapshot, Dashboard, Profile, ProgressionConfig, ProgressionEngine, Sphere, Task,
        TaskCadence, TaskKind, TaskStatus,
    },
    infrastructure::{Repository, RepositoryError},
};

#[derive(Debug, Clone, Copy)]
pub struct Actor {
    pub profile_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct CreateProfileRequest {
    pub full_name: String,
    pub birth_date: NaiveDate,
    pub occupation: String,
    pub telegram: Option<String>,
    pub email: Option<String>,
    pub timezone: String,
}

#[derive(Debug, Clone)]
pub struct CreateTaskRequest {
    pub profile_id: Uuid,
    pub title: String,
    pub sphere_id: Option<Uuid>,
    pub kind: TaskKind,
    pub weight: i32,
    pub cadence: TaskCadence,
    pub scheduled_for: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct CreateSphereRequest {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct CompleteTaskRequest {
    pub task_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FinalizeDayRequest {
    pub profile_id: Uuid,
    pub date: NaiveDate,
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("resource not found")]
    NotFound,
    #[error("forbidden")]
    Forbidden,
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("storage failed: {0}")]
    Storage(String),
}

pub struct ProgressionService<R: Repository> {
    repository: Arc<R>,
    engine: ProgressionEngine,
}

impl<R: Repository> ProgressionService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self {
            repository,
            engine: ProgressionEngine::new(ProgressionConfig::default()),
        }
    }

    pub async fn create_profile(
        &self,
        request: CreateProfileRequest,
    ) -> Result<Profile, ServiceError> {
        if request.full_name.trim().is_empty() {
            return Err(ServiceError::Validation("full_name is required".to_owned()));
        }

        if request.occupation.trim().is_empty() {
            return Err(ServiceError::Validation(
                "occupation is required".to_owned(),
            ));
        }

        if request.timezone.trim().is_empty() {
            return Err(ServiceError::Validation("timezone is required".to_owned()));
        }

        let profile = Profile {
            id: Uuid::new_v4(),
            full_name: request.full_name,
            birth_date: request.birth_date,
            occupation: request.occupation,
            telegram: request.telegram,
            email: request.email,
            timezone: request.timezone,
        };

        self.repository
            .create_profile(profile.clone())
            .await
            .map_err(Self::storage_error)?;

        Ok(profile)
    }

    pub async fn get_profile(
        &self,
        actor: Actor,
        profile_id: Uuid,
    ) -> Result<Profile, ServiceError> {
        self.authorize(actor, profile_id)?;
        self.repository
            .get_profile(profile_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)
    }

    pub async fn create_sphere(
        &self,
        request: CreateSphereRequest,
    ) -> Result<Sphere, ServiceError> {
        if request.name.trim().is_empty() {
            return Err(ServiceError::Validation("name is required".to_owned()));
        }

        let sphere = Sphere {
            id: Uuid::new_v4(),
            name: request.name,
        };

        self.repository
            .create_sphere(sphere.clone())
            .await
            .map_err(Self::storage_error)?;

        Ok(sphere)
    }

    pub async fn list_spheres(&self) -> Result<Vec<Sphere>, ServiceError> {
        self.repository
            .list_spheres()
            .await
            .map_err(Self::storage_error)
    }

    pub async fn create_task(
        &self,
        actor: Actor,
        request: CreateTaskRequest,
    ) -> Result<Task, ServiceError> {
        self.authorize(actor, request.profile_id)?;

        if request.title.trim().is_empty() {
            return Err(ServiceError::Validation("title is required".to_owned()));
        }

        if request.weight <= 0 {
            return Err(ServiceError::Validation(
                "weight must be greater than zero".to_owned(),
            ));
        }

        self.repository
            .get_profile(request.profile_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;

        if let Some(sphere_id) = request.sphere_id {
            let known_spheres = self
                .repository
                .list_spheres()
                .await
                .map_err(Self::storage_error)?;
            if !known_spheres.iter().any(|sphere| sphere.id == sphere_id) {
                return Err(ServiceError::Validation(
                    "sphere_id does not exist".to_owned(),
                ));
            }
        }

        let task = Task {
            id: Uuid::new_v4(),
            profile_id: request.profile_id,
            title: request.title,
            sphere_id: request.sphere_id,
            kind: request.kind,
            weight: request.weight,
            cadence: request.cadence,
            scheduled_for: request.scheduled_for,
            status: TaskStatus::Planned,
            completed_at: None,
        };

        self.repository
            .create_task(task.clone())
            .await
            .map_err(Self::storage_error)?;
        Ok(task)
    }

    pub async fn complete_task(
        &self,
        actor: Actor,
        request: CompleteTaskRequest,
    ) -> Result<Task, ServiceError> {
        let task = self
            .repository
            .get_task(request.task_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;

        self.authorize(actor, task.profile_id)?;

        if task.status == TaskStatus::Completed {
            return Err(ServiceError::Conflict(
                "task is already completed".to_owned(),
            ));
        }

        let updated = Task {
            status: TaskStatus::Completed,
            completed_at: Some(Utc::now()),
            ..task
        };

        self.repository
            .update_task(updated.clone())
            .await
            .map_err(Self::storage_error)?;
        Ok(updated)
    }

    pub async fn finalize_day(
        &self,
        actor: Actor,
        request: FinalizeDayRequest,
    ) -> Result<DailySnapshot, ServiceError> {
        self.authorize(actor, request.profile_id)?;

        if self
            .repository
            .get_snapshot(request.profile_id, request.date)
            .await
            .map_err(Self::storage_error)?
            .is_some()
        {
            return Err(ServiceError::Conflict(
                "day is already finalized".to_owned(),
            ));
        }

        let tasks = self
            .repository
            .list_tasks_for_date(request.profile_id, request.date)
            .await
            .map_err(Self::storage_error)?;
        let summary = self.engine.summarize_tasks(&tasks, request.date);
        let history = self
            .repository
            .list_snapshots(request.profile_id)
            .await
            .map_err(Self::storage_error)?;
        let previous = self.engine.current_from_history(&history);
        let snapshot = self.engine.finalize_snapshot(
            summary,
            (!history.is_empty()).then_some(&previous),
            Utc::now(),
        );

        self.repository
            .create_snapshot(snapshot.clone(), request.profile_id)
            .await
            .map_err(Self::storage_error)?;

        Ok(snapshot)
    }

    pub async fn dashboard(
        &self,
        actor: Actor,
        profile_id: Uuid,
        date: Option<NaiveDate>,
    ) -> Result<Dashboard, ServiceError> {
        self.authorize(actor, profile_id)?;

        self.repository
            .get_profile(profile_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;

        let today = date.unwrap_or_else(|| Utc::now().date_naive());
        let tasks = self
            .repository
            .list_tasks(profile_id)
            .await
            .map_err(Self::storage_error)?;
        let history = self
            .repository
            .list_snapshots(profile_id)
            .await
            .map_err(Self::storage_error)?;
        let current = self.engine.current_from_history(&history);
        let today_summary = history
            .iter()
            .find(|snapshot| snapshot.date == today)
            .map(|snapshot| snapshot.summary.clone())
            .unwrap_or_else(|| self.engine.summarize_tasks(&tasks, today));

        Ok(Dashboard {
            profile_id,
            current,
            today: today_summary,
            history,
        })
    }

    fn authorize(&self, actor: Actor, profile_id: Uuid) -> Result<(), ServiceError> {
        if actor.profile_id != profile_id {
            return Err(ServiceError::Forbidden);
        }

        Ok(())
    }

    fn storage_error(error: RepositoryError) -> ServiceError {
        ServiceError::Storage(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveDate;

    use super::{
        Actor, CompleteTaskRequest, CreateProfileRequest, CreateTaskRequest, FinalizeDayRequest,
        ProgressionService, ServiceError,
    };
    use crate::{
        domain::{TaskCadence, TaskKind},
        infrastructure::SqliteRepository,
    };

    #[tokio::test]
    async fn finalize_day_persists_snapshot_and_prevents_second_finalize() {
        let database = tempfile::NamedTempFile::new().unwrap();
        let repository = Arc::new(SqliteRepository::new(database.path()).unwrap());
        let service = ProgressionService::new(repository);
        let scheduled_for = NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();

        let profile = service
            .create_profile(CreateProfileRequest {
                full_name: "Test User".to_owned(),
                birth_date: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                occupation: "Builder".to_owned(),
                telegram: None,
                email: Some("test@example.com".to_owned()),
                timezone: "Europe/Samara".to_owned(),
            })
            .await
            .unwrap();
        let actor = Actor {
            profile_id: profile.id,
        };
        let sphere_id = service.list_spheres().await.unwrap()[0].id;

        let task = service
            .create_task(
                actor,
                CreateTaskRequest {
                    profile_id: profile.id,
                    title: "Morning run".to_owned(),
                    sphere_id: Some(sphere_id),
                    kind: TaskKind::Positive,
                    weight: 3,
                    cadence: TaskCadence::Day,
                    scheduled_for,
                },
            )
            .await
            .unwrap();

        service
            .complete_task(actor, CompleteTaskRequest { task_id: task.id })
            .await
            .unwrap();

        let snapshot = service
            .finalize_day(
                actor,
                FinalizeDayRequest {
                    profile_id: profile.id,
                    date: scheduled_for,
                },
            )
            .await
            .unwrap();

        assert_eq!(snapshot.summary.net_score, 3);
        assert_eq!(snapshot.rest_day_credits, 1);

        let dashboard = service
            .dashboard(actor, profile.id, Some(scheduled_for))
            .await
            .unwrap();
        assert_eq!(dashboard.history.len(), 1);
        assert_eq!(dashboard.current.rest_day_credits, 1);
        assert_eq!(dashboard.today.net_score, 3);

        let repeated = service
            .finalize_day(
                actor,
                FinalizeDayRequest {
                    profile_id: profile.id,
                    date: scheduled_for,
                },
            )
            .await;

        assert!(matches!(repeated, Err(ServiceError::Conflict(_))));
    }
}
