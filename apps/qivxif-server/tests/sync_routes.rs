mod support;

use axum::http::StatusCode;
use qivxif_api::{ApiEnvelope, PullResponse, PushRequest, PushResponse};
use qivxif_core::{MetadataMap, NodeId, OperationId, ServerTime, Visibility};
use qivxif_graph::{NodeKind, NodeRecord};
use qivxif_history::{
    OperationEnvelope, OperationKind, OperationPayload, OperationScope, hash_payload,
};
use qivxif_server::routes;
use support::{get, login_full, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn pushes_duplicate_and_pulls_graph_operation() {
    let app = routes::router(seeded_state("sync"));
    let login = login_full(&app).await;
    let op = node_create_op(&login);
    let request = PushRequest {
        client_id: "client-a".to_owned(),
        actor_id: login.actor_id.clone(),
        operations: vec![op.clone()],
        cursor_summary: None,
    };

    let response = app
        .clone()
        .oneshot(post_json(
            "/api/sync/push",
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<PushResponse> = read_json(response).await;
    let pushed = envelope.payload.unwrap();
    assert_eq!(pushed.accepted.len(), 1);
    assert!(pushed.rejected.is_empty());

    let duplicate = app
        .clone()
        .oneshot(post_json(
            "/api/sync/push",
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    let envelope: ApiEnvelope<PushResponse> = read_json(duplicate).await;
    assert_eq!(envelope.payload.unwrap().accepted, pushed.accepted);

    let response = app
        .oneshot(get(
            "/api/sync/pull?limit=10&scope=graph",
            Some(&login.cookie),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<PullResponse> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().operations, [op]);
}

fn node_create_op(login: &support::TestLogin) -> OperationEnvelope {
    let now = ServerTime::now();
    let node = NodeRecord {
        id: NodeId::generate(),
        kind: NodeKind::Text,
        owner_user_id: login.user_id.clone(),
        created_by: login.actor_id.clone(),
        created_at: now,
        updated_at: now,
        visibility: Visibility::Private,
        acl_ref: None,
        current_commit_group: None,
        current_text_ref: None,
        metadata_map: MetadataMap::empty(),
        tombstone: None,
    };
    let bytes = bincode::serialize(&node).unwrap();
    OperationEnvelope {
        op_id: OperationId::generate(),
        actor_id: login.actor_id.clone(),
        actor_seq: 1,
        parents: Vec::new(),
        scope: OperationScope::Graph,
        kind: OperationKind::NodeCreate,
        target_node_ids: vec![node.id],
        payload: OperationPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: None,
        auth_context: None,
    }
}
