mod support;

use axum::http::StatusCode;
use qivxif_api::{
    ApiEnvelope, NodeCreatePayload, NodeCreateRequest, NodePayload, TileLayoutPayload,
    TileLayoutSetRequest,
};
use qivxif_core::{EventId, MetadataMap, NodeId, Visibility};
use qivxif_graph::{NodeKind, TileLayout, TileTab, TileTree};
use qivxif_server::routes;
use support::{get, login_full, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn accepts_tile_layout_snapshot() {
    let app = routes::router(seeded_state("tile-layout"));
    let login = login_full(&app).await;
    let layout_node = create_layout_node(&app, &login).await;
    let pane_node = create_pane_node(&app, &login).await;
    let event_id = EventId::generate();
    let request = TileLayoutSetRequest {
        event_id: event_id.clone(),
        actor_seq: 3,
        layout_node_id: layout_node.clone(),
        layout: layout(pane_node),
    };

    let response = app
        .clone()
        .oneshot(post_json(
            "/api/tile-layout",
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<TileLayoutPayload> = read_json(response).await;
    let accepted = envelope.payload.unwrap();
    assert_eq!(accepted.event.event_id, event_id);

    let duplicate = app
        .clone()
        .oneshot(post_json(
            "/api/tile-layout",
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    let envelope: ApiEnvelope<TileLayoutPayload> = read_json(duplicate).await;
    assert_eq!(envelope.payload.unwrap().event, accepted.event);

    let response = app
        .oneshot(get(
            &format!("/api/nodes/{layout_node}"),
            Some(&login.cookie),
        ))
        .await
        .unwrap();
    let envelope: ApiEnvelope<NodePayload> = read_json(response).await;
    let node = envelope.payload.unwrap().projection.node;
    assert!(node.metadata_map.get("layout_json").is_some());
}

async fn create_layout_node(app: &axum::Router, login: &support::TestLogin) -> NodeId {
    create_node(app, login, 1, NodeKind::TileLayout).await
}

async fn create_pane_node(app: &axum::Router, login: &support::TestLogin) -> NodeId {
    create_node(app, login, 2, NodeKind::Pane).await
}

async fn create_node(
    app: &axum::Router,
    login: &support::TestLogin,
    actor_seq: u64,
    kind: NodeKind,
) -> NodeId {
    let node_id = NodeId::generate();
    let request = NodeCreateRequest {
        actor_seq,
        kind,
        metadata_map: MetadataMap::empty(),
        node_id: node_id.clone(),
        event_id: EventId::generate(),
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
    read_json::<ApiEnvelope<NodeCreatePayload>>(response).await;
    node_id
}

fn layout(pane_node_id: NodeId) -> TileLayout {
    TileLayout {
        maximized_pane_id: Some(pane_node_id.clone()),
        root: TileTree::Stack {
            active: 0,
            tabs: vec![TileTab {
                pane_kind: "text_editor".to_owned(),
                pane_node_id,
                target_node_id: None,
                title: "Text".to_owned(),
            }],
        },
    }
}
