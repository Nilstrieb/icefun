use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use super::{Filter, FilterBase, Internal, Tuple};
use crate::reject::Rejection;



















pub struct BoxedFilter<T: Tuple> {
    filter: Arc<
        dyn Filter<
            Extract = T,
            Error = Rejection,
            Future = Pin<Box<dyn Future<Output = Result<T, Rejection>> + Send>>,
        > + Send + Sync,
    >,
}
impl<T: Tuple + Send> BoxedFilter<T> {
    pub(super) fn new<F>(filter: F) -> BoxedFilter<T>
    where
        F: Filter<Extract = T> + Send + Sync + 'static,
        F::Error: Into<Rejection>,
    {
        loop {}
    }
}
impl<T: Tuple> Clone for BoxedFilter<T> {
    fn clone(&self) -> BoxedFilter<T> {
        loop {}
    }
}
impl<T: Tuple> fmt::Debug for BoxedFilter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
fn _assert_send() {
    loop {}
}
impl<T: Tuple + Send> FilterBase for BoxedFilter<T> {
    type Extract = T;
    type Error = Rejection;
    type Future = Pin<Box<dyn Future<Output = Result<T, Rejection>> + Send>>;
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}
struct BoxingFilter<F> {
    filter: F,
}
impl<F> FilterBase for BoxingFilter<F>
where
    F: Filter,
    F::Future: Send + 'static,
{
    type Extract = F::Extract;
    type Error = F::Error;
    type Future = Pin<
        Box<dyn Future<Output = Result<Self::Extract, Self::Error>> + Send>,
    >;
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}
