use rust_rate_limiter::middleware_app;
use std::{net::SocketAddr, sync::Arc, time::Duration};

use hyper::StatusCode;
use rust_rate_limiter::{RateLimitError, RateLimitLayer};

use axum::{
    self, error_handling::HandleErrorLayer, extract::State, routing::get, BoxError, Router,
};
use serde_json::json;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[derive(Clone)]
pub struct AppState {}

pub struct AppConfig {
    rate_num: usize,
    rate_interval: Duration,

    app_state: AppState,
    log_level: Level,
}

async fn app(config: AppConfig) -> Router {
    let state = Arc::new(RwLock::new(config.app_state));
    Router::new()
        .route("/", get(root))
        .route("/test", get(test))
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(config.log_level))
                .on_response(DefaultOnResponse::new().level(config.log_level)),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    if err.is::<RateLimitError>() {
                        (
                            StatusCode::TOO_MANY_REQUESTS,
                            json!({"error": format!("Too many requests: {}", err)}).to_string(),
                        )
                    } else {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            json!({"error": format!("Server error: {}", err)}).to_string(),
                        )
                    }
                }))
                .layer(RateLimitLayer::new(config.rate_num, config.rate_interval)),
        )
        .with_state(state)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let app_config = AppConfig {
        app_state: AppState {},
        log_level: Level::INFO,
        rate_num: 5,
        rate_interval: Duration::from_secs(5),
    };

    let app = app(app_config).await;
    let middleware_app = middleware_app();

    tracing::info!("Hello on port 3000!");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        //.serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .serve(middleware_app.into_make_service())
        .await
        .expect("Error serving application")
}

async fn root(State(_state): State<Arc<RwLock<AppState>>>) -> String {
    json!({"message": "hello"}).to_string()
}

async fn test() -> &'static str {
    "test"
}

#[cfg(test)]
mod tests {
    use tokio::time::sleep;

    use super::*;

    #[tokio::test]
    async fn test_axum_router() {
        let app_config = AppConfig {
            app_state: AppState {},
            log_level: Level::INFO,
            rate_num: 5,
            rate_interval: Duration::from_secs(5),
        };

        let app = app(app_config).await.into_make_service();

        let server = axum_test::TestServer::new(app).unwrap();

        let response_text = server.get("/").await.text();

        assert!(response_text.contains("hello"))
    }

    #[tokio::test]
    async fn test_axum_router_rate_limited() {
        let app_config = AppConfig {
            app_state: AppState {},
            log_level: Level::INFO,
            rate_num: 5,
            rate_interval: Duration::from_secs(5),
        };

        let app = app(app_config).await.into_make_service();

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
        let app_config = AppConfig {
            app_state: AppState {},
            log_level: Level::INFO,
            rate_num: 5,
            rate_interval: Duration::from_secs(5),
        };

        let app = app(app_config).await.into_make_service();
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
