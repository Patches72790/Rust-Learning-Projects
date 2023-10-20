use std::{net::SocketAddr, sync::Arc, time::Duration};

use hyper::StatusCode;
use rust_rate_limiter::{RateLimitError, RateLimitLayer};

use axum::{
    self, error_handling::HandleErrorLayer, extract::State, routing::get, BoxError, Router,
};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[derive(Clone)]
pub struct AppState {}

async fn app() -> Router {
    let state = Arc::new(RwLock::new(AppState {}));
    Router::new()
        .route("/", get(root))
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    if err.is::<RateLimitError>() {
                        (
                            StatusCode::TOO_MANY_REQUESTS,
                            format!("Too many requests: {}", err),
                        )
                    } else {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unknown error: {}", err),
                        )
                    }
                }))
                .layer(RateLimitLayer::new(5, Duration::from_secs(5))),
        )
        .with_state(state)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let app = app().await;

    tracing::info!("Hello on port 3000!");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Error serving application")
}

async fn root(State(_state): State<Arc<RwLock<AppState>>>) -> &'static str {
    "hello"
}

#[cfg(test)]
mod tests {
    use tokio::time::sleep;

    use super::*;

    #[tokio::test]
    async fn test_axum_router() {
        let app = app().await.into_make_service();

        let server = axum_test::TestServer::new(app).unwrap();

        let response_text = server.get("/").await.text();

        assert_eq!(response_text, "hello")
    }

    #[tokio::test]
    async fn test_axum_router_rate_limited() {
        let app = app().await.into_make_service();

        let server = axum_test::TestServer::new(app).unwrap();

        let mut response_texts: Vec<String> = vec![];

        for _ in 0..10 {
            response_texts.push(server.get("/").await.text());
        }

        let rate_limited = response_texts
            .iter()
            .filter(|s| s.contains("Too many requests"))
            .collect::<Vec<&String>>();

        let successful_resp = response_texts
            .iter()
            .filter(|s| !s.contains("Too many requests"))
            .collect::<Vec<&String>>();

        assert_eq!(rate_limited.len(), 5);
        assert_eq!(successful_resp.len(), 5);
        println!("{:?}", response_texts);
    }

    #[tokio::test]
    async fn test_axum_router_rate_limited_with_wait() {
        let app = app().await.into_make_service();
        let server = axum_test::TestServer::new(app).unwrap();
        let mut response_texts: Vec<String> = vec![];

        for i in 0..15 {
            if i == 10 {
                println!("sleeping for wait limiter...");
                sleep(Duration::from_secs(5)).await;
            }

            response_texts.push(server.get("/").await.text());
        }

        let rate_limited = response_texts
            .iter()
            .filter(|s| s.contains("Too many requests"))
            .collect::<Vec<&String>>();

        let successful_resp = response_texts
            .iter()
            .filter(|s| !s.contains("Too many requests"))
            .collect::<Vec<&String>>();

        assert_eq!(rate_limited.len(), 5);
        assert_eq!(successful_resp.len(), 10);
    }
}
