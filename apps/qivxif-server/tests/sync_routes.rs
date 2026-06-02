mod support;

use axum::http::StatusCode;
use qivxif_api::{ApiEnvelope, PullResponse, PushRequest, PushResponse};
use qivxif_core::{EventId, MetadataMap, NodeId, ServerTime, TextDocId, Visibility};
use qivxif_graph::{NodeKind, NodeRecord};
use qivxif_history::{
    EventEnvelope, EventKind, EventPayload, EventScope, hash_payload,
    text::{TextEdit, TextEvent, TextRestore},
};
use qivxif_server::routes;
use support::{get, login_full, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn pushes_duplicate_and_pulls_graph_event() {
    let app = routes::router(seeded_state("sync"));
    let login = login_full(&app).await;
    let (event, _) = node_create_event(&login, 1);
    let request = PushRequest {
        client_id: "client-a".to_owned(),
        actor_id: login.actor_id.clone(),
        events: vec![event.clone()],
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
    assert_opaque_cursor(&pushed.accepted[0].server_cursor);

    let (mut conflict_event, _) = node_create_event(&login, 1);
    conflict_event.event_id = event.event_id.clone();
    let conflict = PushRequest {
        client_id: "client-a".to_owned(),
        actor_id: login.actor_id.clone(),
        events: vec![conflict_event],
        cursor_summary: None,
    };
    let response = app
        .clone()
        .oneshot(post_json(
            "/api/sync/push",
            &conflict,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    let envelope: ApiEnvelope<PushResponse> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().rejected[0].code, "store.conflict");

    let response = app
        .oneshot(get(
            "/api/sync/pull?limit=10&scope=graph",
            Some(&login.cookie),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<PullResponse> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().events, [event]);
}

#[tokio::test]
async fn pushes_text_event_after_graph_node() {
    let app = routes::router(seeded_state("sync-text"));
    let login = login_full(&app).await;
    let (node_event, node_id) = node_create_event(&login, 1);
    let text_event = text_restore_event(&login, &node_id, 2);
    let request = PushRequest {
        client_id: "client-text".to_owned(),
        actor_id: login.actor_id.clone(),
        events: vec![node_event, text_event],
        cursor_summary: None,
    };

    let response = app
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
    assert!(pushed.rejected.is_empty(), "{:?}", pushed.rejected);
    assert_eq!(pushed.accepted.len(), 2);
}

fn assert_opaque_cursor(cursor: &qivxif_core::CursorId) {
    let text = cursor.as_str();
    assert!(text.starts_with("cur_"));
    assert_ne!(
        text,
        "cur_0000000000000000000000000000000000000000000000000000000000000001"
    );
}

fn node_create_event(login: &support::TestLogin, actor_seq: u64) -> (EventEnvelope, NodeId) {
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
    let envelope = EventEnvelope {
        event_id: EventId::generate(),
        actor_id: login.actor_id.clone(),
        actor_seq,
        parents: Vec::new(),
        scope: EventScope::Graph,
        kind: EventKind::NodeCreate,
        target_node_ids: vec![node_id.clone()],
        target_edge_ids: Vec::new(),
        target_event_ids: Vec::new(),
        payload: EventPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: None,
        auth_context: None,
    };
    (envelope, node_id)
}

fn text_restore_event(
    login: &support::TestLogin,
    node_id: &NodeId,
    actor_seq: u64,
) -> EventEnvelope {
    let event = TextEvent {
        event_id: EventId::generate(),
        doc_id: TextDocId::generate(),
        edit: TextEdit::Restore(TextRestore {
            content: "sync text".to_owned(),
            actor_id: login.actor_id.clone(),
            first_seq: actor_seq * 1000000,
        }),
    };
    let bytes = serde_json::to_vec(&event).unwrap();
    EventEnvelope {
        event_id: event.event_id.clone(),
        actor_id: login.actor_id.clone(),
        actor_seq,
        parents: Vec::new(),
        scope: EventScope::Text,
        kind: EventKind::TextRestore,
        target_node_ids: vec![node_id.clone()],
        target_edge_ids: Vec::new(),
        target_event_ids: Vec::new(),
        payload: EventPayload {
            bytes: bytes.clone(),
        },
        payload_hash: hash_payload(&bytes),
        created_at_client: None,
        received_at_server: None,
        auth_context: None,
    }
}
