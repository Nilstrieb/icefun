use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures_util::{TryFuture};
use pin_project::pin_project;
use super::{Filter, FilterBase, Func, Internal};
use crate::generic::Either;
use crate::reject::IsReject;

#[derive(Clone, Copy, Debug)]
pub struct Recover<T, F> {
    pub(super) filter: T,
    pub(super) callback: F,
}
impl<T, F> FilterBase for Recover<T, F>
where
    T: Filter,
    F: Func<T::Error> + Clone + Send,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: IsReject,
{
    type Extract = (Either<T::Extract, (<F::Output as TryFuture>::Ok,)>,);
    type Error = <F::Output as TryFuture>::Error;
    type Future = RecoverFuture<T, F>;
    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
#[pin_project]
pub struct RecoverFuture<T: Filter, F>
where
    T: Filter,
    F: Func<T::Error>,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: IsReject,
{
    #[pin]
    state: State<T, F>,
    original_path_index: PathIndex,
}
#[pin_project(project = StateProj)]
enum State<T, F>
where
    T: Filter,
    F: Func<T::Error>,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: IsReject,
{
    First(#[pin] T::Future, F),
    Second(#[pin] F::Output),
    Done,
}
#[derive(Copy, Clone)]
struct PathIndex(usize);
impl PathIndex {
    fn reset_path(&self) {
        loop {}
    }
}
impl<T, F> Future for RecoverFuture<T, F>
where
    T: Filter,
    F: Func<T::Error>,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: IsReject,
{
    type Output = Result<
        (Either<T::Extract, (<F::Output as TryFuture>::Ok,)>,),
        <F::Output as TryFuture>::Error,
    >;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
