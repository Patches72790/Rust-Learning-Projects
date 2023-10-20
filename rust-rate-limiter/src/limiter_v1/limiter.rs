use std::time::{Duration, SystemTime};

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

//pub async fn rate_limited_middlware_fn<T>(
//    State(state): State<Arc<RwLock<AppState>>>,
//    request: Request<T>,
//    next: Next<T>,
//) -> Result<Response, &'static str> {
//    while !state.write().await.limiter.can_process_request() {
//        tracing::info!("Sleeping for rate limiter...");
//        let _ = sleep(Duration::from_secs(3)).await;
//    }
//    let response = next.run(request).await;
//
//    Ok(response)
//}
