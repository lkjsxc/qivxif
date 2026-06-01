mod support;

use axum::http::StatusCode;
use qivxif_api::{
    ApiEnvelope, NodeCreatePayload, NodeCreateRequest, NodePayload, WorkspaceLayoutPayload,
    WorkspaceLayoutSetRequest,
};
use qivxif_core::{MetadataMap, NodeId, OperationId, Visibility};
use qivxif_graph::{NodeKind, WorkspaceLayout, WorkspaceTab, WorkspaceTile};
use qivxif_server::routes;
use support::{get, login_full, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn accepts_workspace_layout_snapshot() {
    let app = routes::router(seeded_state("workspace-layout"));
    let login = login_full(&app).await;
    let layout_node = create_layout_node(&app, &login).await;
    let pane_node = create_pane_node(&app, &login).await;
    let op_id = OperationId::generate();
    let request = WorkspaceLayoutSetRequest {
        op_id: op_id.clone(),
        actor_seq: 3,
        layout_node_id: layout_node.clone(),
        layout: layout(pane_node),
    };

    let response = app
        .clone()
        .oneshot(post_json(
            "/api/workspace/layout",
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<WorkspaceLayoutPayload> = read_json(response).await;
    let accepted = envelope.payload.unwrap();
    assert_eq!(accepted.operation.op_id, op_id);

    let duplicate = app
        .clone()
        .oneshot(post_json(
            "/api/workspace/layout",
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    let envelope: ApiEnvelope<WorkspaceLayoutPayload> = read_json(duplicate).await;
    assert_eq!(envelope.payload.unwrap().operation, accepted.operation);

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
    create_node(app, login, 1, NodeKind::WorkspaceLayout).await
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
    read_json::<ApiEnvelope<NodeCreatePayload>>(response).await;
    node_id
}

fn layout(pane_node_id: NodeId) -> WorkspaceLayout {
    WorkspaceLayout {
        maximized_pane_id: Some(pane_node_id.clone()),
        root: WorkspaceTile::Stack {
            active: 0,
            tabs: vec![WorkspaceTab {
                pane_kind: "text_editor".to_owned(),
                pane_node_id,
                target_node_id: None,
                title: "Text".to_owned(),
            }],
        },
    }
}
