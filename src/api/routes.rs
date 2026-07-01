use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use chrono::NaiveDate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::{ApiError, AppState, actor_from_headers},
    application::{
        CompleteTaskRequest, CreateProfileRequest, CreateSphereRequest, CreateTaskRequest,
        FinalizeDayRequest,
    },
    domain::{DailySnapshot, Dashboard, Profile, Sphere, Task, TaskCadence, TaskKind},
};

pub fn build_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/metrics", get(metrics))
        .route("/api/v1/spheres", get(list_spheres).post(create_sphere))
        .route("/api/v1/profiles", post(create_profile))
        .route("/api/v1/profiles/{profile_id}", get(get_profile))
        .route(
            "/api/v1/profiles/{profile_id}/dashboard",
            get(get_dashboard),
        )
        .route(
            "/api/v1/profiles/{profile_id}/days/{date}/finalize",
            post(finalize_day),
        )
        .route("/api/v1/tasks", post(create_task))
        .route("/api/v1/tasks/{task_id}/complete", post(complete_task))
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProfilePayload {
    full_name: String,
    birth_date: NaiveDate,
    occupation: String,
    telegram: Option<String>,
    email: Option<String>,
    timezone: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/profiles",
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
    let profile = state
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSpherePayload {
    name: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/spheres",
    request_body = CreateSpherePayload,
    responses(
        (status = 201, description = "Sphere created", body = Sphere),
        (status = 400, description = "Validation error", body = crate::api::error::ErrorEnvelope)
    )
)]
pub(crate) async fn create_sphere(
    State(state): State<AppState>,
    Json(payload): Json<CreateSpherePayload>,
) -> Result<(StatusCode, Json<Sphere>), ApiError> {
    let sphere = state
        .service
        .create_sphere(CreateSphereRequest { name: payload.name })
        .await?;
    Ok((StatusCode::CREATED, Json(sphere)))
}

#[utoipa::path(
    get,
    path = "/api/v1/spheres",
    responses(
        (status = 200, description = "List of spheres", body = [Sphere]),
        (status = 500, description = "Storage error", body = crate::api::error::ErrorEnvelope)
    )
)]
pub(crate) async fn list_spheres(
    State(state): State<AppState>,
) -> Result<Json<Vec<Sphere>>, ApiError> {
    Ok(Json(state.service.list_spheres().await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/profiles/{profile_id}",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for development authorization")
    ),
    responses(
        (status = 200, description = "Profile", body = Profile),
        (status = 400, description = "Invalid actor id", body = crate::api::error::ErrorEnvelope),
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
    let profile = state.service.get_profile(actor, profile_id).await?;
    Ok(Json(profile))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DashboardQuery {
    date: Option<NaiveDate>,
}

#[utoipa::path(
    get,
    path = "/api/v1/profiles/{profile_id}/dashboard",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("date" = Option<NaiveDate>, Query, description = "Optional dashboard date override"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for development authorization")
    ),
    responses(
        (status = 200, description = "Dashboard summary", body = Dashboard),
        (status = 400, description = "Invalid request", body = crate::api::error::ErrorEnvelope),
        (status = 403, description = "Access denied", body = crate::api::error::ErrorEnvelope),
        (status = 404, description = "Profile not found", body = crate::api::error::ErrorEnvelope)
    )
)]
pub(crate) async fn get_dashboard(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
    Query(query): Query<DashboardQuery>,
    headers: HeaderMap,
) -> Result<Json<Dashboard>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    let dashboard = state
        .service
        .dashboard(actor, profile_id, query.date)
        .await?;
    Ok(Json(dashboard))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTaskPayload {
    profile_id: Uuid,
    title: String,
    sphere_id: Option<Uuid>,
    kind: TaskKind,
    weight: i32,
    cadence: TaskCadence,
    scheduled_for: NaiveDate,
}

#[utoipa::path(
    post,
    path = "/api/v1/tasks",
    request_body = CreateTaskPayload,
    params(
        ("X-Actor-Id" = String, Header, description = "Profile id used for development authorization")
    ),
    responses(
        (status = 201, description = "Task created", body = Task),
        (status = 400, description = "Invalid request", body = crate::api::error::ErrorEnvelope),
        (status = 403, description = "Access denied", body = crate::api::error::ErrorEnvelope),
        (status = 404, description = "Profile not found", body = crate::api::error::ErrorEnvelope)
    )
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
                weight: payload.weight,
                cadence: payload.cadence,
                scheduled_for: payload.scheduled_for,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(task)))
}

#[utoipa::path(
    post,
    path = "/api/v1/tasks/{task_id}/complete",
    params(
        ("task_id" = Uuid, Path, description = "Task identifier"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for development authorization")
    ),
    responses(
        (status = 200, description = "Task completed", body = Task),
        (status = 400, description = "Invalid request", body = crate::api::error::ErrorEnvelope),
        (status = 403, description = "Access denied", body = crate::api::error::ErrorEnvelope),
        (status = 404, description = "Task not found", body = crate::api::error::ErrorEnvelope),
        (status = 409, description = "Task already completed", body = crate::api::error::ErrorEnvelope)
    )
)]
pub(crate) async fn complete_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Task>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    let task = state
        .service
        .complete_task(actor, CompleteTaskRequest { task_id })
        .await?;
    Ok(Json(task))
}

#[utoipa::path(
    post,
    path = "/api/v1/profiles/{profile_id}/days/{date}/finalize",
    params(
        ("profile_id" = Uuid, Path, description = "Profile identifier"),
        ("date" = NaiveDate, Path, description = "Day to finalize in YYYY-MM-DD format"),
        ("X-Actor-Id" = String, Header, description = "Profile id used for development authorization")
    ),
    responses(
        (status = 200, description = "Finalized snapshot", body = DailySnapshot),
        (status = 400, description = "Invalid request", body = crate::api::error::ErrorEnvelope),
        (status = 403, description = "Access denied", body = crate::api::error::ErrorEnvelope),
        (status = 404, description = "Profile not found", body = crate::api::error::ErrorEnvelope),
        (status = 409, description = "Day already finalized", body = crate::api::error::ErrorEnvelope)
    )
)]
pub(crate) async fn finalize_day(
    State(state): State<AppState>,
    Path((profile_id, date)): Path<(Uuid, NaiveDate)>,
    headers: HeaderMap,
) -> Result<Json<DailySnapshot>, ApiError> {
    let actor = actor_from_headers(&headers)?;
    let snapshot = state
        .service
        .finalize_day(actor, FinalizeDayRequest { profile_id, date })
        .await?;
    Ok(Json(snapshot))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::{
        api::{AppState, build_router},
        application::ProgressionService,
        infrastructure::SqliteRepository,
    };

    #[tokio::test]
    async fn protected_profile_read_requires_matching_actor() {
        let database = tempfile::NamedTempFile::new().unwrap();
        let repository = Arc::new(SqliteRepository::new(database.path()).unwrap());
        let service = Arc::new(ProgressionService::new(repository));
        let app = build_router(AppState::new(service, None));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/profiles/00000000-0000-0000-0000-000000000001")
                    .header("x-actor-id", "00000000-0000-0000-0000-000000000002")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn docs_ui_and_openapi_routes_are_available() {
        let database = tempfile::NamedTempFile::new().unwrap();
        let repository = Arc::new(SqliteRepository::new(database.path()).unwrap());
        let service = Arc::new(ProgressionService::new(repository));
        let app = build_router(AppState::new(service, None));

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

        let openapi_response = app
            .oneshot(
                Request::builder()
                    .uri("/docs/openapi.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(openapi_response.status(), StatusCode::OK);
    }
}
