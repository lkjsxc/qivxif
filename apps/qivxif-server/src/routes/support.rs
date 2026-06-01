use axum::{
    Json,
    http::{
        HeaderMap, HeaderValue, StatusCode,
        header::{COOKIE, SET_COOKIE},
    },
};
use qivxif_api::{ApiEnvelope, ApiError, ApiErrorCode};
use qivxif_core::{Capability, RequestId};

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
