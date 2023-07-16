use hyper::{body::Body, Request, StatusCode};
use serde::Serialize;
use std::{net::SocketAddr, time::Duration};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{info, Level, Span};

use axum::{
    self,
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;

struct AppState {
    env: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let shared_state = Arc::new(AppState {
        env: "test".to_string(),
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/stuff", get(stuff))
        .route("/env", get(env))
        .route("/slow/:sleep_time", get(slow))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<Body>, _span: &Span| {
                    info!("in request {}", request.uri())
                })
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello world!"
}

async fn env(State(state): State<Arc<AppState>>) -> String {
    state.env.to_string()
}

async fn slow(Path(sleep_time): Path<usize>) -> String {
    info!("Sleeping for {} seconds", sleep_time);
    std::thread::sleep(Duration::from_millis((1000 * sleep_time) as u64));

    format!("Slept {} seconds", (1000 * sleep_time) as u64)
}

async fn stuff() -> (StatusCode, Json<Stuff>) {
    let stuff = Stuff {
        things: "thingy-things".to_string(),
    };

    (StatusCode::OK, Json(stuff))
}

#[derive(Serialize)]
struct Stuff {
    things: String,
}
