use axum::{
    Json,
    http::{
        HeaderMap, HeaderValue, StatusCode,
        header::{COOKIE, SET_COOKIE},
    },
};
use qivxif_api::{ApiEnvelope, ApiError, ApiErrorCode};
use qivxif_core::{Capability, RequestId};
use qivxif_store_redb::StoreError;

pub type ApiResponse<T> = (StatusCode, Json<ApiEnvelope<T>>);

pub fn ok<T>(payload: T, capabilities: Vec<Capability>) -> ApiResponse<T> {
    (
        StatusCode::OK,
        Json(ApiEnvelope::success(
            RequestId::generate(),
            payload,
            capabilities,
        )),
    )
}

pub fn fail<T>(
    status: StatusCode,
    code: ApiErrorCode,
    message: impl Into<String>,
    capabilities: Vec<Capability>,
) -> ApiResponse<T> {
    (
        status,
        Json(ApiEnvelope::failure(
            RequestId::generate(),
            ApiError {
                code,
                message: message.into(),
                field_errors: Vec::new(),
                retry: None,
                conflict: None,
                required_capability: None,
            },
            capabilities,
        )),
    )
}

pub fn capabilities(values: &[&str]) -> Vec<Capability> {
    values
        .iter()
        .filter_map(|value| value.parse().ok())
        .collect()
}

pub fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    let cookie = headers.get(COOKIE)?.to_str().ok()?;
    cookie.split(';').find_map(|part| {
        part.trim()
            .strip_prefix(&format!("{name}="))
            .map(ToOwned::to_owned)
    })
}

pub fn write_cookie(headers: &mut HeaderMap, value: String) {
    if let Ok(value) = HeaderValue::from_str(&value) {
        headers.insert(SET_COOKIE, value);
    }
}

pub fn auth_missing<T>(caps: Vec<Capability>) -> ApiResponse<T> {
    fail(
        StatusCode::UNAUTHORIZED,
        ApiErrorCode::AuthSessionMissing,
        "valid session cookie is required",
        caps,
    )
}

pub fn csrf_missing<T>(caps: Vec<Capability>) -> ApiResponse<T> {
    fail(
        StatusCode::FORBIDDEN,
        ApiErrorCode::AuthCsrfMissing,
        "csrf token is required",
        caps,
    )
}

pub fn invalid_id<T>(caps: Vec<Capability>) -> ApiResponse<T> {
    fail(
        StatusCode::BAD_REQUEST,
        ApiErrorCode::SchemaInvalidId,
        "id is malformed",
        caps,
    )
}

pub fn graph_not_found<T>(caps: Vec<Capability>) -> ApiResponse<T> {
    fail(
        StatusCode::NOT_FOUND,
        ApiErrorCode::GraphNotFound,
        "graph record is absent or hidden",
        caps,
    )
}

pub fn graph_store_error<T>(error: StoreError, caps: Vec<Capability>) -> ApiResponse<T> {
    match error {
        StoreError::Forbidden => fail(
            StatusCode::FORBIDDEN,
            ApiErrorCode::AuthForbidden,
            "actor cannot access graph record",
            caps,
        ),
        StoreError::NodeMissing => graph_not_found(caps),
        StoreError::DuplicateActorSeq => fail(
            StatusCode::CONFLICT,
            ApiErrorCode::EventDuplicateActorSeq,
            "actor sequence already belongs to another event",
            caps,
        ),
        StoreError::NodeExists | StoreError::EdgeExists | StoreError::EventConflict => fail(
            StatusCode::CONFLICT,
            ApiErrorCode::StoreConflict,
            "durable graph write conflicts with existing data",
            caps,
        ),
        _ => fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorCode::StoreUnavailable,
            "store could not complete the request",
            caps,
        ),
    }
}
