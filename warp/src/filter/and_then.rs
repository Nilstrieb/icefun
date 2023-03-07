use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures_util::{TryFuture};
use pin_project::pin_project;
use super::{Filter, FilterBase, Func, Internal};
use crate::reject::CombineRejection;
#[derive(Clone, Copy, Debug)]
pub struct AndThen<T, F> {
    pub(super) filter: T,
    pub(super) callback: F,
}
impl<T, F> FilterBase for AndThen<T, F>
where
    T: Filter,
    F: Func<T::Extract> + Clone + Send,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: CombineRejection<T::Error>,
{
    type Extract = (<F::Output as TryFuture>::Ok,);
    type Error = <<F::Output as TryFuture>::Error as CombineRejection<T::Error>>::One;
    type Future = AndThenFuture<T, F>;
    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
#[pin_project]
pub struct AndThenFuture<T, F>
where
    T: Filter,
    F: Func<T::Extract>,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: CombineRejection<T::Error>,
{
    #[pin]
    state: State<T::Future, F>,
}
#[pin_project(project = StateProj)]
enum State<T, F>
where
    T: TryFuture,
    F: Func<T::Ok>,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: CombineRejection<T::Error>,
{
    First(#[pin] T, F),
    Second(#[pin] F::Output),
    Done,
}
impl<T, F> Future for AndThenFuture<T, F>
where
    T: Filter,
    F: Func<T::Extract>,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: CombineRejection<T::Error>,
{
    type Output = Result<
        (<F::Output as TryFuture>::Ok,),
        <<F::Output as TryFuture>::Error as CombineRejection<T::Error>>::One,
    >;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
impl<T, F> Future for State<T, F>
where
    T: TryFuture,
    F: Func<T::Ok>,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: CombineRejection<T::Error>,
{
    type Output = Result<
        (<F::Output as TryFuture>::Ok,),
        <<F::Output as TryFuture>::Error as CombineRejection<T::Error>>::One,
    >;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
