use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::BoxError;
use futures_core::Future;
use pin_project::pin_project;
use tokio::time::Sleep;

use crate::RateLimitError;

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    pub(crate) response_future: F,

    #[pin]
    pub(crate) sleep: Sleep,
}

impl<T> ResponseFuture<T> {
    pub(crate) fn new(response: T, sleep: Sleep) -> Self {
        ResponseFuture {
            response_future: response,
            sleep,
        }
    }
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<BoxError>,
{
    type Output = Result<Response, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.response_future.poll(cx) {
            Poll::Ready(v) => return Poll::Ready(v.map_err(Into::into)),
            Poll::Pending => {}
        };

        match this.sleep.poll(cx) {
            Poll::Ready(_) => Poll::Ready(Err(Box::new(RateLimitError(())))),
            Poll::Pending => Poll::Pending,
        }
    }
}
