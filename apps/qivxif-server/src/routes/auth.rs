use super::{
    session::{clear_session_cookie, csrf_matches, load_session_user, role_names, session_cookie},
    support::{ApiResponse, capabilities, fail, ok, write_cookie},
};
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use qivxif_api::{ApiErrorCode, LoginPayload, LoginRequest, LogoutPayload, MePayload, UserSummary};
use qivxif_auth::{generate_csrf_token, hash_csrf_token, verify_password};
use qivxif_core::SessionId;
use qivxif_store_redb::{StoredSession, StoredUser};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/me", get(me))
}

async fn login(State(state): State<AppState>, Json(request): Json<LoginRequest>) -> Response {
    let caps = auth_capabilities();
    let Ok(Some(user)) = state.store.find_user_by_name(&request.name) else {
        return invalid_credentials::<LoginPayload>(caps).into_response();
    };
    if verify_password(&request.password, &user.password_hash).is_err() {
        return invalid_credentials::<LoginPayload>(caps).into_response();
    }
    let Ok(next_actor_seq) = state.store.next_actor_seq(&user.actor_id) else {
        return store_unavailable::<LoginPayload>(caps).into_response();
    };
    let csrf_token = generate_csrf_token();
    let session = StoredSession {
        id: SessionId::generate(),
        user_id: user.id.clone(),
        actor_id: user.actor_id.clone(),
        csrf_token_hash: hash_csrf_token(&csrf_token),
    };
    if state.store.create_session(session.clone()).is_err() {
        return store_unavailable::<LoginPayload>(caps).into_response();
    }
    let mut response = ok(
        LoginPayload {
            user: user_summary(&user),
            csrf_token,
            next_actor_seq,
        },
        caps,
    )
    .into_response();
    write_cookie(
        response.headers_mut(),
        session_cookie(session.id.as_str(), state.config.cookie_secure),
    );
    response
}

async fn logout(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let caps = auth_capabilities();
    let Some(session_user) = load_session_user(&state, &headers) else {
        return missing_session::<LogoutPayload>(caps).into_response();
    };
    if !csrf_matches(&headers, &session_user.session) {
        return missing_csrf::<LogoutPayload>(caps).into_response();
    }
    if state
        .store
        .delete_session(&session_user.session.id)
        .is_err()
    {
        return store_unavailable::<LogoutPayload>(caps).into_response();
    }
    let mut response = ok(LogoutPayload { logged_out: true }, caps).into_response();
    write_cookie(
        response.headers_mut(),
        clear_session_cookie(state.config.cookie_secure),
    );
    response
}

async fn me(State(state): State<AppState>, headers: HeaderMap) -> ApiResponse<MePayload> {
    let caps = auth_capabilities();
    match load_session_user(&state, &headers) {
        Some(session_user) => ok(
            MePayload {
                user: user_summary(&session_user.user),
            },
            caps,
        ),
        None => missing_session(caps),
    }
}

fn user_summary(user: &StoredUser) -> UserSummary {
    UserSummary {
        user_id: user.id.clone(),
        actor_id: user.actor_id.clone(),
        name: user.name.clone(),
        roles: role_names(&user.roles),
        profile_node_id: None,
    }
}

fn invalid_credentials<T>(caps: Vec<qivxif_core::Capability>) -> ApiResponse<T> {
    fail(
        StatusCode::UNAUTHORIZED,
        ApiErrorCode::AuthInvalidCredentials,
        "login name or password did not verify",
        caps,
    )
}

fn missing_session<T>(caps: Vec<qivxif_core::Capability>) -> ApiResponse<T> {
    fail(
        StatusCode::UNAUTHORIZED,
        ApiErrorCode::AuthSessionMissing,
        "valid session cookie is required",
        caps,
    )
}

fn missing_csrf<T>(caps: Vec<qivxif_core::Capability>) -> ApiResponse<T> {
    fail(
        StatusCode::FORBIDDEN,
        ApiErrorCode::AuthCsrfMissing,
        "csrf token is required",
        caps,
    )
}

fn store_unavailable<T>(caps: Vec<qivxif_core::Capability>) -> ApiResponse<T> {
    fail(
        StatusCode::INTERNAL_SERVER_ERROR,
        ApiErrorCode::StoreUnavailable,
        "store could not complete the request",
        caps,
    )
}

fn auth_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["auth.session"])
}
