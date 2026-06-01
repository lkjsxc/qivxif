mod support;

use axum::{Router, http::StatusCode};
use qivxif_api::{
    ApiEnvelope, EdgeCreatePayload, EdgeCreateRequest, EdgeListPayload, NodeCreatePayload,
    NodeCreateRequest, NodePayload,
};
use qivxif_core::{EdgeId, EventId, MetadataMap, NodeId, Visibility};
use qivxif_graph::{EdgeKind, NodeKind};
use qivxif_server::routes;
use support::{get, login, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn creates_reads_and_links_graph_records() {
    let app = routes::router(seeded_state("graph"));
    let (cookie, csrf) = login(&app).await;
    let first = NodeId::generate();
    let second = NodeId::generate();
    let first_event = EventId::generate();

    let created = create_node(
        &app,
        &cookie,
        &csrf,
        &first,
        &first_event,
        1,
        Visibility::Public,
    )
    .await;
    assert_eq!(created.node.id, first);
    let duplicate = create_node(
        &app,
        &cookie,
        &csrf,
        &first,
        &first_event,
        1,
        Visibility::Public,
    )
    .await;
    assert_eq!(duplicate.event, created.event);
    create_node(
        &app,
        &cookie,
        &csrf,
        &second,
        &EventId::generate(),
        2,
        Visibility::Private,
    )
    .await;

    let edge = EdgeId::generate();
    let body = EdgeCreateRequest {
        event_id: EventId::generate(),
        actor_seq: 3,
        edge_id: edge.clone(),
        from_node: first.clone(),
        to_node: second.clone(),
        kind: EdgeKind::LinksTo,
        metadata_map: MetadataMap::empty(),
    };
    let response = app
        .clone()
        .oneshot(post_json("/api/edges", &body, Some(&cookie), Some(&csrf)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<EdgeCreatePayload> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().edge.id, edge);

    let response = app
        .clone()
        .oneshot(get(&format!("/api/nodes/{first}"), Some(&cookie)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<NodePayload> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().projection.node.id, first);

    let response = app
        .oneshot(get(&format!("/api/nodes/{first}/edges"), Some(&cookie)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<EdgeListPayload> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().outgoing.len(), 1);
}

#[tokio::test]
async fn public_viewer_cannot_read_private_node() {
    let app = routes::router(seeded_state("private"));
    let (cookie, csrf) = login(&app).await;
    let node = NodeId::generate();
    create_node(
        &app,
        &cookie,
        &csrf,
        &node,
        &EventId::generate(),
        1,
        Visibility::Private,
    )
    .await;

    let response = app
        .oneshot(get(&format!("/api/nodes/{node}"), None))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

async fn create_node(
    app: &Router,
    cookie: &str,
    csrf: &str,
    node_id: &NodeId,
    event_id: &EventId,
    actor_seq: u64,
    visibility: Visibility,
) -> NodeCreatePayload {
    let body = NodeCreateRequest {
        event_id: event_id.clone(),
        actor_seq,
        node_id: node_id.clone(),
        kind: NodeKind::Text,
        visibility,
        metadata_map: MetadataMap::empty(),
    };
    let response = app
        .clone()
        .oneshot(post_json("/api/nodes", &body, Some(cookie), Some(csrf)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    read_json::<ApiEnvelope<NodeCreatePayload>>(response)
        .await
        .payload
        .unwrap()
}
