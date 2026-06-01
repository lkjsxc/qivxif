mod support;

use axum::http::StatusCode;
use qivxif_api::{ApiEnvelope, LoginPayload, NodeCreateRequest};
use qivxif_core::{MetadataMap, NodeId, OperationId, Visibility};
use qivxif_graph::NodeKind;
use qivxif_server::routes;
use support::{login_full, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn login_returns_next_server_safe_actor_sequence() {
    let app = routes::router(seeded_state("login-actor-seq"));
    let login = login_full(&app).await;
    assert_eq!(login.next_actor_seq, 1);

    let request = NodeCreateRequest {
        actor_seq: 1,
        kind: NodeKind::Text,
        metadata_map: MetadataMap::empty(),
        node_id: NodeId::generate(),
        op_id: OperationId::generate(),
        visibility: Visibility::Private,
    };
    let response = app
        .clone()
        .oneshot(post_json(
            "/api/nodes",
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = serde_json::json!({ "name": "admin", "password": "secret" });
    let response = app
        .oneshot(post_json("/api/auth/login", &body, None, None))
        .await
        .unwrap();
    let envelope: ApiEnvelope<LoginPayload> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().next_actor_seq, 2);
}
