use super::{
    session::{role_names, session_cookie},
    support::{ApiResponse, capabilities, fail, ok, write_cookie},
};
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use qivxif_api::{
    ApiErrorCode, SetupOwnerPayload, SetupOwnerRequest, SetupStatusPayload, UserSummary,
};
use qivxif_auth::{generate_csrf_token, hash_csrf_token, hash_password};
use qivxif_core::SessionId;
use qivxif_store_redb::{StoreError, StoredSession, StoredUser};

const MIN_PASSWORD_LEN: usize = 6;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/setup", get(status))
        .route("/api/setup/owner", post(create_owner))
}

async fn status(State(state): State<AppState>) -> ApiResponse<SetupStatusPayload> {
    let caps = setup_capabilities();
    match state.store.has_users() {
        Ok(has_users) => ok(
            SetupStatusPayload {
                required: !has_users,
                owner_creation_open: !has_users,
            },
            caps,
        ),
        Err(_) => store_unavailable(caps),
    }
}

async fn create_owner(
    State(state): State<AppState>,
    Json(request): Json<SetupOwnerRequest>,
) -> Response {
    let caps = setup_capabilities();
    let name = request.name.trim().to_owned();
    if name.is_empty() || request.password.len() < MIN_PASSWORD_LEN {
        return invalid_setup::<SetupOwnerPayload>(caps).into_response();
    }
    let Ok(password_hash) = hash_password(&request.password) else {
        return store_unavailable::<SetupOwnerPayload>(caps).into_response();
    };
    let user = match state.store.create_admin_user(name, password_hash) {
        Ok(user) => user,
        Err(StoreError::AdminExists | StoreError::DuplicateUserName) => {
            return setup_closed::<SetupOwnerPayload>(caps).into_response();
        }
        Err(_) => return store_unavailable::<SetupOwnerPayload>(caps).into_response(),
    };
    let Ok(next_actor_seq) = state.store.next_actor_seq(&user.actor_id) else {
        return store_unavailable::<SetupOwnerPayload>(caps).into_response();
    };
    let csrf_token = generate_csrf_token();
    let session = StoredSession {
        id: SessionId::generate(),
        user_id: user.id.clone(),
        actor_id: user.actor_id.clone(),
        csrf_token_hash: hash_csrf_token(&csrf_token),
    };
    if state.store.create_session(session.clone()).is_err() {
        return store_unavailable::<SetupOwnerPayload>(caps).into_response();
    }
    let mut response = ok(
        SetupOwnerPayload {
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

fn user_summary(user: &StoredUser) -> UserSummary {
    UserSummary {
        user_id: user.id.clone(),
        actor_id: user.actor_id.clone(),
        name: user.name.clone(),
        roles: role_names(&user.roles),
        profile_node_id: Some(user.profile_node_id.clone()),
    }
}

fn invalid_setup<T>(caps: Vec<qivxif_core::Capability>) -> ApiResponse<T> {
    fail(
        StatusCode::BAD_REQUEST,
        ApiErrorCode::SchemaInvalidInput,
        "name and password are required",
        caps,
    )
}

fn setup_closed<T>(caps: Vec<qivxif_core::Capability>) -> ApiResponse<T> {
    fail(
        StatusCode::CONFLICT,
        ApiErrorCode::StoreConflict,
        "owner creation is closed",
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

fn setup_capabilities() -> Vec<qivxif_core::Capability> {
    capabilities(&["setup.owner_create"])
}
