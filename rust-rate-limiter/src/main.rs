use std::{net::SocketAddr, sync::Arc, time::Duration};

use rust_rate_limiter::RateLimiter;

use axum::{
    self, extract::State, http::Request, middleware::Next, response::Response, routing::get, Router,
};
use tokio::{sync::RwLock, time::sleep};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[derive(Clone)]
pub struct AppState {
    limiter: RateLimiter,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let state = Arc::new(RwLock::new(AppState {
        limiter: RateLimiter::default(),
    }));

    let app = Router::new()
        .route("/", get(root))
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            rate_limited_middlware_fn,
        ))
        .with_state(state);

    tracing::info!("Hello on port 3000!");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Error serving application")
}

async fn rate_limited_middlware_fn<T>(
    State(state): State<Arc<RwLock<AppState>>>,
    request: Request<T>,
    next: Next<T>,
) -> Result<Response, &'static str> {
    while !state.write().await.limiter.can_process_request() {
        tracing::info!("Sleeping for rate limiter...");
        let _ = sleep(Duration::from_secs(3)).await;
    }
    let response = next.run(request).await;

    Ok(response)
}

async fn root(State(state): State<Arc<RwLock<AppState>>>) -> &'static str {
    "hello"
}
