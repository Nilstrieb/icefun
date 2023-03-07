use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project::pin_project;
use super::{Filter, FilterBase, Internal};
use crate::generic::Either;
use crate::reject::CombineRejection;

type Combined<E1, E2> = <E1 as CombineRejection<E2>>::Combined;
#[derive(Clone, Copy, Debug)]
pub struct Or<T, U> {
    pub(super) first: T,
    pub(super) second: U,
}
impl<T, U> FilterBase for Or<T, U>
where
    T: Filter,
    U: Filter + Clone + Send,
    U::Error: CombineRejection<T::Error>,
{
    type Extract = (Either<T::Extract, U::Extract>,);
    type Error = Combined<U::Error, T::Error>;
    type Future = EitherFuture<T, U>;
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
#[pin_project]
pub struct EitherFuture<T: Filter, U: Filter> {
    #[pin]
    state: State<T, U>,
    original_path_index: PathIndex,
}
#[pin_project(project = StateProj)]
enum State<T: Filter, U: Filter> {
    First(#[pin] T::Future, U),
    Second(Option<T::Error>, #[pin] U::Future),
    Done,
}
#[derive(Copy, Clone)]
struct PathIndex(usize);
impl PathIndex {
    fn reset_path(&self) {
        loop {}
    }
}
impl<T, U> Future for EitherFuture<T, U>
where
    T: Filter,
    U: Filter,
    U::Error: CombineRejection<T::Error>,
{
    type Output = Result<
        (Either<T::Extract, U::Extract>,),
        Combined<U::Error, T::Error>,
    >;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
