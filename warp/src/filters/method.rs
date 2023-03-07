//! HTTP Method filters.
//!
//! The filters deal with the HTTP Method part of a request. Several here will
//! match the request `Method`, and if not matched, will reject the request
//! with a `405 Method Not Allowed`.
//!
//! There is also [`warp::method()`](method), which never rejects
//! a request, and just extracts the method to be used in your filter chains.
use futures_util::future;
use http::Method;
use crate::filter::{filter_fn, filter_fn_one, Filter, One};
use crate::reject::Rejection;
use std::convert::Infallible;









pub fn get() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::GET)
}









pub fn post() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::POST)
}









pub fn put() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::PUT)
}









pub fn delete() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::DELETE)
}









pub fn head() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::HEAD)
}









pub fn options() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::OPTIONS)
}









pub fn patch() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::PATCH)
}














pub fn method() -> impl Filter<Extract = One<Method>, Error = Infallible> + Copy {
    filter_fn_one(|route| future::ok::<_, Infallible>(route.method().clone()))
}
fn method_is<F>(func: F) -> impl Filter<Extract = (), Error = Rejection> + Copy
where
    F: Fn() -> &'static Method + Copy,
{
    filter_fn(move |route| {
        let method = func();
        tracing::trace!("method::{:?}?: {:?}", method, route.method());
        if route.method() == method {
            future::ok(())
        } else {
            future::err(crate::reject::method_not_allowed())
        }
    })
}
#[cfg(test)]
mod tests {
    #[test]
    fn method_size_of() {
        loop {}
    }
}
