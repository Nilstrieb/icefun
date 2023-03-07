use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project::pin_project;
use super::{Filter, FilterBase, Func, Internal};
#[derive(Clone, Copy, Debug)]
pub struct Map<T, F> {
    pub(super) filter: T,
    pub(super) callback: F,
}
impl<T, F> FilterBase for Map<T, F>
where
    T: Filter,
    F: Func<T::Extract> + Clone + Send,
{
    type Extract = (F::Output,);
    type Error = T::Error;
    type Future = MapFuture<T, F>;
    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
#[pin_project]
pub struct MapFuture<T: Filter, F> {
    #[pin]
    extract: T::Future,
    callback: F,
}
impl<T, F> Future for MapFuture<T, F>
where
    T: Filter,
    F: Func<T::Extract>,
{
    type Output = Result<(F::Output,), T::Error>;
    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
