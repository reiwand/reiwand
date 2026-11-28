use super::open;
use std::future::{Future, ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tower_service::Service;

type BoxError = Box<dyn std::error::Error + Send + Sync>;
type Answer<T> = Pin<Box<dyn Future<Output = Result<T, BoxError>> + Send>>;

const IDLE: Duration = Duration::from_secs(15);

#[derive(Clone, Copy)]
pub struct Gate;

#[derive(Clone)]
pub struct Gated<S>(S);

impl<S> tower_layer::Layer<S> for Gate {
    type Service = Gated<S>;

    fn layer(&self, inner: S) -> Gated<S> {
        Gated(inner)
    }
}

impl<S, R> Service<R> for Gated<S>
where
    S: Service<R, Error = BoxError>,
    S::Future: Send + 'static,
    S::Response: Send + 'static,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = Answer<S::Response>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), BoxError>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, request: R) -> Self::Future {
        match open() {
            Err(refused) => Box::pin(ready(Err(refused.into()))),
            Ok(()) => Box::pin(self.0.call(request)),
        }
    }
}

pub fn client() -> reqwest::ClientBuilder {
    reqwest::Client::builder().connector_layer(Gate).pool_idle_timeout(IDLE)
}

pub fn blocking() -> reqwest::blocking::ClientBuilder {
    reqwest::blocking::Client::builder().connector_layer(Gate).pool_idle_timeout(IDLE)
}
