use super::support::cookie_value;
use crate::state::AppState;
use axum::http::HeaderMap;
use qivxif_auth::{AuthContext, AuthRole, Viewer, verify_csrf_token};
use qivxif_core::SessionId;
use qivxif_store_redb::{StoredSession, StoredUser};

pub const SESSION_COOKIE: &str = "qivxif_session";
pub const CSRF_HEADER: &str = "x-qivxif-csrf";

pub struct SessionUser {
    pub session: StoredSession,
    pub user: StoredUser,
}

pub fn load_session_user(state: &AppState, headers: &HeaderMap) -> Option<SessionUser> {
    let session_id = session_id(headers)?;
    let Ok(Some(session)) = state.store.get_session(&session_id) else {
        return None;
    };
    let Ok(Some(user)) = state.store.get_user(&session.user_id) else {
        return None;
    };
    Some(SessionUser { session, user })
}

pub fn csrf_matches(headers: &HeaderMap, session: &StoredSession) -> bool {
    headers
        .get(CSRF_HEADER)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|token| verify_csrf_token(token, &session.csrf_token_hash))
}

pub fn auth_context(session_user: &SessionUser) -> AuthContext {
    AuthContext {
        viewer: Viewer::Session {
            user_id: session_user.user.id.clone(),
            actor_id: session_user.user.actor_id.clone(),
            session_id: session_user.session.id.clone(),
        },
        roles: session_user.user.roles.clone(),
    }
}

pub fn session_cookie(value: &str, secure: bool) -> String {
    format!(
        "{SESSION_COOKIE}={value}; Path=/; HttpOnly; SameSite=Lax{}",
        secure_suffix(secure)
    )
}

pub fn clear_session_cookie(secure: bool) -> String {
    format!(
        "{SESSION_COOKIE}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax{}",
        secure_suffix(secure)
    )
}

fn session_id(headers: &HeaderMap) -> Option<SessionId> {
    cookie_value(headers, SESSION_COOKIE)?.parse().ok()
}

fn secure_suffix(secure: bool) -> &'static str {
    if secure { "; Secure" } else { "" }
}

pub fn role_names(roles: &[AuthRole]) -> Vec<String> {
    roles
        .iter()
        .map(|role| format!("{role:?}").to_ascii_lowercase())
        .collect()
}
