use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project::pin_project;
use super::{Filter, FilterBase, Internal, Tuple};
#[derive(Clone, Copy, Debug)]
pub struct UntupleOne<F> {
    pub(super) filter: F,
}
impl<F, T> FilterBase for UntupleOne<F>
where
    F: Filter<Extract = (T,)>,
    T: Tuple,
{
    type Extract = T;
    type Error = F::Error;
    type Future = UntupleOneFuture<F>;
    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
#[pin_project]
pub struct UntupleOneFuture<F: Filter> {
    #[pin]
    extract: F::Future,
}
impl<F, T> Future for UntupleOneFuture<F>
where
    F: Filter<Extract = (T,)>,
    T: Tuple,
{
    type Output = Result<T, F::Error>;
    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
