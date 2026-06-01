mod support;

use axum::http::{StatusCode, header::SET_COOKIE};
use qivxif_api::{ApiEnvelope, MePayload, SetupOwnerPayload, SetupStatusPayload};
use qivxif_server::routes;
use support::{empty_state, get, post_json, read_json};
use tower::ServiceExt;

#[tokio::test]
async fn empty_store_creates_owner_session_once() {
    let app = routes::router(empty_state("setup-owner"));

    let response = app.clone().oneshot(get("/api/setup", None)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<SetupStatusPayload> = read_json(response).await;
    assert!(envelope.payload.unwrap().owner_creation_open);

    let body = serde_json::json!({ "name": " admin ", "password": "secret" });
    let response = app
        .clone()
        .oneshot(post_json("/api/setup/owner", &body, None, None))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let cookie = response
        .headers()
        .get(SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    assert!(cookie.contains("HttpOnly"));
    let envelope: ApiEnvelope<SetupOwnerPayload> = read_json(response).await;
    let payload = envelope.payload.unwrap();
    assert_eq!(payload.user.name, "admin");
    assert_eq!(payload.user.roles, ["owner", "admin"]);
    assert_eq!(payload.next_actor_seq, 1);
    assert!(!payload.csrf_token.is_empty());

    let response = app
        .clone()
        .oneshot(get("/api/me", Some(&cookie)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<MePayload> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().user.name, "admin");

    let conflict = app
        .clone()
        .oneshot(post_json("/api/setup/owner", &body, None, None))
        .await
        .unwrap();
    assert_eq!(conflict.status(), StatusCode::CONFLICT);

    let response = app.oneshot(get("/api/setup", None)).await.unwrap();
    let envelope: ApiEnvelope<SetupStatusPayload> = read_json(response).await;
    let status = envelope.payload.unwrap();
    assert!(!status.required);
    assert!(!status.owner_creation_open);
}

#[tokio::test]
async fn setup_rejects_invalid_owner_request() {
    let app = routes::router(empty_state("setup-invalid"));
    let body = serde_json::json!({ "name": " ", "password": "short" });
    let response = app
        .oneshot(post_json("/api/setup/owner", &body, None, None))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
