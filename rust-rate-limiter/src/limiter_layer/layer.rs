use std::time::Duration;

use tower::Layer;

use super::{rate::Rate, service::RateLimit};

#[derive(Debug, Clone)]
pub struct RateLimitLayer {
    rate: Rate,
}

impl RateLimitLayer {
    pub fn new(num: usize, interval: Duration) -> RateLimitLayer {
        RateLimitLayer {
            rate: Rate::new(num, interval),
        }
    }
}

impl<T> Layer<T> for RateLimitLayer {
    type Service = RateLimit<T>;
    fn layer(&self, service: T) -> Self::Service {
        RateLimit::new(service, self.rate)
    }
}
