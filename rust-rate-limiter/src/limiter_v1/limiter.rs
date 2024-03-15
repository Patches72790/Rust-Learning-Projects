use std::{
    sync::{Arc, RwLock},
    time::{Duration, SystemTime},
};

use axum::{
    body::HttpBody,
    extract::State,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use hyper::Request;
use serde_json::json;
use tokio::time::sleep;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RateLimiter {
    bucket: usize,
    last_sent: SystemTime,
    interval: Duration,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self {
            bucket: 0,
            last_sent: SystemTime::UNIX_EPOCH,
            interval: Duration::from_secs(5),
        }
    }
}

#[allow(dead_code)]
impl RateLimiter {
    pub fn new(bucket: usize, interval: usize) -> RateLimiter {
        assert!(interval > 0);
        RateLimiter {
            bucket,
            last_sent: SystemTime::UNIX_EPOCH,
            interval: Duration::from_secs(interval as u64),
        }
    }

    pub fn can_process_request(&mut self) -> bool {
        if (self.last_sent.elapsed().unwrap()) >= self.interval {
            self.bucket = 5;
            self.last_sent = SystemTime::now();
        }
        if self.bucket >= 1 {
            self.bucket -= 1;
            true
        } else {
            false
        }
    }
}

#[allow(dead_code)]
pub struct AppState {
    pub(crate) limiter: RateLimiter,
}

#[allow(dead_code)]
pub async fn rate_limited_middlware_fn<T: HttpBody>(
    State(state): State<Arc<RwLock<AppState>>>,
    request: Request<T>,
    next: Next<T>,
) -> Result<Response, &'static str> {
    while !state.write().unwrap().limiter.can_process_request() {
        tracing::info!("Sleeping for rate limiter...");
        let _ = sleep(Duration::from_secs(3)).await;
    }

    let response = next.run(request).await;

    Ok(response)
}

pub fn middleware_app() -> Router {
    let state = Arc::new(RwLock::new(AppState {
        limiter: RateLimiter::new(5, 5),
    }));
    Router::new()
        .route(
            "/",
            get(|| async { json!({"message": "hello"}).to_string() }),
        )
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            rate_limited_middlware_fn,
        ))
        .with_state(state)
}
