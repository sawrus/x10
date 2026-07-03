use std::{path::PathBuf, sync::Arc};

use chrono::{DateTime, Days, Months, NaiveDate, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    domain::{
        Dashboard, DayFinalization, Level, Profile, ProfileBalance, ProfileLevelState,
        ProfilePhoto, ProfilePhotoSummary, ProgressionConfig, ProgressionEngine,
        ProgressionSummary, Sphere, Task, TaskCadence, TaskExecution, TaskKind, TaskStatus,
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

#[derive(Debug, Clone, Default)]
pub struct UpdateProfileRequest {
    pub full_name: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub occupation: Option<String>,
    pub telegram: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PhotoUpload {
    pub original_name: String,
    pub mime_type: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct CreateSphereRequest {
    pub name: String,
    pub weight: i32,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateSphereRequest {
    pub name: Option<String>,
    pub weight: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct CreateTaskRequest {
    pub profile_id: Uuid,
    pub title: String,
    pub sphere_id: Option<Uuid>,
    pub kind: TaskKind,
    pub planned_weight: i32,
    pub planned_score: i32,
    pub planned_rate: i32,
    pub cadence: TaskCadence,
    pub starts_on: NaiveDate,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub sphere_id: Option<Option<Uuid>>,
    pub kind: Option<TaskKind>,
    pub planned_weight: Option<i32>,
    pub planned_score: Option<i32>,
    pub planned_rate: Option<i32>,
    pub cadence: Option<TaskCadence>,
    pub starts_on: Option<NaiveDate>,
    pub status: Option<TaskStatus>,
}

#[derive(Debug, Clone)]
pub struct CreateTaskExecutionRequest {
    pub actual_score: i32,
    pub actual_rate: i32,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct CreateLevelRequest {
    pub profile_id: Uuid,
    pub code: String,
    pub ordinal: i32,
    pub min_balance: i32,
    pub target_planned_score: i32,
    pub target_planned_rate: i32,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateLevelRequest {
    pub code: Option<String>,
    pub ordinal: Option<i32>,
    pub min_balance: Option<i32>,
    pub target_planned_score: Option<i32>,
    pub target_planned_rate: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct CreateDayFinalizationRequest {
    pub profile_id: Uuid,
    pub date: NaiveDate,
    pub note: Option<String>,
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
    uploads_path: PathBuf,
}

impl<R: Repository> ProgressionService<R> {
    pub fn new(repository: Arc<R>, uploads_path: PathBuf) -> Self {
        std::fs::create_dir_all(&uploads_path).expect("uploads directory should initialize");
        Self {
            repository,
            engine: ProgressionEngine::new(ProgressionConfig),
            uploads_path,
        }
    }

    pub async fn create_profile(
        &self,
        request: CreateProfileRequest,
    ) -> Result<(Profile, Vec<Level>), ServiceError> {
        require_non_empty("full_name", &request.full_name)?;
        require_non_empty("occupation", &request.occupation)?;
        require_non_empty("timezone", &request.timezone)?;

        let profile = Profile {
            id: Uuid::new_v4(),
            full_name: request.full_name.trim().to_owned(),
            birth_date: request.birth_date,
            occupation: request.occupation.trim().to_owned(),
            telegram: request.telegram.filter(|value| !value.trim().is_empty()),
            email: request.email.filter(|value| !value.trim().is_empty()),
            timezone: request.timezone.trim().to_owned(),
            current_photo_id: None,
        };

        let levels = self
            .repository
            .initialize_profile(profile.clone())
            .await
            .map_err(Self::storage_error)?;

        Ok((profile, levels))
    }

    pub async fn list_profiles(&self) -> Result<Vec<Profile>, ServiceError> {
        self.repository
            .list_profiles()
            .await
            .map_err(Self::storage_error)
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

    pub async fn update_profile(
        &self,
        actor: Actor,
        profile_id: Uuid,
        request: UpdateProfileRequest,
    ) -> Result<Profile, ServiceError> {
        self.authorize(actor, profile_id)?;
        let mut profile = self.get_profile(actor, profile_id).await?;

        if let Some(full_name) = request.full_name {
            require_non_empty("full_name", &full_name)?;
            profile.full_name = full_name.trim().to_owned();
        }
        if let Some(birth_date) = request.birth_date {
            profile.birth_date = birth_date;
        }
        if let Some(occupation) = request.occupation {
            require_non_empty("occupation", &occupation)?;
            profile.occupation = occupation.trim().to_owned();
        }
        if let Some(telegram) = request.telegram {
            profile.telegram = telegram.filter(|value| !value.trim().is_empty());
        }
        if let Some(email) = request.email {
            profile.email = email.filter(|value| !value.trim().is_empty());
        }
        if let Some(timezone) = request.timezone {
            require_non_empty("timezone", &timezone)?;
            profile.timezone = timezone.trim().to_owned();
        }

        self.repository
            .update_profile(profile.clone())
            .await
            .map_err(Self::storage_error)?;

        Ok(profile)
    }

    pub async fn delete_profile(&self, profile_id: Uuid) -> Result<(), ServiceError> {
        let photos = self
            .repository
            .list_photos(profile_id)
            .await
            .map_err(Self::storage_error)?;
        self.ensure_profile_exists(profile_id).await?;
        self.repository
            .delete_profile(profile_id)
            .await
            .map_err(Self::storage_error)?;
        for photo in photos {
            let _ = tokio::fs::remove_file(self.uploads_path.join(&photo.storage_path)).await;
        }
        let _ = tokio::fs::remove_dir(self.uploads_path.join(profile_id.to_string())).await;
        Ok(())
    }

    pub async fn upload_photo(
        &self,
        actor: Actor,
        profile_id: Uuid,
        upload: PhotoUpload,
    ) -> Result<ProfilePhotoSummary, ServiceError> {
        self.authorize(actor, profile_id)?;
        self.ensure_profile_exists(profile_id).await?;
        if upload.bytes.is_empty() {
            return Err(ServiceError::Validation("file is required".to_owned()));
        }
        if upload.mime_type.trim().is_empty() {
            return Err(ServiceError::Validation("mime_type is required".to_owned()));
        }

        let photo_id = Uuid::new_v4();
        let sanitized_name = sanitize_file_name(&upload.original_name);
        let relative_dir = format!("{profile_id}");
        let relative_path = format!("{relative_dir}/{photo_id}-{sanitized_name}");
        let absolute_path = self.uploads_path.join(&relative_path);
        if let Some(parent) = absolute_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|error| ServiceError::Storage(error.to_string()))?;
        }
        tokio::fs::write(&absolute_path, &upload.bytes)
            .await
            .map_err(|error| ServiceError::Storage(error.to_string()))?;

        let now = Utc::now();
        let photo = ProfilePhoto {
            id: photo_id,
            profile_id,
            storage_path: relative_path.clone(),
            original_name: if upload.original_name.trim().is_empty() {
                "upload.bin".to_owned()
            } else {
                upload.original_name
            },
            mime_type: upload.mime_type,
            size_bytes: i64::try_from(upload.bytes.len())
                .map_err(|_| ServiceError::Validation("file is too large".to_owned()))?,
            width: None,
            height: None,
            created_at: now,
            updated_at: now,
        };

        self.repository
            .create_photo(photo.clone())
            .await
            .map_err(Self::storage_error)?;

        Ok(self.photo_summary(&photo))
    }

    pub async fn list_photos(
        &self,
        actor: Actor,
        profile_id: Uuid,
    ) -> Result<Vec<ProfilePhotoSummary>, ServiceError> {
        self.authorize(actor, profile_id)?;
        Ok(self
            .repository
            .list_photos(profile_id)
            .await
            .map_err(Self::storage_error)?
            .into_iter()
            .map(|photo| self.photo_summary(&photo))
            .collect())
    }

    pub async fn get_photo_file(
        &self,
        actor: Actor,
        photo_id: Uuid,
    ) -> Result<(ProfilePhoto, Vec<u8>), ServiceError> {
        let photo = self
            .repository
            .get_photo(photo_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;
        self.authorize(actor, photo.profile_id)?;
        let bytes = tokio::fs::read(self.uploads_path.join(&photo.storage_path))
            .await
            .map_err(|error| ServiceError::Storage(error.to_string()))?;
        Ok((photo, bytes))
    }

    pub async fn delete_photo(&self, actor: Actor, photo_id: Uuid) -> Result<(), ServiceError> {
        let photo = self
            .repository
            .get_photo(photo_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;
        self.authorize(actor, photo.profile_id)?;
        let profile = self.get_profile(actor, photo.profile_id).await?;
        if profile.current_photo_id == Some(photo.id) {
            return Err(ServiceError::Conflict(
                "current photo must be unselected before deletion".to_owned(),
            ));
        }

        self.repository
            .delete_photo(photo.id)
            .await
            .map_err(Self::storage_error)?;
        let _ = tokio::fs::remove_file(self.uploads_path.join(&photo.storage_path)).await;
        Ok(())
    }

    pub async fn select_photo(
        &self,
        actor: Actor,
        profile_id: Uuid,
        photo_id: Uuid,
    ) -> Result<Profile, ServiceError> {
        self.authorize(actor, profile_id)?;
        let photo = self
            .repository
            .get_photo(photo_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;
        if photo.profile_id != profile_id {
            return Err(ServiceError::Forbidden);
        }

        self.repository
            .set_current_photo(profile_id, Some(photo_id))
            .await
            .map_err(Self::storage_error)?;
        self.get_profile(actor, profile_id).await
    }

    pub async fn create_sphere(
        &self,
        request: CreateSphereRequest,
    ) -> Result<Sphere, ServiceError> {
        require_non_empty("name", &request.name)?;
        if request.weight <= 0 {
            return Err(ServiceError::Validation(
                "weight must be greater than zero".to_owned(),
            ));
        }

        let sphere = Sphere {
            id: Uuid::new_v4(),
            name: request.name.trim().to_owned(),
            weight: request.weight,
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

    pub async fn get_sphere(&self, sphere_id: Uuid) -> Result<Sphere, ServiceError> {
        self.repository
            .get_sphere(sphere_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)
    }

    pub async fn update_sphere(
        &self,
        sphere_id: Uuid,
        request: UpdateSphereRequest,
    ) -> Result<Sphere, ServiceError> {
        let mut sphere = self.get_sphere(sphere_id).await?;
        if let Some(name) = request.name {
            require_non_empty("name", &name)?;
            sphere.name = name.trim().to_owned();
        }
        if let Some(weight) = request.weight {
            if weight <= 0 {
                return Err(ServiceError::Validation(
                    "weight must be greater than zero".to_owned(),
                ));
            }
            sphere.weight = weight;
        }
        self.repository
            .update_sphere(sphere.clone())
            .await
            .map_err(Self::storage_error)?;
        Ok(sphere)
    }

    pub async fn delete_sphere(&self, sphere_id: Uuid) -> Result<(), ServiceError> {
        self.get_sphere(sphere_id).await?;
        self.repository
            .delete_sphere(sphere_id)
            .await
            .map_err(Self::storage_error)
    }

    pub async fn create_task(
        &self,
        actor: Actor,
        request: CreateTaskRequest,
    ) -> Result<Task, ServiceError> {
        self.authorize(actor, request.profile_id)?;
        self.ensure_profile_exists(request.profile_id).await?;
        self.validate_task_fields(
            &request.title,
            request.planned_weight,
            request.planned_score,
            request.planned_rate,
        )?;
        self.ensure_sphere_exists(request.sphere_id).await?;

        let now = Utc::now();
        let task = Task {
            id: Uuid::new_v4(),
            profile_id: request.profile_id,
            title: request.title.trim().to_owned(),
            sphere_id: request.sphere_id,
            kind: request.kind,
            planned_weight: request.planned_weight,
            planned_score: request.planned_score,
            planned_rate: request.planned_rate,
            cadence: request.cadence,
            starts_on: request.starts_on,
            status: TaskStatus::Planned,
            created_at: now,
            updated_at: now,
        };

        self.repository
            .create_task(task.clone())
            .await
            .map_err(Self::storage_error)?;
        Ok(task)
    }

    pub async fn list_tasks(
        &self,
        actor: Actor,
        profile_id: Uuid,
    ) -> Result<Vec<Task>, ServiceError> {
        self.authorize(actor, profile_id)?;
        self.ensure_profile_exists(profile_id).await?;
        self.repository
            .list_tasks(profile_id)
            .await
            .map_err(Self::storage_error)
    }

    pub async fn get_task(&self, actor: Actor, task_id: Uuid) -> Result<Task, ServiceError> {
        let task = self
            .repository
            .get_task(task_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;
        self.authorize(actor, task.profile_id)?;
        Ok(task)
    }

    pub async fn update_task(
        &self,
        actor: Actor,
        task_id: Uuid,
        request: UpdateTaskRequest,
    ) -> Result<Task, ServiceError> {
        let mut task = self.get_task(actor, task_id).await?;
        if let Some(title) = request.title {
            require_non_empty("title", &title)?;
            task.title = title.trim().to_owned();
        }
        if let Some(sphere_id) = request.sphere_id {
            self.ensure_sphere_exists(sphere_id).await?;
            task.sphere_id = sphere_id;
        }
        if let Some(kind) = request.kind {
            task.kind = kind;
        }
        if let Some(planned_weight) = request.planned_weight {
            if planned_weight <= 0 {
                return Err(ServiceError::Validation(
                    "planned_weight must be greater than zero".to_owned(),
                ));
            }
            task.planned_weight = planned_weight;
        }
        if let Some(planned_score) = request.planned_score {
            validate_score("planned_score", planned_score)?;
            task.planned_score = planned_score;
        }
        if let Some(planned_rate) = request.planned_rate {
            validate_rate("planned_rate", planned_rate)?;
            task.planned_rate = planned_rate;
        }
        if let Some(cadence) = request.cadence {
            task.cadence = cadence;
        }
        if let Some(starts_on) = request.starts_on {
            task.starts_on = starts_on;
        }
        if let Some(status) = request.status {
            task.status = status;
        }
        task.updated_at = Utc::now();

        self.repository
            .update_task(task.clone())
            .await
            .map_err(Self::storage_error)?;
        Ok(task)
    }

    pub async fn delete_task(&self, actor: Actor, task_id: Uuid) -> Result<(), ServiceError> {
        let task = self.get_task(actor, task_id).await?;
        self.repository
            .delete_task(task.id)
            .await
            .map_err(Self::storage_error)
    }

    pub async fn create_execution(
        &self,
        actor: Actor,
        task_id: Uuid,
        request: CreateTaskExecutionRequest,
    ) -> Result<TaskExecution, ServiceError> {
        let task = self.get_task(actor, task_id).await?;
        if task.status != TaskStatus::Planned {
            return Err(ServiceError::Conflict(
                "only planned tasks can be executed".to_owned(),
            ));
        }
        validate_score("actual_score", request.actual_score)?;
        validate_rate("actual_rate", request.actual_rate)?;

        let completed_at = request.completed_at.unwrap_or_else(Utc::now);
        let (period_start, period_end) =
            compute_period_bounds(task.starts_on, completed_at.date_naive(), task.cadence);
        let (execution, _) = self
            .repository
            .create_execution(
                &task,
                request.actual_score,
                request.actual_rate,
                completed_at,
                period_start,
                period_end,
            )
            .await
            .map_err(Self::storage_error)?;
        Ok(execution)
    }

    pub async fn list_executions(
        &self,
        actor: Actor,
        profile_id: Uuid,
    ) -> Result<Vec<TaskExecution>, ServiceError> {
        self.authorize(actor, profile_id)?;
        self.ensure_profile_exists(profile_id).await?;
        self.repository
            .list_executions(profile_id)
            .await
            .map_err(Self::storage_error)
    }

    pub async fn get_execution(
        &self,
        actor: Actor,
        execution_id: Uuid,
    ) -> Result<TaskExecution, ServiceError> {
        let execution = self
            .repository
            .get_execution(execution_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;
        self.authorize(actor, execution.profile_id)?;
        Ok(execution)
    }

    pub async fn delete_execution(
        &self,
        actor: Actor,
        execution_id: Uuid,
    ) -> Result<(), ServiceError> {
        let execution = self.get_execution(actor, execution_id).await?;
        self.repository
            .delete_execution(execution.id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;
        Ok(())
    }

    pub async fn list_balances(
        &self,
        actor: Actor,
        profile_id: Uuid,
    ) -> Result<Vec<ProfileBalance>, ServiceError> {
        self.authorize(actor, profile_id)?;
        self.ensure_profile_exists(profile_id).await?;
        self.repository
            .list_balances(profile_id)
            .await
            .map_err(Self::storage_error)
    }

    pub async fn list_levels(
        &self,
        actor: Actor,
        profile_id: Uuid,
    ) -> Result<Vec<Level>, ServiceError> {
        self.authorize(actor, profile_id)?;
        self.ensure_profile_exists(profile_id).await?;
        self.repository
            .list_levels(profile_id)
            .await
            .map_err(Self::storage_error)
    }

    pub async fn create_level(
        &self,
        actor: Actor,
        request: CreateLevelRequest,
    ) -> Result<Level, ServiceError> {
        self.authorize(actor, request.profile_id)?;
        self.ensure_profile_exists(request.profile_id).await?;
        self.validate_level_fields(
            &request.code,
            request.ordinal,
            request.target_planned_score,
            request.target_planned_rate,
        )?;

        let now = Utc::now();
        let level = Level {
            id: Uuid::new_v4(),
            profile_id: request.profile_id,
            code: request.code.trim().to_owned(),
            ordinal: request.ordinal,
            min_balance: request.min_balance,
            target_planned_score: request.target_planned_score,
            target_planned_rate: request.target_planned_rate,
            created_at: now,
            updated_at: now,
        };
        self.repository
            .create_level(level.clone())
            .await
            .map_err(Self::storage_error)?;
        self.repository
            .recalculate_level_state(level.profile_id)
            .await
            .map_err(Self::storage_error)?;
        Ok(level)
    }

    pub async fn update_level(
        &self,
        actor: Actor,
        level_id: Uuid,
        request: UpdateLevelRequest,
    ) -> Result<Level, ServiceError> {
        let mut level = self
            .repository
            .get_level(level_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;
        self.authorize(actor, level.profile_id)?;

        if let Some(code) = request.code {
            require_non_empty("code", &code)?;
            level.code = code.trim().to_owned();
        }
        if let Some(ordinal) = request.ordinal {
            if ordinal <= 0 {
                return Err(ServiceError::Validation(
                    "ordinal must be greater than zero".to_owned(),
                ));
            }
            level.ordinal = ordinal;
        }
        if let Some(min_balance) = request.min_balance {
            level.min_balance = min_balance;
        }
        if let Some(target_planned_score) = request.target_planned_score {
            validate_score("target_planned_score", target_planned_score)?;
            level.target_planned_score = target_planned_score;
        }
        if let Some(target_planned_rate) = request.target_planned_rate {
            validate_rate("target_planned_rate", target_planned_rate)?;
            level.target_planned_rate = target_planned_rate;
        }
        level.updated_at = Utc::now();

        self.repository
            .update_level(level.clone())
            .await
            .map_err(Self::storage_error)?;
        self.repository
            .recalculate_level_state(level.profile_id)
            .await
            .map_err(Self::storage_error)?;
        Ok(level)
    }

    pub async fn delete_level(&self, actor: Actor, level_id: Uuid) -> Result<(), ServiceError> {
        let level = self
            .repository
            .get_level(level_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;
        self.authorize(actor, level.profile_id)?;
        let levels = self.list_levels(actor, level.profile_id).await?;
        if levels.len() <= 1 {
            return Err(ServiceError::Conflict(
                "a profile must keep at least one level".to_owned(),
            ));
        }
        self.repository
            .delete_level(level_id)
            .await
            .map_err(Self::storage_error)?;
        self.repository
            .recalculate_level_state(level.profile_id)
            .await
            .map_err(Self::storage_error)?;
        Ok(())
    }

    pub async fn create_day_finalization(
        &self,
        actor: Actor,
        request: CreateDayFinalizationRequest,
    ) -> Result<DayFinalization, ServiceError> {
        self.authorize(actor, request.profile_id)?;
        self.ensure_profile_exists(request.profile_id).await?;
        let finalization = DayFinalization {
            id: Uuid::new_v4(),
            profile_id: request.profile_id,
            date: request.date,
            note: request.note.and_then(|note| {
                let trimmed = note.trim().to_owned();
                (!trimmed.is_empty()).then_some(trimmed)
            }),
            created_at: Utc::now(),
        };
        self.repository
            .create_day_finalization(finalization.clone())
            .await
            .map_err(Self::storage_error)?;
        Ok(finalization)
    }

    pub async fn list_day_finalizations(
        &self,
        actor: Actor,
        profile_id: Uuid,
    ) -> Result<Vec<DayFinalization>, ServiceError> {
        self.authorize(actor, profile_id)?;
        self.ensure_profile_exists(profile_id).await?;
        self.repository
            .list_day_finalizations(profile_id)
            .await
            .map_err(Self::storage_error)
    }

    pub async fn delete_day_finalization(
        &self,
        actor: Actor,
        finalization_id: Uuid,
    ) -> Result<(), ServiceError> {
        let finalization = self
            .repository
            .list_day_finalizations(actor.profile_id)
            .await
            .map_err(Self::storage_error)?
            .into_iter()
            .find(|item| item.id == finalization_id)
            .ok_or(ServiceError::NotFound)?;
        self.authorize(actor, finalization.profile_id)?;
        self.repository
            .delete_day_finalization(finalization_id)
            .await
            .map_err(Self::storage_error)?;
        Ok(())
    }

    pub async fn get_level_state(
        &self,
        profile_id: Uuid,
    ) -> Result<Option<ProfileLevelState>, ServiceError> {
        self.ensure_profile_exists(profile_id).await?;
        let state = self
            .repository
            .get_level_state(profile_id)
            .await
            .map_err(Self::storage_error)?;
        if state.is_some() {
            return Ok(state);
        }
        self.repository
            .recalculate_level_state(profile_id)
            .await
            .map_err(Self::storage_error)
    }

    pub async fn dashboard(
        &self,
        actor: Actor,
        profile_id: Uuid,
    ) -> Result<Dashboard, ServiceError> {
        self.authorize(actor, profile_id)?;
        let profile = self
            .repository
            .get_profile(profile_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;
        let tasks = self
            .repository
            .list_tasks(profile_id)
            .await
            .map_err(Self::storage_error)?;
        let recent_executions = self
            .repository
            .list_executions(profile_id)
            .await
            .map_err(Self::storage_error)?;
        let balances = self
            .repository
            .list_balances(profile_id)
            .await
            .map_err(Self::storage_error)?;
        let levels = self
            .repository
            .list_levels(profile_id)
            .await
            .map_err(Self::storage_error)?;
        let finalizations = self
            .repository
            .list_day_finalizations(profile_id)
            .await
            .map_err(Self::storage_error)?;
        let level_state = self
            .repository
            .get_level_state(profile_id)
            .await
            .map_err(Self::storage_error)?
            .or(self
                .repository
                .recalculate_level_state(profile_id)
                .await
                .map_err(Self::storage_error)?);
        let balance_score = self.engine.balance_from_history(&balances);
        let current_level = resolve_current_level(level_state.as_ref(), &levels)
            .ok_or_else(|| ServiceError::Conflict("profile has no levels configured".to_owned()))?;
        let current_photo = match profile.current_photo_id {
            Some(photo_id) => self
                .repository
                .get_photo(photo_id)
                .await
                .map_err(Self::storage_error)?
                .map(|photo| self.photo_summary(&photo)),
            None => None,
        };

        Ok(Dashboard {
            profile,
            current_photo,
            current: ProgressionSummary {
                balance_score,
                current_level: current_level.code.clone(),
                current_level_id: current_level.id,
            },
            tasks: tasks.clone(),
            execution_queue: tasks,
            recent_executions,
            balances,
            levels,
            finalizations,
        })
    }

    fn authorize(&self, actor: Actor, profile_id: Uuid) -> Result<(), ServiceError> {
        if actor.profile_id != profile_id {
            return Err(ServiceError::Forbidden);
        }
        Ok(())
    }

    async fn ensure_profile_exists(&self, profile_id: Uuid) -> Result<(), ServiceError> {
        self.repository
            .get_profile(profile_id)
            .await
            .map_err(Self::storage_error)?
            .ok_or(ServiceError::NotFound)?;
        Ok(())
    }

    async fn ensure_sphere_exists(&self, sphere_id: Option<Uuid>) -> Result<(), ServiceError> {
        if let Some(sphere_id) = sphere_id {
            self.repository
                .get_sphere(sphere_id)
                .await
                .map_err(Self::storage_error)?
                .ok_or_else(|| ServiceError::Validation("sphere_id does not exist".to_owned()))?;
        }
        Ok(())
    }

    fn validate_task_fields(
        &self,
        title: &str,
        planned_weight: i32,
        planned_score: i32,
        planned_rate: i32,
    ) -> Result<(), ServiceError> {
        require_non_empty("title", title)?;
        if planned_weight <= 0 {
            return Err(ServiceError::Validation(
                "planned_weight must be greater than zero".to_owned(),
            ));
        }
        validate_score("planned_score", planned_score)?;
        validate_rate("planned_rate", planned_rate)?;
        Ok(())
    }

    fn validate_level_fields(
        &self,
        code: &str,
        ordinal: i32,
        target_planned_score: i32,
        target_planned_rate: i32,
    ) -> Result<(), ServiceError> {
        require_non_empty("code", code)?;
        if ordinal <= 0 {
            return Err(ServiceError::Validation(
                "ordinal must be greater than zero".to_owned(),
            ));
        }
        validate_score("target_planned_score", target_planned_score)?;
        validate_rate("target_planned_rate", target_planned_rate)?;
        Ok(())
    }

    fn photo_summary(&self, photo: &ProfilePhoto) -> ProfilePhotoSummary {
        ProfilePhotoSummary {
            id: photo.id,
            original_name: photo.original_name.clone(),
            mime_type: photo.mime_type.clone(),
            size_bytes: photo.size_bytes,
            content_url: format!("/api/v2/photos/{}", photo.id),
        }
    }

    fn storage_error(error: RepositoryError) -> ServiceError {
        match error {
            RepositoryError::Conflict(message) => ServiceError::Conflict(message),
            RepositoryError::Operation(message) => ServiceError::Storage(message),
        }
    }
}

fn resolve_current_level<'a>(
    level_state: Option<&ProfileLevelState>,
    levels: &'a [Level],
) -> Option<&'a Level> {
    level_state
        .and_then(|state| {
            levels
                .iter()
                .find(|level| level.id == state.current_level_id)
        })
        .or_else(|| levels.iter().min_by_key(|level| level.ordinal))
}

fn validate_score(field: &str, value: i32) -> Result<(), ServiceError> {
    if !(1..=5).contains(&value) {
        return Err(ServiceError::Validation(format!(
            "{field} must be between 1 and 5"
        )));
    }
    Ok(())
}

fn validate_rate(field: &str, value: i32) -> Result<(), ServiceError> {
    if !(0..=100).contains(&value) {
        return Err(ServiceError::Validation(format!(
            "{field} must be between 0 and 100"
        )));
    }
    Ok(())
}

fn require_non_empty(field: &str, value: &str) -> Result<(), ServiceError> {
    if value.trim().is_empty() {
        return Err(ServiceError::Validation(format!("{field} is required")));
    }
    Ok(())
}

fn sanitize_file_name(name: &str) -> String {
    let fallback = "upload.bin";
    let candidate = if name.trim().is_empty() {
        fallback
    } else {
        name
    };
    candidate
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn compute_period_bounds(
    starts_on: NaiveDate,
    completed_on: NaiveDate,
    cadence: TaskCadence,
) -> (NaiveDate, NaiveDate) {
    if completed_on <= starts_on {
        return match cadence {
            TaskCadence::Day => (starts_on, starts_on),
            TaskCadence::Week => (starts_on, starts_on + Days::new(6)),
            TaskCadence::Month => (starts_on, starts_on + Months::new(1) - Days::new(1)),
            TaskCadence::Year => (starts_on, starts_on + Months::new(12) - Days::new(1)),
        };
    }

    match cadence {
        TaskCadence::Day => (completed_on, completed_on),
        TaskCadence::Week => {
            let days = completed_on
                .signed_duration_since(starts_on)
                .num_days()
                .max(0) as u64;
            let offset_weeks = days / 7;
            let period_start = starts_on + Days::new(offset_weeks * 7);
            (period_start, period_start + Days::new(6))
        }
        TaskCadence::Month => {
            let mut period_start = starts_on;
            loop {
                let next_start = period_start + Months::new(1);
                if next_start > completed_on {
                    return (period_start, next_start - Days::new(1));
                }
                period_start = next_start;
            }
        }
        TaskCadence::Year => {
            let mut period_start = starts_on;
            loop {
                let next_start = period_start + Months::new(12);
                if next_start > completed_on {
                    return (period_start, next_start - Days::new(1));
                }
                period_start = next_start;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveDate;

    use super::{
        Actor, CreateProfileRequest, CreateTaskExecutionRequest, CreateTaskRequest, PhotoUpload,
        ProgressionService,
    };
    use crate::{
        domain::{TaskCadence, TaskKind},
        infrastructure::SqliteRepository,
    };

    #[tokio::test]
    async fn create_execution_updates_dashboard_balance() {
        let database = tempfile::NamedTempFile::new().unwrap();
        let upload_dir = tempfile::tempdir().unwrap();
        let repository = Arc::new(SqliteRepository::new(database.path()).unwrap());
        let service = ProgressionService::new(repository, upload_dir.path().to_path_buf());

        let (profile, _) = service
            .create_profile(CreateProfileRequest {
                full_name: "Test User".into(),
                birth_date: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                occupation: "Builder".into(),
                telegram: None,
                email: None,
                timezone: "Europe/Samara".into(),
            })
            .await
            .unwrap();
        let actor = Actor {
            profile_id: profile.id,
        };
        let task = service
            .create_task(
                actor,
                CreateTaskRequest {
                    profile_id: profile.id,
                    title: "Run".into(),
                    sphere_id: None,
                    kind: TaskKind::Positive,
                    planned_weight: 2,
                    planned_score: 4,
                    planned_rate: 90,
                    cadence: TaskCadence::Day,
                    starts_on: NaiveDate::from_ymd_opt(2026, 7, 2).unwrap(),
                },
            )
            .await
            .unwrap();

        service
            .create_execution(
                actor,
                task.id,
                CreateTaskExecutionRequest {
                    actual_score: 5,
                    actual_rate: 95,
                    completed_at: None,
                },
            )
            .await
            .unwrap();

        let dashboard = service.dashboard(actor, profile.id).await.unwrap();
        assert_eq!(dashboard.current.balance_score, 2);
    }

    #[tokio::test]
    async fn upload_photo_creates_summary_url() {
        let database = tempfile::NamedTempFile::new().unwrap();
        let upload_dir = tempfile::tempdir().unwrap();
        let repository = Arc::new(SqliteRepository::new(database.path()).unwrap());
        let service = ProgressionService::new(repository, upload_dir.path().to_path_buf());

        let (profile, _) = service
            .create_profile(CreateProfileRequest {
                full_name: "Test User".into(),
                birth_date: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                occupation: "Builder".into(),
                telegram: None,
                email: None,
                timezone: "Europe/Samara".into(),
            })
            .await
            .unwrap();
        let actor = Actor {
            profile_id: profile.id,
        };

        let photo = service
            .upload_photo(
                actor,
                profile.id,
                PhotoUpload {
                    original_name: "avatar.png".into(),
                    mime_type: "image/png".into(),
                    bytes: vec![1, 2, 3],
                },
            )
            .await
            .unwrap();

        assert!(photo.content_url.contains("/api/v2/photos/"));
    }
}
