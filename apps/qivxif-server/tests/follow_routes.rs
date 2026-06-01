mod support;

use axum::http::StatusCode;
use qivxif_api::{
    ApiEnvelope, FeedHomePayload, FollowPayload, FollowRequest, ShortPostRequest, UnfollowRequest,
};
use qivxif_auth::{AuthRole, hash_password};
use qivxif_core::{EdgeId, NodeId, OperationId, Visibility};
use qivxif_graph::EdgeKind;
use qivxif_server::routes;
use support::{get, login_full, login_named, post_json, read_json, seeded_state};
use tower::ServiceExt;

#[tokio::test]
async fn profile_follows_drive_home_feed_markers() {
    let state = seeded_state("follow-feed");
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
    let follow = follow_request(admin.next_actor_seq, member.profile_node_id.clone());

    let response = app
        .clone()
        .oneshot(post_json(
            "/api/social/follow",
            &follow,
            Some(&admin.cookie),
            Some(&admin.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<FollowPayload> = read_json(response).await;
    let followed = envelope.payload.unwrap().edge;
    assert_eq!(followed.kind, EdgeKind::Follows);

    let post = short_post(member.next_actor_seq, "hello followers");
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

    let feed = home_feed(&app, &admin.cookie).await;
    assert_eq!(feed.items.len(), 1);
    assert_eq!(feed.items[0].author_user_id, member.user_id);

    let unfollow = UnfollowRequest {
        op_id: OperationId::generate(),
        actor_seq: admin.next_actor_seq + 1,
        edge_id: followed.id,
    };
    let response = app
        .clone()
        .oneshot(post_json(
            "/api/social/unfollow",
            &unfollow,
            Some(&admin.cookie),
            Some(&admin.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(home_feed(&app, &admin.cookie).await.items.is_empty());

    let later = short_post(member.next_actor_seq + 1, "not fanout");
    let response = app
        .clone()
        .oneshot(post_json(
            "/api/social/short-posts",
            &later,
            Some(&member.cookie),
            Some(&member.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(home_feed(&app, &admin.cookie).await.items.is_empty());
}

#[tokio::test]
async fn rejects_self_follow() {
    let app = routes::router(seeded_state("follow-self"));
    let admin = login_full(&app).await;
    let request = follow_request(admin.next_actor_seq, admin.profile_node_id);
    let response = app
        .oneshot(post_json(
            "/api/social/follow",
            &request,
            Some(&admin.cookie),
            Some(&admin.csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
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

fn follow_request(actor_seq: u64, target_profile_node_id: NodeId) -> FollowRequest {
    FollowRequest {
        op_id: OperationId::generate(),
        actor_seq,
        edge_id: EdgeId::generate(),
        target_profile_node_id,
    }
}

fn short_post(actor_seq: u64, body: &str) -> ShortPostRequest {
    ShortPostRequest {
        actor_seq,
        body: body.to_owned(),
        node_id: NodeId::generate(),
        op_id: OperationId::generate(),
        reply_to: None,
        visibility: Visibility::Public,
    }
}
