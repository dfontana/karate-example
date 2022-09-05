use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use redis::Commands;
use serde::Deserialize;
use std::{env, net::SocketAddr, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    cache: redis::Client,
    http: reqwest::Client,
    http_host: String,
}

#[tokio::main]
async fn main() {
    // Tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").expect("Must set RUST_LOG"),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Cache setup
    let cache_host = env::var("CACHE_HOST").expect("Must set CACHE_HOST");
    let cache =
        redis::Client::open(format!("redis://{}", cache_host)).expect("Failed to connect to Cache");

    // HTTP client setup
    let http = reqwest::Client::new();
    let http_host = env::var("HTTP_HOST").expect("Must set HTTP_HOST");

    // State object to share between routes
    let shared_state = Arc::new(AppState {
        cache,
        http,
        http_host,
    });

    // Axum setup
    let app = Router::new()
        .route("/:key", get(root))
        .layer(Extension(shared_state));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start HTTP Server");
}

#[derive(Deserialize)]
struct CanDoResponse {
    can_do: bool,
}

async fn root(state: Extension<Arc<AppState>>, Path(key): Path<String>) -> impl IntoResponse {
    //Hit HTTP endpoint to determine if we can continue
    let res = state
        .http
        .get(format!("http://{}/can-do/{}", state.http_host, key))
        .send()
        .await;
    let can_do = match res {
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
        Ok(v) => v.json::<CanDoResponse>().await.unwrap().can_do,
    };
    if !can_do {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized for Resource".into()));
    }

    // Assuming we can, hit the cache dependency
    state
        .cache
        .get_connection()
        .and_then(|mut c| c.get("myTest"))
        .map(|v: i64| {
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/json")],
                format!("{}", v),
            )
        })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))
}
