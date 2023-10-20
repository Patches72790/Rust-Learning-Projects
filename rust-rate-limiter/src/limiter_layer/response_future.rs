use axum::BoxError;
use futures_core::{ready, Future};
use pin_project::pin_project;
use tokio::sync::OwnedSemaphorePermit;

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    pub response_future: F,
    pub _permit: OwnedSemaphorePermit,
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<BoxError>,
{
    type Output = Result<Response, BoxError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(ready!(self
            .project()
            .response_future
            .poll(cx)
            .map_err(Into::into)))
    }
}
