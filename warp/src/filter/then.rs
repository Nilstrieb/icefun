use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures_util::{TryFuture};
use pin_project::pin_project;
use super::{Filter, FilterBase, Func, Internal};
#[derive(Clone, Copy, Debug)]
pub struct Then<T, F> {
    pub(super) filter: T,
    pub(super) callback: F,
}
impl<T, F> FilterBase for Then<T, F>
where
    T: Filter,
    F: Func<T::Extract> + Clone + Send,
    F::Output: Future + Send,
{
    type Extract = (<F::Output as Future>::Output,);
    type Error = T::Error;
    type Future = ThenFuture<T, F>;
    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
#[pin_project]
pub struct ThenFuture<T, F>
where
    T: Filter,
    F: Func<T::Extract>,
    F::Output: Future + Send,
{
    #[pin]
    state: State<T::Future, F>,
}
#[pin_project(project = StateProj)]
enum State<T, F>
where
    T: TryFuture,
    F: Func<T::Ok>,
    F::Output: Future + Send,
{
    First(#[pin] T, F),
    Second(#[pin] F::Output),
    Done,
}
impl<T, F> Future for ThenFuture<T, F>
where
    T: Filter,
    F: Func<T::Extract>,
    F::Output: Future + Send,
{
    type Output = Result<(<F::Output as Future>::Output,), T::Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
impl<T, F> Future for State<T, F>
where
    T: TryFuture,
    F: Func<T::Ok>,
    F::Output: Future + Send,
{
    type Output = Result<(<F::Output as Future>::Output,), T::Error>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
