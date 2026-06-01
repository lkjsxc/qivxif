use axum::{
    body::{Body, to_bytes},
    http::{
        Request, StatusCode,
        header::{CONTENT_TYPE, COOKIE, SET_COOKIE},
    },
};
use qivxif_api::{ApiEnvelope, LoginPayload, LogoutPayload, MePayload};
use qivxif_auth::hash_password;
use qivxif_server::{config::ServerConfig, routes, state::AppState};
use qivxif_store_redb::{StoreConfig, open_or_create};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tower::ServiceExt;

#[tokio::test]
async fn login_sets_cookie_and_me_reads_session() {
    let state = seeded_state("login_me");
    let app = routes::router(state);
    let login = post_json(
        "/api/auth/login",
        r#"{"name":"admin","password":"secret"}"#,
        None,
        None,
    );
    let response = app.clone().oneshot(login).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let cookie = response
        .headers()
        .get(SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    assert!(cookie.contains("HttpOnly"));
    let envelope: ApiEnvelope<LoginPayload> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().user.name, "admin");

    let response = app.oneshot(get("/api/me", Some(&cookie))).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let envelope: ApiEnvelope<MePayload> = read_json(response).await;
    assert_eq!(envelope.payload.unwrap().user.roles, ["owner", "admin"]);
}

#[tokio::test]
async fn login_rejects_bad_password() {
    let app = routes::router(seeded_state("bad_password"));
    let response = app
        .oneshot(post_json(
            "/api/auth/login",
            r#"{"name":"admin","password":"wrong"}"#,
            None,
            None,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let envelope: ApiEnvelope<LoginPayload> = read_json(response).await;
    assert!(envelope.error.is_some());
}

#[tokio::test]
async fn logout_requires_csrf_and_clears_session() {
    let app = routes::router(seeded_state("logout"));
    let response = app
        .clone()
        .oneshot(post_json(
            "/api/auth/login",
            r#"{"name":"admin","password":"secret"}"#,
            None,
            None,
        ))
        .await
        .unwrap();
    let cookie = response
        .headers()
        .get(SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let envelope: ApiEnvelope<LoginPayload> = read_json(response).await;
    let csrf = envelope.payload.unwrap().csrf_token;

    let missing = app
        .clone()
        .oneshot(post_json("/api/auth/logout", "{}", Some(&cookie), None))
        .await
        .unwrap();
    assert_eq!(missing.status(), StatusCode::FORBIDDEN);

    let response = app
        .oneshot(post_json(
            "/api/auth/logout",
            "{}",
            Some(&cookie),
            Some(&csrf),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response
            .headers()
            .get(SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap()
            .contains("Max-Age=0")
    );
    let envelope: ApiEnvelope<LogoutPayload> = read_json(response).await;
    assert!(envelope.payload.unwrap().logged_out);
}

fn seeded_state(name: &str) -> AppState {
    let root = test_dir(name);
    let database_file = root.join("qivxif.redb");
    let store = open_or_create(StoreConfig::new(&database_file)).unwrap();
    let hash = hash_password("secret").unwrap();
    store.create_admin_user("admin".to_owned(), hash).unwrap();
    AppState {
        config: ServerConfig {
            bind: "127.0.0.1:0".parse().unwrap(),
            data_dir: root.clone(),
            database_file,
            static_dir: root,
            cookie_secure: false,
        },
        store,
    }
}

fn test_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("qivxif-{name}-{}-{nanos}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

fn get(uri: &str, cookie: Option<&str>) -> Request<Body> {
    let mut builder = Request::builder().method("GET").uri(uri);
    if let Some(cookie) = cookie {
        builder = builder.header(COOKIE, cookie);
    }
    builder.body(Body::empty()).unwrap()
}

fn post_json(uri: &str, body: &str, cookie: Option<&str>, csrf: Option<&str>) -> Request<Body> {
    let mut builder = Request::builder()
        .method("POST")
        .uri(uri)
        .header(CONTENT_TYPE, "application/json");
    if let Some(cookie) = cookie {
        builder = builder.header(COOKIE, cookie);
    }
    if let Some(csrf) = csrf {
        builder = builder.header("x-qivxif-csrf", csrf);
    }
    builder.body(Body::from(body.to_owned())).unwrap()
}

async fn read_json<T>(response: axum::response::Response) -> T
where
    T: serde::de::DeserializeOwned,
{
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}
