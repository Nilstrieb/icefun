use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project::pin_project;
use super::{Combine, Filter, FilterBase, Internal, Tuple};
use crate::generic::CombinedTuples;
use crate::reject::CombineRejection;
#[derive(Clone, Copy, Debug)]
pub struct And<T, U> {
    pub(super) first: T,
    pub(super) second: U,
}
impl<T, U> FilterBase for And<T, U>
where
    T: Filter,
    T::Extract: Send,
    U: Filter + Clone + Send,
    <T::Extract as Tuple>::HList: Combine<<U::Extract as Tuple>::HList> + Send,
    CombinedTuples<T::Extract, U::Extract>: Send,
    U::Error: CombineRejection<T::Error>,
{
    type Extract = CombinedTuples<T::Extract, U::Extract>;
    type Error = <U::Error as CombineRejection<T::Error>>::One;
    type Future = AndFuture<T, U>;
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
#[pin_project]
pub struct AndFuture<T: Filter, U: Filter> {
    #[pin]
    state: State<T::Future, T::Extract, U>,
}
#[pin_project(project = StateProj)]
enum State<T, TE, U: Filter> {
    First(#[pin] T, U),
    Second(Option<TE>, #[pin] U::Future),
    Done,
}
impl<T, U> Future for AndFuture<T, U>
where
    T: Filter,
    U: Filter,
    <T::Extract as Tuple>::HList: Combine<<U::Extract as Tuple>::HList> + Send,
    U::Error: CombineRejection<T::Error>,
{
    type Output = Result<
        CombinedTuples<T::Extract, U::Extract>,
        <U::Error as CombineRejection<T::Error>>::One,
    >;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
impl<T, TE, U, E> Future for State<T, TE, U>
where
    T: Future<Output = Result<TE, E>>,
    U: Filter,
    TE: Tuple,
    TE::HList: Combine<<U::Extract as Tuple>::HList> + Send,
    U::Error: CombineRejection<E>,
{
    type Output = Result<
        CombinedTuples<TE, U::Extract>,
        <U::Error as CombineRejection<E>>::One,
    >;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
