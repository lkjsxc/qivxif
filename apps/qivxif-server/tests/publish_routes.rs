mod support;

use axum::{
    body::to_bytes,
    http::{Response, StatusCode},
};
use qivxif_api::{
    ApiEnvelope, NodeCreatePayload, NodeCreateRequest, PublishPayload, PublishRequest,
};
use qivxif_core::{MetadataMap, NodeId, OperationId, TextDocId, Visibility};
use qivxif_graph::NodeKind;
use qivxif_history::text::{TextEdit, TextOperation, TextRestore};
use qivxif_server::routes;
use support::{get, login_full, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn publishes_and_unpublishes_blog_post() {
    let app = routes::router(seeded_state("publish-blog"));
    let login = login_full(&app).await;
    let body = create_text_body(&app, &login, 1, "# Hello\n\nBody <safe>").await;
    let post = create_blog_post(&app, &login, 3, &body, "Hello").await;

    let publish = publish(&app, &login, 4, &post, "hello").await;
    assert_eq!(publish.status(), StatusCode::OK);
    let envelope: ApiEnvelope<PublishPayload> = read_json(publish).await;
    assert_eq!(
        envelope.payload.unwrap().post.visibility,
        Visibility::Public
    );

    let public = app
        .clone()
        .oneshot(get("/@admin/hello", None))
        .await
        .unwrap();
    assert_eq!(public.status(), StatusCode::OK);
    let html = read_text(public).await;
    assert!(html.contains("<h1>Hello</h1>"));
    assert!(html.contains("Body &lt;safe&gt;"));

    let unpublish = serde_json::json!({
        "op_id": OperationId::generate(),
        "actor_seq": 5_u64,
        "reason": "draft correction"
    });
    let response = app
        .clone()
        .oneshot(post_json(
            &format!("/api/unpublish/{post}"),
            &unpublish,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let public = app.oneshot(get("/@admin/hello", None)).await.unwrap();
    assert_eq!(public.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn rejects_duplicate_published_slug_for_author() {
    let app = routes::router(seeded_state("publish-conflict"));
    let login = login_full(&app).await;
    let first_body = create_text_body(&app, &login, 1, "first").await;
    let first = create_blog_post(&app, &login, 3, &first_body, "First").await;
    assert_eq!(
        publish(&app, &login, 4, &first, "same").await.status(),
        StatusCode::OK
    );

    let second_body = create_text_body(&app, &login, 5, "second").await;
    let second = create_blog_post(&app, &login, 7, &second_body, "Second").await;
    let conflict = publish(&app, &login, 8, &second, "same").await;
    assert_eq!(conflict.status(), StatusCode::CONFLICT);
}

async fn create_text_body(
    app: &axum::Router,
    login: &support::TestLogin,
    actor_seq: u64,
    content: &str,
) -> NodeId {
    let node_id = create_node(app, login, actor_seq, NodeKind::Text, MetadataMap::empty()).await;
    let request = serde_json::json!({
        "actor_seq": actor_seq + 1,
        "operation": TextOperation {
            op_id: OperationId::generate(),
            doc_id: TextDocId::generate(),
            edit: TextEdit::Restore(TextRestore {
                content: content.to_owned(),
                actor_id: login.actor_id.clone(),
                first_seq: actor_seq * 1000,
            }),
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
    node_id
}

async fn create_blog_post(
    app: &axum::Router,
    login: &support::TestLogin,
    actor_seq: u64,
    body_node_id: &NodeId,
    title: &str,
) -> NodeId {
    let mut metadata = MetadataMap::empty();
    metadata.insert("body_node_id", body_node_id.to_string());
    metadata.insert("title", title);
    metadata.insert("publication_state", "draft");
    create_node(app, login, actor_seq, NodeKind::BlogPost, metadata).await
}

async fn create_node(
    app: &axum::Router,
    login: &support::TestLogin,
    actor_seq: u64,
    kind: NodeKind,
    metadata_map: MetadataMap,
) -> NodeId {
    let node_id = NodeId::generate();
    let request = NodeCreateRequest {
        op_id: OperationId::generate(),
        actor_seq,
        node_id: node_id.clone(),
        kind,
        visibility: Visibility::Private,
        metadata_map,
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

async fn publish(
    app: &axum::Router,
    login: &support::TestLogin,
    actor_seq: u64,
    post: &NodeId,
    slug: &str,
) -> Response<axum::body::Body> {
    let request = PublishRequest {
        op_id: OperationId::generate(),
        actor_seq,
        slug: slug.to_owned(),
        summary: "summary".to_owned(),
    };
    app.clone()
        .oneshot(post_json(
            &format!("/api/publish/{post}"),
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap()
}

async fn read_text(response: Response<axum::body::Body>) -> String {
    String::from_utf8(
        to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap()
}
