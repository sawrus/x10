use argon2::{Argon2, PasswordVerifier};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post},
};
use password_hash::PasswordHash;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::{
        ApiError, AppState, admin_session_from_headers, build_admin_session_cookie,
        clear_admin_session_cookie, new_admin_session,
    },
    domain::{Profile, ProfileLevelState},
};

pub fn build_admin_router() -> Router<AppState> {
    Router::new()
        .route("/api/admin/auth/login", post(login))
        .route("/api/admin/auth/logout", post(logout))
        .route("/api/admin/auth/session", get(session))
        .route("/api/admin/profiles", get(list_profiles))
        .route("/api/admin/profiles/{profile_id}", delete(delete_profile))
        .route(
            "/api/admin/profiles/{profile_id}/level-state",
            get(get_level_state),
        )
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionPayload {
    authenticated: bool,
    username: Option<String>,
    expires_at: Option<i64>,
}

#[utoipa::path(
    post,
    path = "/api/admin/auth/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Admin session created", body = SessionPayload),
        (status = 401, description = "Invalid credentials", body = crate::api::error::ErrorEnvelope)
    )
)]
pub(crate) async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let parsed_hash = PasswordHash::new(&state.admin_password_hash).map_err(|_| {
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INVALID_ADMIN_CONFIG",
            "admin password hash is invalid",
        )
    })?;

    let password_ok = payload.username == state.admin_username
        && Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash)
            .is_ok();

    if !password_ok {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "INVALID_CREDENTIALS",
            "invalid username or password",
        ));
    }

    let set_cookie = build_admin_session_cookie(&state)?;
    let session = new_admin_session(&state);

    Ok((
        [(axum::http::header::SET_COOKIE, set_cookie)],
        Json(SessionPayload {
            authenticated: true,
            username: Some(session.username),
            expires_at: Some(session.expires_at),
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/api/admin/auth/logout",
    responses((status = 200, description = "Admin session cleared", body = SessionPayload))
)]
pub(crate) async fn logout() -> impl IntoResponse {
    (
        [(axum::http::header::SET_COOKIE, clear_admin_session_cookie())],
        Json(SessionPayload {
            authenticated: false,
            username: None,
            expires_at: None,
        }),
    )
}

#[utoipa::path(
    get,
    path = "/api/admin/auth/session",
    responses((status = 200, description = "Current admin session", body = SessionPayload))
)]
pub(crate) async fn session(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Json<SessionPayload> {
    let session = admin_session_from_headers(&headers, &state).ok();
    Json(SessionPayload {
        authenticated: session.is_some(),
        username: session.as_ref().map(|value| value.username.clone()),
        expires_at: session.as_ref().map(|value| value.expires_at),
    })
}

#[utoipa::path(
    get,
    path = "/api/admin/profiles",
    responses((status = 200, description = "Profiles list", body = [Profile]))
)]
pub(crate) async fn list_profiles(
    State(state): State<AppState>,
) -> Result<Json<Vec<Profile>>, ApiError> {
    Ok(Json(state.service.list_profiles().await?))
}

#[utoipa::path(
    delete,
    path = "/api/admin/profiles/{profile_id}",
    params(("profile_id" = Uuid, Path, description = "Profile identifier")),
    responses(
        (status = 204, description = "Profile deleted"),
        (status = 404, description = "Profile not found", body = crate::api::error::ErrorEnvelope)
    )
)]
pub(crate) async fn delete_profile(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    state.service.delete_profile(profile_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/admin/profiles/{profile_id}/level-state",
    params(("profile_id" = Uuid, Path, description = "Profile identifier")),
    responses((status = 200, description = "Level state", body = ProfileLevelState))
)]
pub(crate) async fn get_level_state(
    State(state): State<AppState>,
    Path(profile_id): Path<Uuid>,
) -> Result<Json<Option<ProfileLevelState>>, ApiError> {
    Ok(Json(state.service.get_level_state(profile_id).await?))
}
