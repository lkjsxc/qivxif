mod support;

use axum::http::StatusCode;
use qivxif_api::{ApiEnvelope, PullResponse, PushRequest, PushResponse};
use qivxif_auth::{AuthRole, hash_password};
use qivxif_core::{MetadataMap, NodeId, OperationId, ServerTime, Visibility};
use qivxif_graph::{NodeKind, NodeRecord};
use qivxif_history::{
    OperationEnvelope, OperationKind, OperationPayload, OperationScope, hash_payload,
};
use qivxif_server::routes;
use support::{get, login_full, login_named, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn pull_filters_private_operations_for_other_member() {
    let state = seeded_state("sync-acl");
    state
        .store
        .create_user(
            "member".to_owned(),
            hash_password("member-secret").unwrap(),
            vec![AuthRole::Member],
        )
        .unwrap();
    let app = routes::router(state);
    let admin = login_full(&app).await;
    let member = login_named(&app, "member", "member-secret").await;
    let (op, _) = node_create_op(&admin, 1);
    let request = PushRequest {
        client_id: "client-owner".to_owned(),
        actor_id: admin.actor_id.clone(),
        operations: vec![op.clone()],
        cursor_summary: None,
    };

    let response = app
        .clone()
        .oneshot(post_json(
            "/api/sync/push",
            &request,
            Some(&admin.cookie),
            Some(&admin.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<PushResponse> = read_json(response).await;
    assert!(envelope.payload.unwrap().rejected.is_empty());

    let response = app
        .clone()
        .oneshot(get(
            "/api/sync/pull?limit=10&scope=graph",
            Some(&member.cookie),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<PullResponse> = read_json(response).await;
    let pulled = envelope.payload.unwrap();
    assert!(pulled.operations.is_empty());
    assert_eq!(pulled.server_cursor, None);

    let response = app
        .oneshot(get(
            "/api/sync/pull?limit=10&scope=graph",
            Some(&admin.cookie),
        ))
        .await
        .unwrap();
    let envelope: ApiEnvelope<PullResponse> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().operations, [op]);
}

fn node_create_op(login: &support::TestLogin, actor_seq: u64) -> (OperationEnvelope, NodeId) {
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
    let node_id = node.id.clone();
    let bytes = bincode::serialize(&node).unwrap();
    let envelope = OperationEnvelope {
        op_id: OperationId::generate(),
        actor_id: login.actor_id.clone(),
        actor_seq,
        parents: Vec::new(),
        scope: OperationScope::Graph,
        kind: OperationKind::NodeCreate,
        target_node_ids: vec![node_id.clone()],
        payload: OperationPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: None,
        auth_context: None,
    };
    (envelope, node_id)
}
