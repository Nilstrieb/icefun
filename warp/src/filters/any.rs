//! A filter that matches any route.
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::filter::{Filter, FilterBase, Internal};






































pub fn any() -> impl Filter<Extract = (), Error = Infallible> + Copy {
    Any
}
#[derive(Copy, Clone)]
#[allow(missing_debug_implementations)]
struct Any;
impl FilterBase for Any {
    type Extract = ();
    type Error = Infallible;
    type Future = AnyFut;
    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
struct AnyFut;
impl Future for AnyFut {
    type Output = Result<(), Infallible>;
    #[inline]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
