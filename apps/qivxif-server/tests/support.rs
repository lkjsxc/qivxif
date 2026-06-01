use axum::{
    Router,
    body::{Body, to_bytes},
    http::{
        Request,
        header::{CONTENT_TYPE, COOKIE, SET_COOKIE},
    },
};
use qivxif_api::{ApiEnvelope, LoginPayload};
use qivxif_auth::hash_password;
use qivxif_server::{config::ServerConfig, state::AppState};
use qivxif_store_redb::{StoreConfig, open_or_create};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tower::ServiceExt;

pub async fn login(app: &Router) -> (String, String) {
    let response = app
        .clone()
        .oneshot(post_raw(
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
    (cookie, envelope.payload.unwrap().csrf_token)
}

pub fn seeded_state(name: &str) -> AppState {
    let root = test_dir(name);
    let database_file = root.join("qivxif.redb");
    let store = open_or_create(StoreConfig::new(&database_file)).unwrap();
    store
        .create_admin_user("admin".to_owned(), hash_password("secret").unwrap())
        .unwrap();
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

pub fn get(uri: &str, cookie: Option<&str>) -> Request<Body> {
    let mut builder = Request::builder().method("GET").uri(uri);
    if let Some(cookie) = cookie {
        builder = builder.header(COOKIE, cookie);
    }
    builder.body(Body::empty()).unwrap()
}

pub fn post_json<T: serde::Serialize>(
    uri: &str,
    body: &T,
    cookie: Option<&str>,
    csrf: Option<&str>,
) -> Request<Body> {
    post_raw(uri, &serde_json::to_string(body).unwrap(), cookie, csrf)
}

pub async fn read_json<T: serde::de::DeserializeOwned>(response: axum::response::Response) -> T {
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}

fn post_raw(uri: &str, body: &str, cookie: Option<&str>, csrf: Option<&str>) -> Request<Body> {
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
