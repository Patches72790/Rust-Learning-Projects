use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
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
