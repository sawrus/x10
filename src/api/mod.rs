pub(crate) mod admin;
pub(crate) mod error;
pub(crate) mod openapi;
pub(crate) mod routes;

use std::{path::PathBuf, sync::Arc};

use axum::{
    Router,
    extract::State,
    http::{HeaderMap, HeaderName, HeaderValue, Request, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use metrics_exporter_prometheus::PrometheusHandle;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tracing::info_span;
use uuid::Uuid;

use crate::{
    application::{Actor, ProgressionService, ServiceError},
    infrastructure::SqliteRepository,
};

use admin::build_admin_router;
pub use error::ApiError;
use openapi::build_docs_router;
pub use routes::build_routes;

type HmacSha256 = Hmac<Sha256>;

const ADMIN_COOKIE_NAME: &str = "x10_admin_session";
const ADMIN_SESSION_TTL_SECONDS: i64 = 60 * 60 * 12;

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<ProgressionService<SqliteRepository>>,
    pub metrics: Option<PrometheusHandle>,
    pub web_dist_path: PathBuf,
    pub admin_username: String,
    pub admin_password_hash: String,
    pub admin_session_secret: String,
    pub admin_session_secure: bool,
}

impl AppState {
    pub fn new(
        service: Arc<ProgressionService<SqliteRepository>>,
        metrics: Option<PrometheusHandle>,
        web_dist_path: PathBuf,
        admin_username: String,
        admin_password_hash: String,
        admin_session_secret: String,
        admin_session_secure: bool,
    ) -> Self {
        Self {
            service,
            metrics,
            web_dist_path,
            admin_username,
            admin_password_hash,
            admin_session_secret,
            admin_session_secure,
        }
    }
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(build_routes())
        .merge(build_admin_router())
        .merge(build_docs_router())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            admin_auth_middleware,
        ))
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminSession {
    pub username: String,
    pub expires_at: i64,
}

pub fn new_admin_session(state: &AppState) -> AdminSession {
    AdminSession {
        username: state.admin_username.clone(),
        expires_at: (Utc::now() + Duration::seconds(ADMIN_SESSION_TTL_SECONDS)).timestamp(),
    }
}

pub fn build_admin_session_cookie(state: &AppState) -> Result<HeaderValue, ApiError> {
    let session = new_admin_session(state);
    let token = encode_admin_session(&session, &state.admin_session_secret)?;
    let mut cookie = format!(
        "{ADMIN_COOKIE_NAME}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={ADMIN_SESSION_TTL_SECONDS}"
    );
    if state.admin_session_secure {
        cookie.push_str("; Secure");
    }
    HeaderValue::from_str(&cookie)
        .map_err(|_| ApiError::bad_request("INVALID_COOKIE", "failed to build session cookie"))
}

pub fn clear_admin_session_cookie() -> HeaderValue {
    HeaderValue::from_static(
        "x10_admin_session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
    )
}

pub fn admin_session_from_headers(
    headers: &HeaderMap,
    state: &AppState,
) -> Result<AdminSession, ApiError> {
    let cookie_header = headers
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| {
            ApiError::new(
                StatusCode::UNAUTHORIZED,
                "UNAUTHENTICATED",
                "login required",
            )
        })?;

    let cookie_value = cookie_header
        .split(';')
        .find_map(|part| {
            let trimmed = part.trim();
            trimmed
                .strip_prefix(&format!("{ADMIN_COOKIE_NAME}="))
                .map(ToOwned::to_owned)
        })
        .ok_or_else(|| {
            ApiError::new(
                StatusCode::UNAUTHORIZED,
                "UNAUTHENTICATED",
                "login required",
            )
        })?;

    let session = decode_admin_session(&cookie_value, &state.admin_session_secret)?;
    if session.username != state.admin_username || session.expires_at < Utc::now().timestamp() {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "UNAUTHENTICATED",
            "login required",
        ));
    }
    Ok(session)
}

fn encode_admin_session(session: &AdminSession, secret: &str) -> Result<String, ApiError> {
    let payload = serde_json::to_vec(session)
        .map_err(|_| ApiError::bad_request("INVALID_SESSION", "failed to encode session"))?;
    let payload_token = URL_SAFE_NO_PAD.encode(payload);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| ApiError::bad_request("INVALID_SESSION", "failed to sign session"))?;
    mac.update(payload_token.as_bytes());
    let signature = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
    Ok(format!("{payload_token}.{signature}"))
}

fn decode_admin_session(token: &str, secret: &str) -> Result<AdminSession, ApiError> {
    let (payload_token, signature_token) = token.split_once('.').ok_or_else(|| {
        ApiError::new(
            StatusCode::UNAUTHORIZED,
            "UNAUTHENTICATED",
            "login required",
        )
    })?;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).map_err(|_| {
        ApiError::new(
            StatusCode::UNAUTHORIZED,
            "UNAUTHENTICATED",
            "login required",
        )
    })?;
    mac.update(payload_token.as_bytes());
    let expected = mac.finalize().into_bytes();
    let provided = URL_SAFE_NO_PAD
        .decode(signature_token.as_bytes())
        .map_err(|_| {
            ApiError::new(
                StatusCode::UNAUTHORIZED,
                "UNAUTHENTICATED",
                "login required",
            )
        })?;

    if provided.as_slice() != expected.as_slice() {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "UNAUTHENTICATED",
            "login required",
        ));
    }

    let payload = URL_SAFE_NO_PAD
        .decode(payload_token.as_bytes())
        .map_err(|_| {
            ApiError::new(
                StatusCode::UNAUTHORIZED,
                "UNAUTHENTICATED",
                "login required",
            )
        })?;
    serde_json::from_slice(&payload).map_err(|_| {
        ApiError::new(
            StatusCode::UNAUTHORIZED,
            "UNAUTHENTICATED",
            "login required",
        )
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

    let span = info_span!(
        "http_request",
        request_id = %request_id,
        method = %request.method(),
        path = %request.uri().path()
    );
    let _entered = span.enter();

    let mut response = next.run(request).await.into_response();
    response
        .headers_mut()
        .entry(HeaderName::from_static("x-request-id"))
        .or_insert_with(|| {
            HeaderValue::from_str(&request_id)
                .unwrap_or_else(|_| HeaderValue::from_static("invalid-request-id"))
        });

    tracing::info!(status = response.status().as_u16(), "request complete");
    response
}

async fn admin_auth_middleware(
    State(state): State<AppState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let path = request.uri().path();
    let requires_admin_session =
        path.starts_with("/api/admin/") && !path.starts_with("/api/admin/auth/");

    if requires_admin_session && admin_session_from_headers(request.headers(), &state).is_err() {
        return ApiError::new(
            StatusCode::UNAUTHORIZED,
            "UNAUTHENTICATED",
            "login required",
        )
        .into_response();
    }

    next.run(request).await
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
