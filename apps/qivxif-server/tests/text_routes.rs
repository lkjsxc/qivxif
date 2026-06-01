mod support;

use axum::http::StatusCode;
use qivxif_api::{
    ApiEnvelope, NodeCreatePayload, NodeCreateRequest, TextOperationPayload, TextOperationRequest,
    TextPayload,
};
use qivxif_core::{MetadataMap, NodeId, OperationId, TextDocId, Visibility};
use qivxif_graph::NodeKind;
use qivxif_history::text::{TextCharId, TextEdit, TextInsert, TextInsertedChar, TextOperation};
use qivxif_server::routes;
use serde_json::json;
use support::{get, login_full, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn applies_and_reads_text_operation() {
    let app = routes::router(seeded_state("text"));
    let login = login_full(&app).await;
    let node_id = NodeId::generate();
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

    let request = TextOperationRequest {
        actor_seq: 2,
        operation: insert_op(&login, "hi"),
    };
    let path = format!("/api/text/{node_id}/ops");
    let response = app
        .clone()
        .oneshot(post_json(
            &path,
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<TextOperationPayload> = read_json(response).await;
    let accepted = envelope.payload.unwrap();
    assert_eq!(accepted.state.content, "hi");

    let duplicate = app
        .clone()
        .oneshot(post_json(
            &path,
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    let envelope: ApiEnvelope<TextOperationPayload> = read_json(duplicate).await;
    assert_eq!(envelope.payload.unwrap().operation, accepted.operation);

    let response = app
        .oneshot(get(&format!("/api/text/{node_id}"), Some(&login.cookie)))
        .await
        .unwrap();
    let envelope: ApiEnvelope<TextPayload> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().state.content, "hi");
}

#[tokio::test]
async fn accepts_browser_restore_text_shape() {
    let app = routes::router(seeded_state("text-restore"));
    let login = login_full(&app).await;
    let node_id = NodeId::generate();
    create_text_node(&app, &login, &node_id).await;

    let request = json!({
        "actor_seq": 2,
        "operation": {
            "op_id": OperationId::generate().to_string(),
            "doc_id": TextDocId::generate().to_string(),
            "edit": {
                "kind": "restore",
                "content": "browser text",
                "actor_id": login.actor_id.to_string(),
                "first_seq": 2000000
            }
        }
    });
    let response = app
        .oneshot(post_json(
            &format!("/api/text/{node_id}/ops"),
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<TextOperationPayload> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().state.content, "browser text");
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
}

fn insert_op(login: &support::TestLogin, text: &str) -> TextOperation {
    TextOperation {
        op_id: OperationId::generate(),
        doc_id: TextDocId::generate(),
        edit: TextEdit::Insert(TextInsert {
            after: None,
            chars: text
                .chars()
                .enumerate()
                .map(|(offset, value)| TextInsertedChar {
                    id: TextCharId {
                        actor_id: login.actor_id.clone(),
                        seq: 1 + offset as u64,
                    },
                    value,
                })
                .collect(),
        }),
    }
}
