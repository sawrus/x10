use axum::{
    Json, Router,
    body::Body,
    extract::{Multipart, Path, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Redirect, Response},
    routing::{delete, get, patch, post},
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::{ApiError, AppState, actor_from_headers},
    application::{
        CreateDayFinalizationRequest, CreateLevelRequest, CreateProfileRequest,
        CreateSphereRequest, CreateTaskExecutionRequest, CreateTaskRequest, PhotoUpload,
        UpdateLevelRequest, UpdateProfileRequest, UpdateSphereRequest, UpdateTaskRequest,
    },
    domain::{
        Dashboard, DayFinalization, Level, Profile, ProfileBalance, ProfilePhotoSummary, Sphere,
        Task, TaskCadence, TaskExecution, TaskKind, TaskStatus,
    },
};

pub fn build_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/metrics", get(metrics))
        .route("/api/v2/spheres", get(list_spheres).post(create_sphere))
        .route(
            "/api/v2/spheres/{sphere_id}",
            get(get_sphere).patch(update_sphere).delete(delete_sphere),
        )
        .route("/api/v2/profiles", get(list_profiles).post(create_profile))
        .route(
            "/api/v2/profiles/{profile_id}",
            get(get_profile).patch(update_profile),
        )
        .route(
            "/api/v2/profiles/{profile_id}/photos",
            post(upload_photo).get(list_photos),
        )
        .route(
            "/api/v2/photos/{photo_id}",
            get(get_photo).delete(delete_photo),
        )
        .route(
            "/api/v2/profiles/{profile_id}/photos/{photo_id}/select",
            post(select_photo),
        )
        .route("/api/v2/tasks", post(create_task))
        .route("/api/v2/profiles/{profile_id}/tasks", get(list_tasks))
        .route(
            "/api/v2/tasks/{task_id}",
            get(get_task).patch(update_task).delete(delete_task),
        )
        .route("/api/v2/tasks/{task_id}/executions", post(create_execution))
        .route(
            "/api/v2/profiles/{profile_id}/executions",
            get(list_executions),
        )
        .route(
            "/api/v2/executions/{execution_id}",
            get(get_execution).delete(delete_execution),
        )
        .route("/api/v2/profiles/{profile_id}/balances", get(list_balances))
        .route(
            "/api/v2/profiles/{profile_id}/dashboard",
            get(get_dashboard),
        )
        .route(
            "/api/v2/profiles/{profile_id}/levels",
            get(list_levels).post(create_level),
        )
        .route(
            "/api/v2/levels/{level_id}",
            patch(update_level).delete(delete_level),
        )
        .route(
            "/api/v2/profiles/{profile_id}/days/{date}/finalize",
            post(finalize_day),
        )
        .route(
            "/api/v2/profiles/{profile_id}/days/finalizations",
            get(list_day_finalizations),
        )
        .route(
            "/api/v2/day-finalizations/{finalization_id}",
            delete(delete_day_finalization),
        )
        .route("/app", get(app_redirect))
        .route("/app/", get(app_redirect))
        .route("/game", get(game_index))
        .route("/game/", get(game_index))
        .route("/game/{*path}", get(game_asset))
        .route("/", get(web_index))
        .route("/{*path}", get(web_asset))
}

#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, description = "Service health", body = serde_json::Value))
)]
pub(crate) async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

#[utoipa::path(
    get,
    path = "/metrics",
    responses(
        (status = 200, description = "Prometheus metrics payload", body = String),
        (status = 503, description = "Metrics are disabled", body = String)
    )
)]
pub(crate) async fn metrics(State(state): State<AppState>) -> (StatusCode, String) {
    match state.metrics {
        Some(ref handle) => (StatusCode::OK, handle.render()),
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            "# metrics disabled\n".to_owned(),
        ),
    }
}

async fn app_redirect() -> Redirect {
    Redirect::permanent("/game")
}

async fn web_index(State(state): State<AppState>) -> Result<Response, ApiError> {
    serve_web_file(state, "index.html").await
}

async fn web_asset(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<Response, ApiError> {
    serve_web_file(state, &path).await
}

async fn game_index(State(state): State<AppState>) -> Result<Response, ApiError> {
    serve_game_file(state, "game/index.html").await
}

async fn game_asset(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<Response, ApiError> {
    let requested = if path.is_empty() {
        "game/index.html".to_owned()
    } else {
        format!("game/{path}")
    };
    serve_game_file(state, &requested).await
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProfilePayload {
    full_name: String,
    birth_date: NaiveDate,
    occupation: String,
    telegram: Option<String>,
    email: Option<String>,
    timezone: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProfilePayload {
    full_name: Option<String>,
    birth_date: Option<NaiveDate>,
    occupation: Option<String>,
    telegram: Option<Option<String>>,
    email: Option<Option<String>>,
    timezone: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/v2/profiles",
    responses((status = 200, description = "Profiles list", body = [Profile]))
)]
pub(crate) async fn list_profiles(
    State(state): State<AppState>,
) -> Result<Json<Vec<Profile>>, ApiError> {
    Ok(Json(state.service.list_profiles().await?))
}

#[utoipa::path(
    post,
    path = "/api/v2/profiles",
    request_body = CreateProfilePayload,
    responses(
        (status = 201, description = "Profile created", body = Profile),
        (status = 400, description = "Validation error", body = crate::api::error::ErrorEnvelope)
    )
)]
pub(crate) async fn create_profile(
    State(state): State<AppState>,
    Json(payload): Json<CreateProfilePayload>,
) -> Result<(StatusCode, Json<Profile>), ApiError> {
    let (profile, _) = state
        .service
        .create_profile(CreateProfileRequest {
            full_name: payload.full_name,
            birth_date: payload.birth_date,
            occupation: payload.occupation,
            telegram: payload.telegram,
            email: payload.email,
            timezone: payload.timezone,
        })
        .await?;
    Ok((StatusCode::CREATED, Json(profile)))
}

#[utoipa::path(
    get,
    path = "/api/v2/profiles/{profile_id}",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses(
        (status = 200, description = "Profile", body = Profile),
        (status = 403, description = "Access denied", body = crate::api::error::ErrorEnvelope),
        (status = 404, description = "Profile not found", body = crate::api::error::ErrorEnvelope)
    )
)]
pub(crate) async fn get_profile(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Profile>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(state.service.get_profile(actor, profile_id).await?))
}

#[utoipa::path(
    patch,
    path = "/api/v2/profiles/{profile_id}",
    request_body = UpdateProfilePayload,
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses(
        (status = 200, description = "Updated profile", body = Profile),
        (status = 400, description = "Validation error", body = crate::api::error::ErrorEnvelope),
        (status = 403, description = "Access denied", body = crate::api::error::ErrorEnvelope)
    )
)]
pub(crate) async fn update_profile(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<UpdateProfilePayload>,
) -> Result<Json<Profile>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(
        state
            .service
            .update_profile(
                actor,
                profile_id,
                UpdateProfileRequest {
                    full_name: payload.full_name,
                    birth_date: payload.birth_date,
                    occupation: payload.occupation,
                    telegram: payload.telegram,
                    email: payload.email,
                    timezone: payload.timezone,
                },
            )
            .await?,
    ))
}

#[utoipa::path(
    post,
    path = "/api/v2/profiles/{profile_id}/photos",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses(
        (status = 201, description = "Uploaded photo", body = ProfilePhotoSummary),
        (status = 400, description = "Validation error", body = crate::api::error::ErrorEnvelope)
    )
)]
pub(crate) async fn upload_photo(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<ProfilePhotoSummary>), ApiError> {
    let actor = actor_from_headers(&headers)?;
    let mut upload = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|error| ApiError::bad_request("INVALID_MULTIPART", error.to_string()))?
    {
        if field.name() != Some("file") {
            continue;
        }
        let original_name = field
            .file_name()
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| "upload.bin".to_owned());
        let mime_type = field
            .content_type()
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| "application/octet-stream".to_owned());
        let bytes = field
            .bytes()
            .await
            .map_err(|error| ApiError::bad_request("INVALID_MULTIPART", error.to_string()))?;
        upload = Some(PhotoUpload {
            original_name,
            mime_type,
            bytes: bytes.to_vec(),
        });
    }

    let photo = state
        .service
        .upload_photo(
            actor,
            profile_id,
            upload
                .ok_or_else(|| ApiError::bad_request("FILE_REQUIRED", "file field is required"))?,
        )
        .await?;

    Ok((StatusCode::CREATED, Json(photo)))
}

#[utoipa::path(
    get,
    path = "/api/v2/profiles/{profile_id}/photos",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Photos", body = [ProfilePhotoSummary]))
)]
pub(crate) async fn list_photos(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Vec<ProfilePhotoSummary>>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(state.service.list_photos(actor, profile_id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v2/photos/{photo_id}",
    params(
        ("photo_id" = Uuid, Path, description = "Photo identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Photo bytes", body = String))
)]
pub(crate) async fn get_photo(
    State(state): State<AppState>,
    Path(photo_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let actor = actor_from_headers(&headers)?;
    let (photo, bytes) = state.service.get_photo_file(actor, photo_id).await?;

    Ok((
        StatusCode::OK,
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_str(&photo.mime_type)
                .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
        )],
        Body::from(bytes),
    )
        .into_response())
}

#[utoipa::path(
    delete,
    path = "/api/v2/photos/{photo_id}",
    params(
        ("photo_id" = Uuid, Path, description = "Photo identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 204, description = "Photo deleted"))
)]
pub(crate) async fn delete_photo(
    State(state): State<AppState>,
    Path(photo_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let actor = actor_from_headers(&headers)?;
    state.service.delete_photo(actor, photo_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/api/v2/profiles/{profile_id}/photos/{photo_id}/select",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("photo_id" = Uuid, Path, description = "Photo identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Updated profile", body = Profile))
)]
pub(crate) async fn select_photo(
    State(state): State<AppState>,
    Path((profile_id, photo_id)): Path<(Uuid, Uuid)>,
    headers: HeaderMap,
) -> Result<Json<Profile>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(
        state
            .service
            .select_photo(actor, profile_id, photo_id)
            .await?,
    ))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSpherePayload {
    name: String,
    weight: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSpherePayload {
    name: Option<String>,
    weight: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/api/v2/spheres",
    request_body = CreateSpherePayload,
    responses((status = 201, description = "Sphere created", body = Sphere))
)]
pub(crate) async fn create_sphere(
    State(state): State<AppState>,
    Json(payload): Json<CreateSpherePayload>,
) -> Result<(StatusCode, Json<Sphere>), ApiError> {
    let sphere = state
        .service
        .create_sphere(CreateSphereRequest {
            name: payload.name,
            weight: payload.weight,
        })
        .await?;
    Ok((StatusCode::CREATED, Json(sphere)))
}

#[utoipa::path(
    get,
    path = "/api/v2/spheres",
    responses((status = 200, description = "Spheres", body = [Sphere]))
)]
pub(crate) async fn list_spheres(
    State(state): State<AppState>,
) -> Result<Json<Vec<Sphere>>, ApiError> {
    Ok(Json(state.service.list_spheres().await?))
}

#[utoipa::path(
    get,
    path = "/api/v2/spheres/{sphere_id}",
    params(("sphere_id" = Uuid, Path, description = "Sphere identifier")),
    responses((status = 200, description = "Sphere", body = Sphere))
)]
pub(crate) async fn get_sphere(
    State(state): State<AppState>,
    Path(sphere_id): Path<Uuid>,
) -> Result<Json<Sphere>, ApiError> {
    Ok(Json(state.service.get_sphere(sphere_id).await?))
}

#[utoipa::path(
    patch,
    path = "/api/v2/spheres/{sphere_id}",
    request_body = UpdateSpherePayload,
    params(("sphere_id" = Uuid, Path, description = "Sphere identifier")),
    responses((status = 200, description = "Updated sphere", body = Sphere))
)]
pub(crate) async fn update_sphere(
    State(state): State<AppState>,
    Path(sphere_id): Path<Uuid>,
    Json(payload): Json<UpdateSpherePayload>,
) -> Result<Json<Sphere>, ApiError> {
    Ok(Json(
        state
            .service
            .update_sphere(
                sphere_id,
                UpdateSphereRequest {
                    name: payload.name,
                    weight: payload.weight,
                },
            )
            .await?,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v2/spheres/{sphere_id}",
    params(("sphere_id" = Uuid, Path, description = "Sphere identifier")),
    responses((status = 204, description = "Sphere deleted"))
)]
pub(crate) async fn delete_sphere(
    State(state): State<AppState>,
    Path(sphere_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    state.service.delete_sphere(sphere_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTaskPayload {
    profile_id: Uuid,
    title: String,
    sphere_id: Option<Uuid>,
    kind: TaskKind,
    planned_weight: i32,
    planned_score: i32,
    planned_rate: i32,
    cadence: TaskCadence,
    starts_on: NaiveDate,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTaskPayload {
    title: Option<String>,
    sphere_id: Option<Option<Uuid>>,
    kind: Option<TaskKind>,
    planned_weight: Option<i32>,
    planned_score: Option<i32>,
    planned_rate: Option<i32>,
    cadence: Option<TaskCadence>,
    starts_on: Option<NaiveDate>,
    status: Option<TaskStatus>,
}

#[utoipa::path(
    post,
    path = "/api/v2/tasks",
    request_body = CreateTaskPayload,
    params(("X-Actor-Id" = String, Header, description = "Profile id used for authorization")),
    responses((status = 201, description = "Task created", body = Task))
)]
pub(crate) async fn create_task(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateTaskPayload>,
) -> Result<(StatusCode, Json<Task>), ApiError> {
    let actor = actor_from_headers(&headers)?;
    let task = state
        .service
        .create_task(
            actor,
            CreateTaskRequest {
                profile_id: payload.profile_id,
                title: payload.title,
                sphere_id: payload.sphere_id,
                kind: payload.kind,
                planned_weight: payload.planned_weight,
                planned_score: payload.planned_score,
                planned_rate: payload.planned_rate,
                cadence: payload.cadence,
                starts_on: payload.starts_on,
            },
        )
        .await?;
    Ok((StatusCode::CREATED, Json(task)))
}

#[utoipa::path(
    get,
    path = "/api/v2/profiles/{profile_id}/tasks",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Tasks", body = [Task]))
)]
pub(crate) async fn list_tasks(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Vec<Task>>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(state.service.list_tasks(actor, profile_id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v2/tasks/{task_id}",
    params(
        ("task_id" = Uuid, Path, description = "Task identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Task", body = Task))
)]
pub(crate) async fn get_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Task>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(state.service.get_task(actor, task_id).await?))
}

#[utoipa::path(
    patch,
    path = "/api/v2/tasks/{task_id}",
    request_body = UpdateTaskPayload,
    params(
        ("task_id" = Uuid, Path, description = "Task identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Updated task", body = Task))
)]
pub(crate) async fn update_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<UpdateTaskPayload>,
) -> Result<Json<Task>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(
        state
            .service
            .update_task(
                actor,
                task_id,
                UpdateTaskRequest {
                    title: payload.title,
                    sphere_id: payload.sphere_id,
                    kind: payload.kind,
                    planned_weight: payload.planned_weight,
                    planned_score: payload.planned_score,
                    planned_rate: payload.planned_rate,
                    cadence: payload.cadence,
                    starts_on: payload.starts_on,
                    status: payload.status,
                },
            )
            .await?,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v2/tasks/{task_id}",
    params(
        ("task_id" = Uuid, Path, description = "Task identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 204, description = "Task deleted"))
)]
pub(crate) async fn delete_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let actor = actor_from_headers(&headers)?;
    state.service.delete_task(actor, task_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTaskExecutionPayload {
    actual_score: i32,
    actual_rate: i32,
    completed_at: Option<DateTime<Utc>>,
}

#[utoipa::path(
    post,
    path = "/api/v2/tasks/{task_id}/executions",
    request_body = CreateTaskExecutionPayload,
    params(
        ("task_id" = Uuid, Path, description = "Task identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 201, description = "Execution created", body = TaskExecution))
)]
pub(crate) async fn create_execution(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<CreateTaskExecutionPayload>,
) -> Result<(StatusCode, Json<TaskExecution>), ApiError> {
    let actor = actor_from_headers(&headers)?;
    let execution = state
        .service
        .create_execution(
            actor,
            task_id,
            CreateTaskExecutionRequest {
                actual_score: payload.actual_score,
                actual_rate: payload.actual_rate,
                completed_at: payload.completed_at,
            },
        )
        .await?;
    Ok((StatusCode::CREATED, Json(execution)))
}

#[utoipa::path(
    get,
    path = "/api/v2/profiles/{profile_id}/executions",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Executions", body = [TaskExecution]))
)]
pub(crate) async fn list_executions(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Vec<TaskExecution>>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(
        state.service.list_executions(actor, profile_id).await?,
    ))
}

#[utoipa::path(
    get,
    path = "/api/v2/executions/{execution_id}",
    params(
        ("execution_id" = Uuid, Path, description = "Execution identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Execution", body = TaskExecution))
)]
pub(crate) async fn get_execution(
    State(state): State<AppState>,
    Path(execution_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<TaskExecution>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(
        state.service.get_execution(actor, execution_id).await?,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v2/executions/{execution_id}",
    params(
        ("execution_id" = Uuid, Path, description = "Execution identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 204, description = "Execution deleted"))
)]
pub(crate) async fn delete_execution(
    State(state): State<AppState>,
    Path(execution_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let actor = actor_from_headers(&headers)?;
    state.service.delete_execution(actor, execution_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/v2/profiles/{profile_id}/balances",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Balance history", body = [ProfileBalance]))
)]
pub(crate) async fn list_balances(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Vec<ProfileBalance>>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(state.service.list_balances(actor, profile_id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v2/profiles/{profile_id}/dashboard",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Dashboard", body = Dashboard))
)]
pub(crate) async fn get_dashboard(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Dashboard>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(state.service.dashboard(actor, profile_id).await?))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateLevelPayload {
    code: String,
    ordinal: i32,
    min_balance: i32,
    target_planned_score: i32,
    target_planned_rate: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateLevelPayload {
    code: Option<String>,
    ordinal: Option<i32>,
    min_balance: Option<i32>,
    target_planned_score: Option<i32>,
    target_planned_rate: Option<i32>,
}

#[utoipa::path(
    get,
    path = "/api/v2/profiles/{profile_id}/levels",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Levels", body = [Level]))
)]
pub(crate) async fn list_levels(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Vec<Level>>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(state.service.list_levels(actor, profile_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v2/profiles/{profile_id}/levels",
    request_body = CreateLevelPayload,
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 201, description = "Level created", body = Level))
)]
pub(crate) async fn create_level(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<CreateLevelPayload>,
) -> Result<(StatusCode, Json<Level>), ApiError> {
    let actor = actor_from_headers(&headers)?;
    let level = state
        .service
        .create_level(
            actor,
            CreateLevelRequest {
                profile_id,
                code: payload.code,
                ordinal: payload.ordinal,
                min_balance: payload.min_balance,
                target_planned_score: payload.target_planned_score,
                target_planned_rate: payload.target_planned_rate,
            },
        )
        .await?;
    Ok((StatusCode::CREATED, Json(level)))
}

#[utoipa::path(
    patch,
    path = "/api/v2/levels/{level_id}",
    request_body = UpdateLevelPayload,
    params(
        ("level_id" = Uuid, Path, description = "Level identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Level updated", body = Level))
)]
pub(crate) async fn update_level(
    State(state): State<AppState>,
    Path(level_id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<UpdateLevelPayload>,
) -> Result<Json<Level>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(
        state
            .service
            .update_level(
                actor,
                level_id,
                UpdateLevelRequest {
                    code: payload.code,
                    ordinal: payload.ordinal,
                    min_balance: payload.min_balance,
                    target_planned_score: payload.target_planned_score,
                    target_planned_rate: payload.target_planned_rate,
                },
            )
            .await?,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v2/levels/{level_id}",
    params(
        ("level_id" = Uuid, Path, description = "Level identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 204, description = "Level deleted"))
)]
pub(crate) async fn delete_level(
    State(state): State<AppState>,
    Path(level_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let actor = actor_from_headers(&headers)?;
    state.service.delete_level(actor, level_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDayFinalizationPayload {
    note: Option<String>,
}

#[utoipa::path(
    post,
    path = "/api/v2/profiles/{profile_id}/days/{date}/finalize",
    request_body = Option<CreateDayFinalizationPayload>,
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("date" = NaiveDate, Path, description = "Finalization day"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 201, description = "Day finalization", body = DayFinalization))
)]
pub(crate) async fn finalize_day(
    State(state): State<AppState>,
    Path((profile_id, date)): Path<(Uuid, NaiveDate)>,
    headers: HeaderMap,
    body: Option<Json<CreateDayFinalizationPayload>>,
) -> Result<(StatusCode, Json<DayFinalization>), ApiError> {
    let actor = actor_from_headers(&headers)?;
    let finalization = state
        .service
        .create_day_finalization(
            actor,
            CreateDayFinalizationRequest {
                profile_id,
                date,
                note: body.and_then(|payload| payload.note.clone()),
            },
        )
        .await?;
    Ok((StatusCode::CREATED, Json(finalization)))
}

#[utoipa::path(
    get,
    path = "/api/v2/profiles/{profile_id}/days/finalizations",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses((status = 200, description = "Day finalizations", body = [DayFinalization]))
)]
pub(crate) async fn list_day_finalizations(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Vec<DayFinalization>>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    Ok(Json(
        state
            .service
            .list_day_finalizations(actor, profile_id)
            .await?,
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v2/day-finalizations/{finalization_id}",
    params(
        ("finalization_id" = Uuid, Path, description = "Finalization identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for authorization")
    ),
    responses(
        (status = 204, description = "Day finalization deleted"),
        (status = 403, description = "Access denied", body = crate::api::error::ErrorEnvelope),
        (status = 404, description = "Day finalization not found", body = crate::api::error::ErrorEnvelope)
    )
)]
pub(crate) async fn delete_day_finalization(
    State(state): State<AppState>,
    Path(finalization_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let actor = actor_from_headers(&headers)?;
    state
        .service
        .delete_day_finalization(actor, finalization_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn serve_web_file(state: AppState, requested_path: &str) -> Result<Response, ApiError> {
    let normalized = if requested_path.is_empty() {
        "index.html"
    } else {
        requested_path
    };
    let path = state.web_dist_path.join(normalized);
    let fallback = state.web_dist_path.join("index.html");
    let target = if path.is_file() { path } else { fallback };
    let bytes = tokio::fs::read(&target)
        .await
        .map_err(|error| ApiError::bad_request("WEB_ASSET_NOT_FOUND", error.to_string()))?;
    let content_type = content_type_for_path(&target);
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, HeaderValue::from_static(content_type))],
        Body::from(bytes),
    )
        .into_response())
}

async fn serve_game_file(state: AppState, requested_path: &str) -> Result<Response, ApiError> {
    let path = state.web_dist_path.join(requested_path);
    let fallback = state.web_dist_path.join("game/index.html");
    let target = if path.is_file() { path } else { fallback };
    let bytes = tokio::fs::read(&target)
        .await
        .map_err(|error| ApiError::bad_request("WEB_ASSET_NOT_FOUND", error.to_string()))?;
    let content_type = content_type_for_path(&target);
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, HeaderValue::from_static(content_type))],
        Body::from(bytes),
    )
        .into_response())
}

fn content_type_for_path(path: &std::path::Path) -> &'static str {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
    {
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "woff2" => "font/woff2",
        "map" => "application/json; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        _ => "text/html; charset=utf-8",
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use chrono::NaiveDate;
    use tower::ServiceExt;

    use crate::{
        api::{AppState, build_router},
        application::{CreateProfileRequest, ProgressionService},
        infrastructure::SqliteRepository,
    };

    async fn build_test_state() -> (AppState, tempfile::TempDir) {
        let upload_dir = tempfile::tempdir().unwrap();
        let web_dir = tempfile::tempdir().unwrap();
        let database_path = web_dir.path().join("x10.sqlite3");
        tokio::fs::create_dir_all(web_dir.path().join("game"))
            .await
            .unwrap();
        tokio::fs::write(web_dir.path().join("index.html"), "<html>admin</html>")
            .await
            .unwrap();
        tokio::fs::write(web_dir.path().join("game/index.html"), "<html>game</html>")
            .await
            .unwrap();
        let repository = Arc::new(SqliteRepository::new(&database_path).unwrap());
        let service = Arc::new(ProgressionService::new(
            repository,
            upload_dir.path().to_path_buf(),
        ));
        (
            AppState::new(
                service,
                None,
                web_dir.path().to_path_buf(),
                "admin".to_owned(),
                "unused".to_owned(),
                "secret".to_owned(),
                false,
            ),
            web_dir,
        )
    }

    #[tokio::test]
    async fn protected_profile_read_requires_matching_actor() {
        let (state, _web_dir) = build_test_state().await;
        let app = build_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v2/profiles/00000000-0000-0000-0000-000000000001")
                    .header("x-actor-id", "00000000-0000-0000-0000-000000000002")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn game_api_does_not_require_admin_session() {
        let (state, _web_dir) = build_test_state().await;
        let app = build_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v2/profiles/00000000-0000-0000-0000-000000000001")
                    .header("x-actor-id", "00000000-0000-0000-0000-000000000001")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn game_profiles_list_is_available_without_admin_session() {
        let (state, _web_dir) = build_test_state().await;
        state
            .service
            .create_profile(CreateProfileRequest {
                full_name: "Test Hero".to_owned(),
                birth_date: NaiveDate::from_ymd_opt(1989, 6, 27).unwrap(),
                occupation: "programmer".to_owned(),
                telegram: None,
                email: None,
                timezone: "Europe/Samara".to_owned(),
            })
            .await
            .unwrap();
        let app = build_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v2/profiles")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn docs_ui_and_web_routes_are_available() {
        let (state, _web_dir) = build_test_state().await;
        let app = build_router(state);

        let docs_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/docs/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(docs_response.status(), StatusCode::OK);

        let web_response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(web_response.status(), StatusCode::OK);
    }
}
