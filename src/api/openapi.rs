use axum::{
    Json, Router,
    response::{Html, Redirect},
    routing::get,
};
use utoipa::{Modify, OpenApi};
use utoipa_scalar::Scalar;

use crate::{
    api::{
        AppState,
        admin::{LoginPayload, SessionPayload},
        error::{ErrorBody, ErrorEnvelope},
        routes::{
            CreateDayFinalizationPayload, CreateLevelPayload, CreateProfilePayload,
            CreateSpherePayload, CreateTaskExecutionPayload, CreateTaskPayload, UpdateLevelPayload,
            UpdateProfilePayload, UpdateSpherePayload, UpdateTaskPayload,
        },
    },
    domain::{
        Dashboard, DayFinalization, Level, Profile, ProfileBalance, ProfileLevelState,
        ProfilePhotoSummary, ProgressionSummary, Sphere, Task, TaskCadence, TaskExecution,
        TaskKind, TaskStatus,
    },
};

struct ServerAddon;

impl Modify for ServerAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.servers = Some(vec![utoipa::openapi::Server::new("/")]);
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&ServerAddon),
    paths(
        crate::api::routes::health,
        crate::api::routes::metrics,
        crate::api::routes::list_profiles,
        crate::api::routes::create_profile,
        crate::api::routes::get_profile,
        crate::api::routes::update_profile,
        crate::api::routes::upload_photo,
        crate::api::routes::list_photos,
        crate::api::routes::get_photo,
        crate::api::routes::delete_photo,
        crate::api::routes::select_photo,
        crate::api::routes::create_sphere,
        crate::api::routes::list_spheres,
        crate::api::routes::get_sphere,
        crate::api::routes::update_sphere,
        crate::api::routes::delete_sphere,
        crate::api::routes::create_task,
        crate::api::routes::list_tasks,
        crate::api::routes::get_task,
        crate::api::routes::update_task,
        crate::api::routes::delete_task,
        crate::api::routes::create_execution,
        crate::api::routes::list_executions,
        crate::api::routes::get_execution,
        crate::api::routes::delete_execution,
        crate::api::routes::list_balances,
        crate::api::routes::get_dashboard,
        crate::api::routes::list_levels,
        crate::api::routes::create_level,
        crate::api::routes::update_level,
        crate::api::routes::delete_level,
        crate::api::routes::finalize_day,
        crate::api::routes::list_day_finalizations,
        crate::api::routes::delete_day_finalization,
        crate::api::admin::login,
        crate::api::admin::logout,
        crate::api::admin::session,
        crate::api::admin::list_profiles,
        crate::api::admin::delete_profile,
        crate::api::admin::get_level_state
    ),
    components(
        schemas(
            ErrorEnvelope,
            ErrorBody,
            LoginPayload,
            SessionPayload,
            CreateProfilePayload,
            UpdateProfilePayload,
            CreateSpherePayload,
            UpdateSpherePayload,
            CreateTaskPayload,
            UpdateTaskPayload,
            CreateTaskExecutionPayload,
            CreateLevelPayload,
            UpdateLevelPayload,
            CreateDayFinalizationPayload,
            Profile,
            ProfilePhotoSummary,
            Sphere,
            Task,
            TaskKind,
            TaskStatus,
            TaskCadence,
            TaskExecution,
            ProfileBalance,
            Level,
            ProfileLevelState,
            DayFinalization,
            Dashboard,
            ProgressionSummary
        )
    ),
    info(
        title = "x10 backend API",
        version = "0.4.5",
        description = "Interactive API documentation for the x10 admin backend."
    )
)]
pub struct ApiDoc;

pub fn build_docs_router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/docs", get(docs_redirect))
        .route("/docs/", get(docs_ui))
        .route("/docs/openapi.json", get(openapi_json))
}

async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

async fn docs_redirect() -> Redirect {
    Redirect::permanent("/docs/")
}

async fn docs_ui() -> Html<String> {
    Html(Scalar::new(ApiDoc::openapi()).to_html())
}
