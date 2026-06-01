mod support;

use axum::http::StatusCode;
use qivxif_api::{ApiEnvelope, FeedHomePayload, ShortPostPayload, ShortPostRequest};
use qivxif_core::{EventId, NodeId, Visibility};
use qivxif_server::routes;
use support::{get, login_full, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn creates_short_posts_and_indexes_home_feed() {
    let app = routes::router(seeded_state("social-feed"));
    let login = login_full(&app).await;
    let first = short_post(1, "hello graph", None);
    let response = app
        .clone()
        .oneshot(post_json(
            "/api/social/short-posts",
            &first,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<ShortPostPayload> = read_json(response).await;
    let accepted = envelope.payload.unwrap();
    assert_eq!(accepted.post.id, first.node_id);

    let duplicate = app
        .clone()
        .oneshot(post_json(
            "/api/social/short-posts",
            &first,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    let envelope: ApiEnvelope<ShortPostPayload> = read_json(duplicate).await;
    assert_eq!(envelope.payload.unwrap().event, accepted.event);

    let reply = short_post(2, "reply body", Some(first.node_id.clone()));
    let response = app
        .clone()
        .oneshot(post_json(
            "/api/social/short-posts",
            &reply,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let feed = app
        .clone()
        .oneshot(get("/api/feed/home?limit=10", Some(&login.cookie)))
        .await
        .unwrap();
    let envelope: ApiEnvelope<FeedHomePayload> = read_json(feed).await;
    let items = envelope.payload.unwrap().items;
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].body, "reply body");
    assert_eq!(items[0].reply_to, Some(first.node_id));
}

#[tokio::test]
async fn rejects_empty_short_post_body() {
    let app = routes::router(seeded_state("social-empty"));
    let login = login_full(&app).await;
    let response = app
        .oneshot(post_json(
            "/api/social/short-posts",
            &short_post(1, "", None),
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

fn short_post(actor_seq: u64, body: &str, reply_to: Option<NodeId>) -> ShortPostRequest {
    ShortPostRequest {
        actor_seq,
        body: body.to_owned(),
        node_id: NodeId::generate(),
        event_id: EventId::generate(),
        reply_to,
        visibility: Visibility::Public,
    }
}
