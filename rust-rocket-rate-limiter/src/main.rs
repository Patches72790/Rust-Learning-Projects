#![feature(proc_macro_hygiene, decl_macro)]

use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use rocket::http::ContentType;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Status,
    routes,
};
use serde_json::json;

#[derive(Debug)]
struct RateLimiter {
    bucket: Arc<Mutex<usize>>,
    interval: Arc<Mutex<Duration>>,
    last_sent: Arc<Mutex<SystemTime>>,
}

impl RateLimiter {
    pub fn new(bucket: usize, interval: usize) -> RateLimiter {
        assert!(interval > 0);
        RateLimiter {
            bucket: Arc::new(Mutex::new(bucket)),
            interval: Arc::new(Mutex::new(Duration::from_secs(interval as u64))),
            last_sent: Arc::new(Mutex::new(SystemTime::UNIX_EPOCH)),
        }
    }
}

impl Fairing for RateLimiter {
    fn info(&self) -> Info {
        Info {
            name: "Rate Limiter",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &rocket::Request, response: &mut rocket::Response) {
        if (self.last_sent.lock().unwrap().elapsed().unwrap()) >= *self.interval.lock().unwrap() {
            *self.bucket.lock().unwrap() = 5;
            *self.last_sent.lock().unwrap() = SystemTime::now();
        }

        if *self.bucket.lock().unwrap() >= 1 {
            *self.bucket.lock().unwrap() -= 1;
        } else {
            response.set_sized_body(std::io::Cursor::new(
                json!({"error": "Rate limited"}).to_string(),
            ));
            response.set_status(Status::TooManyRequests);
            response.set_header(ContentType::JSON);
        }
    }
}

#[rocket::get("/")]
fn hello() -> String {
    json!({"message": "hello"}).to_string()
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let r = rocket::Rocket::ignite();
    r.attach(RateLimiter::new(5, 5))
        .mount("/", routes![hello])
        .launch();

    Ok(())
}
