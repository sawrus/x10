pub(crate) mod error;
pub(crate) mod openapi;
pub(crate) mod routes;

use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    http::{HeaderMap, HeaderName, HeaderValue, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use metrics_exporter_prometheus::PrometheusHandle;
use tracing::info_span;
use uuid::Uuid;

use crate::{
    application::{Actor, ProgressionService, ServiceError},
    infrastructure::SqliteRepository,
};

pub use error::ApiError;
use openapi::build_docs_router;
pub use routes::build_routes;

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<ProgressionService<SqliteRepository>>,
    pub metrics: Option<PrometheusHandle>,
}

impl AppState {
    pub fn new(
        service: Arc<ProgressionService<SqliteRepository>>,
        metrics: Option<PrometheusHandle>,
    ) -> Self {
        Self { service, metrics }
    }
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(build_routes())
        .merge(build_docs_router())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            metrics_middleware,
        ))
        .route_layer(middleware::from_fn(request_id_middleware))
        .with_state(state)
}

pub fn actor_from_headers(headers: &HeaderMap) -> Result<Actor, ApiError> {
    let actor_id = headers
        .get("x-actor-id")
        .ok_or_else(|| ApiError::from(ServiceError::Forbidden))?
        .to_str()
        .ok()
        .and_then(|value| Uuid::parse_str(value).ok())
        .ok_or_else(|| ApiError::bad_request("INVALID_ACTOR_ID", "x-actor-id must be a UUID"))?;

    Ok(Actor {
        profile_id: actor_id,
    })
}

async fn request_id_middleware(
    request: Request<axum::body::Body>,
    next: Next,
) -> impl IntoResponse {
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.trim().is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let span = info_span!("http_request", request_id = %request_id, method = %request.method(), path = %request.uri().path());
    let _entered = span.enter();

    let mut response = next.run(request).await.into_response();
    response.headers_mut().insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(&request_id)
            .unwrap_or_else(|_| HeaderValue::from_static("invalid-request-id")),
    );

    tracing::info!(status = response.status().as_u16(), "request complete");
    response
}

async fn metrics_middleware(
    State(_state): State<AppState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let method = request.method().to_string();
    let path = request.uri().path().to_owned();
    let started_at = std::time::Instant::now();

    let response = next.run(request).await;
    let duration = started_at.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    metrics::counter!("http_requests_total", "method" => method.clone(), "path" => path.clone(), "status" => status).increment(1);
    metrics::histogram!("http_request_duration_seconds", "method" => method, "path" => path)
        .record(duration);

    if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
        tracing::error!("internal server error");
    }

    response
}
