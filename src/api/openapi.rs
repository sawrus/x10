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
        error::{ErrorBody, ErrorEnvelope},
        routes::{CreateProfilePayload, CreateSpherePayload, CreateTaskPayload, DashboardQuery},
    },
    domain::{
        DailySnapshot, Dashboard, DaySummary, Profile, ProgressionSummary, Sphere, Task,
        TaskCadence, TaskKind, TaskStatus,
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
        crate::api::routes::list_spheres,
        crate::api::routes::create_sphere,
        crate::api::routes::create_profile,
        crate::api::routes::get_profile,
        crate::api::routes::get_dashboard,
        crate::api::routes::finalize_day,
        crate::api::routes::create_task,
        crate::api::routes::complete_task
    ),
    components(
        schemas(
            ErrorEnvelope,
            ErrorBody,
            CreateProfilePayload,
            CreateSpherePayload,
            DashboardQuery,
            CreateTaskPayload,
            Profile,
            Sphere,
            Task,
            TaskKind,
            TaskStatus,
            TaskCadence,
            DaySummary,
            DailySnapshot,
            Dashboard,
            ProgressionSummary
        )
    ),
    info(
        title = "x10 backend API",
        version = "0.2.0",
        description = "Interactive API documentation served by the same Axum service as the backend."
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
