use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures_util::future::TryFuture;
use hyper::service::Service;
use pin_project::pin_project;
use crate::reject::IsReject;
use crate::reply::{Reply, Response};
use crate::route::{Route};
use crate::{Filter, Request};




































pub fn service<F>(filter: F) -> FilteredService<F>
where
    F: Filter,
    <F::Future as TryFuture>::Ok: Reply,
    <F::Future as TryFuture>::Error: IsReject,
{
    loop {}
}
#[derive(Copy, Clone, Debug)]
pub struct FilteredService<F> {
    filter: F,
}
impl<F> FilteredService<F>
where
    F: Filter,
    <F::Future as TryFuture>::Ok: Reply,
    <F::Future as TryFuture>::Error: IsReject,
{
    #[inline]
    pub(crate) fn call_with_addr(
        &self,
        req: Request,
        remote_addr: Option<SocketAddr>,
    ) -> FilteredFuture<F::Future> {
        loop {}
    }
}
impl<F> Service<Request> for FilteredService<F>
where
    F: Filter,
    <F::Future as TryFuture>::Ok: Reply,
    <F::Future as TryFuture>::Error: IsReject,
{
    type Response = Response;
    type Error = Infallible;
    type Future = FilteredFuture<F::Future>;
    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    #[inline]
    fn call(&mut self, req: Request) -> Self::Future {
        loop {}
    }
}
#[pin_project]
#[derive(Debug)]
pub struct FilteredFuture<F> {
    #[pin]
    future: F,
    route: ::std::cell::RefCell<Route>,
}
impl<F> Future for FilteredFuture<F>
where
    F: TryFuture,
    F::Ok: Reply,
    F::Error: IsReject,
{
    type Output = Result<Response, Infallible>;
    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
