mod and;
mod and_then;
mod boxed;
mod map;
mod map_err;
mod or;
mod or_else;
mod recover;
pub(crate) mod service;
mod then;
mod unify;
mod untuple_one;
mod wrap;
pub(crate) use self::and::And;
use self::and_then::AndThen;
pub use self::boxed::BoxedFilter;
pub(crate) use self::map::Map;
pub(crate) use self::map_err::MapErr;
pub(crate) use self::or::Or;
use self::or_else::OrElse;
use self::recover::Recover;
use self::then::Then;
use self::unify::Unify;
use self::untuple_one::UntupleOne;
pub use self::wrap::wrap_fn;
pub(crate) use self::wrap::{Wrap, WrapSealed};
pub(crate) use crate::generic::{one, Combine, Either, Func, One, Tuple};
use crate::reject::{CombineRejection, IsReject, Rejection};
use crate::route::Route;
use futures_util::{future, TryFuture, TryFutureExt};
use std::future::Future;
pub trait FilterBase {
    type Extract: Tuple;
    type Error: IsReject;
    type Future: Future<Output = Result<Self::Extract, Self::Error>> + Send;
    fn filter(&self, internal: Internal) -> Self::Future;
    fn map_err<F, E>(self, _internal: Internal, fun: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Error) -> E + Clone,
        E: ::std::fmt::Debug + Send,
    {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
pub struct Internal;

pub trait Filter: FilterBase {
    fn and<F>(self, other: F) -> And<Self, F>
    where
        Self: Sized,
        <Self::Extract as Tuple>::HList: Combine<<F::Extract as Tuple>::HList>,
        F: Filter + Clone,
        F::Error: CombineRejection<Self::Error>,
    {
        loop {}
    }

    fn or<F>(self, other: F) -> Or<Self, F>
    where
        Self: Filter<Error = Rejection> + Sized,
        F: Filter,
        F::Error: CombineRejection<Self::Error>,
    {
        loop {}
    }

    fn map<F>(self, fun: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Func<Self::Extract> + Clone,
    {
        loop {}
    }

    fn then<F>(self, fun: F) -> Then<Self, F>
    where
        Self: Sized,
        F: Func<Self::Extract> + Clone,
        F::Output: Future + Send,
    {
        loop {}
    }

    fn and_then<F>(self, fun: F) -> AndThen<Self, F>
    where
        Self: Sized,
        F: Func<Self::Extract> + Clone,
        F::Output: TryFuture + Send,
        <F::Output as TryFuture>::Error: CombineRejection<Self::Error>,
    {
        loop {}
    }

    fn or_else<F>(self, fun: F) -> OrElse<Self, F>
    where
        Self: Filter<Error = Rejection> + Sized,
        F: Func<Rejection>,
        F::Output: TryFuture<Ok = Self::Extract> + Send,
        <F::Output as TryFuture>::Error: IsReject,
    {
        loop {}
    }

    fn recover<F>(self, fun: F) -> Recover<Self, F>
    where
        Self: Filter<Error = Rejection> + Sized,
        F: Func<Rejection>,
        F::Output: TryFuture + Send,
        <F::Output as TryFuture>::Error: IsReject,
    {
        loop {}
    }

    fn unify<T>(self) -> Unify<Self>
    where
        Self: Filter<Extract = (Either<T, T>,)> + Sized,
        T: Tuple,
    {
        loop {}
    }

    fn untuple_one<T>(self) -> UntupleOne<Self>
    where
        Self: Filter<Extract = (T,)> + Sized,
        T: Tuple,
    {
        loop {}
    }

    fn with<W>(self, wrapper: W) -> W::Wrapped
    where
        Self: Sized,
        W: Wrap<Self>,
    {
        loop {}
    }

    fn boxed(self) -> BoxedFilter<Self::Extract>
    where
        Self: Sized + Send + Sync + 'static,
        Self::Extract: Send,
        Self::Error: Into<Rejection>,
    {
        loop {}
    }
}
impl<T: FilterBase> Filter for T {}
pub trait FilterClone: Filter + Clone {}
impl<T: Filter + Clone> FilterClone for T {}
fn _assert_object_safe() {
    loop {}
}
pub(crate) fn filter_fn<F, U>(func: F) -> FilterFn<F>
where
    F: Fn(&mut Route) -> U,
    U: TryFuture,
    U::Ok: Tuple,
    U::Error: IsReject,
{
    loop {}
}
pub(crate) fn filter_fn_one<F, U>(
    func: F,
) -> impl Filter<Extract = (U::Ok,), Error = U::Error> + Copy
where
    F: Fn(&mut Route) -> U + Copy,
    U: TryFuture + Send + 'static,
    U::Ok: Send,
    U::Error: IsReject,
{
    filter_fn(move |route| func(route).map_ok(|item| (item,)))
}
#[derive(Copy, Clone)]
#[allow(missing_debug_implementations)]
pub(crate) struct FilterFn<F> {
    func: F,
}
impl<F, U> FilterBase for FilterFn<F>
where
    F: Fn(&mut Route) -> U,
    U: TryFuture + Send + 'static,
    U::Ok: Tuple + Send,
    U::Error: IsReject,
{
    type Extract = U::Ok;
    type Error = U::Error;
    type Future = future::IntoFuture<U>;
    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}
