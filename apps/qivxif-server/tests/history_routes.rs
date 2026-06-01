mod support;

use axum::http::StatusCode;
use qivxif_api::{
    ApiEnvelope, NodeCreatePayload, NodeCreateRequest, NodeHistoryPayload, TextOperationPayload,
};
use qivxif_core::{MetadataMap, NodeId, OperationId, TextDocId, Visibility};
use qivxif_graph::NodeKind;
use qivxif_history::OperationKind;
use qivxif_server::routes;
use serde_json::json;
use support::{get, login_full, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn lists_authorized_node_history_in_acceptance_order() {
    let app = routes::router(seeded_state("history"));
    let login = login_full(&app).await;
    let node_id = NodeId::generate();
    create_text_node(&app, &login, &node_id).await;
    restore_text(&app, &login, &node_id).await;

    let response = app
        .clone()
        .oneshot(get(
            &format!("/api/nodes/{node_id}/history"),
            Some(&login.cookie),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<NodeHistoryPayload> = read_json(response).await;
    let operations = envelope.payload.unwrap().operations;
    assert_eq!(operations.len(), 2);
    assert_eq!(operations[0].kind, OperationKind::NodeCreate);
    assert_eq!(operations[1].kind, OperationKind::TextRestore);

    let public_response = app
        .oneshot(get(&format!("/api/nodes/{node_id}/history"), None))
        .await
        .unwrap();
    assert_eq!(public_response.status(), StatusCode::FORBIDDEN);
}

async fn create_text_node(app: &axum::Router, login: &support::TestLogin, node_id: &NodeId) {
    let node = NodeCreateRequest {
        op_id: OperationId::generate(),
        actor_seq: 1,
        node_id: node_id.clone(),
        kind: NodeKind::Text,
        visibility: Visibility::Private,
        metadata_map: MetadataMap::empty(),
    };
    let response = app
        .clone()
        .oneshot(post_json(
            "/api/nodes",
            &node,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    read_json::<ApiEnvelope<NodeCreatePayload>>(response).await;
}

async fn restore_text(app: &axum::Router, login: &support::TestLogin, node_id: &NodeId) {
    let request = json!({
        "actor_seq": 2,
        "operation": {
            "op_id": OperationId::generate().to_string(),
            "doc_id": TextDocId::generate().to_string(),
            "edit": {
                "kind": "restore",
                "content": "history text",
                "actor_id": login.actor_id.to_string(),
                "first_seq": 2000000
            }
        }
    });
    let response = app
        .clone()
        .oneshot(post_json(
            &format!("/api/text/{node_id}/ops"),
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    read_json::<ApiEnvelope<TextOperationPayload>>(response).await;
}
