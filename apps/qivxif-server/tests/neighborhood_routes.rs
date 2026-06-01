mod support;

use axum::http::StatusCode;
use qivxif_api::{
    ApiEnvelope, EdgeCreatePayload, EdgeCreateRequest, NeighborhoodPayload, NodeCreatePayload,
    NodeCreateRequest,
};
use qivxif_core::{EdgeId, MetadataMap, NodeId, OperationId, Visibility};
use qivxif_graph::{EdgeKind, NodeKind};
use qivxif_server::routes;
use support::{get, login, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn returns_acl_filtered_neighborhood() {
    let app = routes::router(seeded_state("neighborhood"));
    let (cookie, csrf) = login(&app).await;
    let first = create_node(&app, &cookie, &csrf, 1).await;
    let second = create_node(&app, &cookie, &csrf, 2).await;
    create_edge(&app, &cookie, &csrf, &first, &second).await;

    let path = format!("/api/graph/neighborhood?node_id={first}&depth=1&limit=10");
    let response = app
        .clone()
        .oneshot(get(&path, Some(&cookie)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<NeighborhoodPayload> = read_json(response).await;
    let projection = envelope.payload.unwrap().projection;
    assert_eq!(projection.nodes.len(), 2);
    assert!(
        projection
            .nodes
            .iter()
            .any(|item| item.node.id == first && item.outgoing.len() == 1)
    );

    let public_response = app.oneshot(get(&path, None)).await.unwrap();
    assert_eq!(public_response.status(), StatusCode::FORBIDDEN);
}

async fn create_node(app: &axum::Router, cookie: &str, csrf: &str, seq: u64) -> NodeId {
    let node_id = NodeId::generate();
    let body = NodeCreateRequest {
        op_id: OperationId::generate(),
        actor_seq: seq,
        node_id: node_id.clone(),
        kind: NodeKind::Text,
        visibility: Visibility::Private,
        metadata_map: MetadataMap::empty(),
    };
    let response = app
        .clone()
        .oneshot(post_json("/api/nodes", &body, Some(cookie), Some(csrf)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    read_json::<ApiEnvelope<NodeCreatePayload>>(response).await;
    node_id
}

async fn create_edge(app: &axum::Router, cookie: &str, csrf: &str, from: &NodeId, to: &NodeId) {
    let body = EdgeCreateRequest {
        op_id: OperationId::generate(),
        actor_seq: 3,
        edge_id: EdgeId::generate(),
        from_node: from.clone(),
        to_node: to.clone(),
        kind: EdgeKind::LinksTo,
        metadata_map: MetadataMap::empty(),
    };
    let response = app
        .clone()
        .oneshot(post_json("/api/edges", &body, Some(cookie), Some(csrf)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    read_json::<ApiEnvelope<EdgeCreatePayload>>(response).await;
}
