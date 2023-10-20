use axum::BoxError;
use std::{
    sync::{Arc, Mutex},
    task::Poll,
};

use futures_core::ready;
use tokio::{
    sync::{OwnedSemaphorePermit, Semaphore},
    time::Instant,
};
use tokio_util::sync::PollSemaphore;
use tower_service::Service;

use super::{rate::Rate, rate_limit_error::RateLimitError, response_future::ResponseFuture};

#[derive(Debug)]
pub struct RateLimit<T> {
    inner: T,
    rate: Rate,

    last_refresh: Arc<Mutex<Instant>>,
    bucket: Arc<Mutex<usize>>,

    permit_semaphore: PollSemaphore,
    permit: Option<OwnedSemaphorePermit>,
}

impl<T> RateLimit<T> {
    pub fn new(service: T, rate: Rate) -> Self {
        RateLimit {
            inner: service,
            rate,
            last_refresh: Arc::new(Mutex::new(Instant::now())),
            bucket: Arc::new(Mutex::new(rate.num())),
            permit_semaphore: PollSemaphore::new(Arc::new(Semaphore::new(rate.num()))),
            permit: None,
        }
    }

    fn refresh_tokens(&mut self) {
        let mut bucket_lock = self.bucket.lock().unwrap();
        let mut last_refresh_lock = self.last_refresh.lock().unwrap();

        if last_refresh_lock.elapsed() > self.rate.interval() {
            *bucket_lock = self.rate.num();
            *last_refresh_lock = Instant::now();
        }
    }
}

impl<S, Request> Service<Request> for RateLimit<S>
where
    S: Service<Request>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        if self.permit.is_none() {
            self.refresh_tokens();

            let mut bucket = self.bucket.lock().unwrap();
            if *bucket > 0 {
                self.permit = ready!(self.permit_semaphore.poll_acquire(cx));
                *bucket -= 1;
            } else {
                return Poll::Ready(Err(Box::new(RateLimitError(()))));
            }
        }

        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let permit = self
            .permit
            .take()
            .expect("Max requests reached permit error");

        let future = self.inner.call(req);

        ResponseFuture {
            response_future: future,
            _permit: permit,
        }
    }
}
