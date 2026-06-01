mod support;

use axum::http::StatusCode;
use qivxif_api::{
    ApiEnvelope, FeedHomePayload, FollowRequest, ModerationClearRequest, ModerationPayload,
    ModerationRequest, ShortPostPayload, ShortPostRequest,
};
use qivxif_auth::{AuthRole, hash_password};
use qivxif_core::{EdgeId, EventId, NodeId, Visibility};
use qivxif_graph::EdgeKind;
use qivxif_server::routes;
use support::{get, login_full, login_named, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn mute_filters_home_feed_without_removing_markers() {
    let app = app_with_member("mute-feed");
    let admin = login_full(&app).await;
    let member = login_named(&app, "member", "member-secret").await;
    follow(&app, &admin, &member.profile_node_id, admin.next_actor_seq).await;

    let post = short_post(member.next_actor_seq, "visible before mute", None);
    let response = app
        .clone()
        .oneshot(post_json(
            "/api/social/short-posts",
            &post,
            Some(&member.cookie),
            Some(&member.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(home_feed(&app, &admin.cookie).await.items.len(), 1);

    let muted = moderation(
        &app,
        "/api/social/mute",
        admin.next_actor_seq + 1,
        &member.profile_node_id,
        &admin.cookie,
        &admin.csrf,
    )
    .await;
    assert_eq!(muted.kind, EdgeKind::Mutes);
    assert!(home_feed(&app, &admin.cookie).await.items.is_empty());

    let clear = ModerationClearRequest {
        event_id: EventId::generate(),
        actor_seq: admin.next_actor_seq + 2,
        edge_id: muted.id,
    };
    let response = app
        .clone()
        .oneshot(post_json(
            "/api/social/unmute",
            &clear,
            Some(&admin.cookie),
            Some(&admin.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(home_feed(&app, &admin.cookie).await.items.len(), 1);
}

#[tokio::test]
async fn block_prevents_replies_between_profiles() {
    let app = app_with_member("block-reply");
    let admin = login_full(&app).await;
    let member = login_named(&app, "member", "member-secret").await;
    let post = short_post(admin.next_actor_seq, "admin post", None);
    let response = app
        .clone()
        .oneshot(post_json(
            "/api/social/short-posts",
            &post,
            Some(&admin.cookie),
            Some(&admin.csrf),
        ))
        .await
        .unwrap();
    let envelope: ApiEnvelope<ShortPostPayload> = read_json(response).await;
    let post_node_id = envelope.payload.unwrap().post.id;
    let blocked = moderation(
        &app,
        "/api/social/block",
        admin.next_actor_seq + 1,
        &member.profile_node_id,
        &admin.cookie,
        &admin.csrf,
    )
    .await;
    assert_eq!(blocked.kind, EdgeKind::Blocks);

    let reply = short_post(member.next_actor_seq, "blocked reply", Some(post_node_id));
    let response = app
        .oneshot(post_json(
            "/api/social/short-posts",
            &reply,
            Some(&member.cookie),
            Some(&member.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

fn app_with_member(name: &str) -> axum::Router {
    let state = seeded_state(name);
    state
        .store
        .create_user(
            "member".to_owned(),
            hash_password("member-secret").unwrap(),
            vec![AuthRole::Member],
        )
        .unwrap();
    routes::router(state)
}

async fn follow(
    app: &axum::Router,
    login: &support::TestLogin,
    target_profile_node_id: &NodeId,
    actor_seq: u64,
) {
    let request = FollowRequest {
        event_id: EventId::generate(),
        actor_seq,
        edge_id: EdgeId::generate(),
        target_profile_node_id: target_profile_node_id.clone(),
    };
    let response = app
        .clone()
        .oneshot(post_json(
            "/api/social/follow",
            &request,
            Some(&login.cookie),
            Some(&login.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

async fn moderation(
    app: &axum::Router,
    path: &str,
    actor_seq: u64,
    target_profile_node_id: &NodeId,
    cookie: &str,
    csrf: &str,
) -> qivxif_graph::EdgeRecord {
    let request = ModerationRequest {
        event_id: EventId::generate(),
        actor_seq,
        edge_id: EdgeId::generate(),
        target_profile_node_id: target_profile_node_id.clone(),
    };
    let response = app
        .clone()
        .oneshot(post_json(path, &request, Some(cookie), Some(csrf)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    read_json::<ApiEnvelope<ModerationPayload>>(response)
        .await
        .payload
        .unwrap()
        .edge
}

async fn home_feed(app: &axum::Router, cookie: &str) -> FeedHomePayload {
    let response = app
        .clone()
        .oneshot(get("/api/feed/home?limit=10", Some(cookie)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    read_json::<ApiEnvelope<FeedHomePayload>>(response)
        .await
        .payload
        .unwrap()
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
