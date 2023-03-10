//! Header Filters
//!
//! These filters are used to interact with the Request HTTP headers. Some
//! of them, like `exact` and `exact_ignore_case`, are just predicates,
//! they don't extract any values. The `header` filter allows parsing
//! a type from any header.
use std::convert::Infallible;
use std::str::FromStr;

use futures_util::future;
use headers::{Header, HeaderMapExt};
use http::header::HeaderValue;
use http::HeaderMap;

use crate::filter::{filter_fn, filter_fn_one, Filter, One};
use crate::reject::{self, Rejection};




















pub fn header<T: FromStr + Send + 'static>(
    name: &'static str,
) -> impl Filter<Extract = One<T>, Error = Rejection> + Copy {
    filter_fn_one(move |route| {
        tracing::trace!("header({:?})", name);
        let route = route
            .headers()
            .get(name)
            .ok_or_else(|| reject::missing_header(name))
            .and_then(|value| value.to_str().map_err(|_| reject::invalid_header(name)))
            .and_then(|s| T::from_str(s).map_err(|_| reject::invalid_header(name)));
        future::ready(route)
    })
}

pub(crate) fn header2<T: Header + Send + 'static>(
) -> impl Filter<Extract = One<T>, Error = Rejection> + Copy {
    filter_fn_one(move |route| {
        tracing::trace!("header2({:?})", T::name());
        let route = route
            .headers()
            .typed_get()
            .ok_or_else(|| reject::invalid_header(T::name().as_str()));
        future::ready(route)
    })
}













pub fn optional<T>(
    name: &'static str,
) -> impl Filter<Extract = One<Option<T>>, Error = Rejection> + Copy
where
    T: FromStr + Send + 'static,
{
    filter_fn_one(move |route| {
        tracing::trace!("optional({:?})", name);
        let result = route.headers().get(name).map(|value| {
            value
                .to_str()
                .map_err(|_| reject::invalid_header(name))?
                .parse::<T>()
                .map_err(|_| reject::invalid_header(name))
        });

        match result {
            Some(Ok(t)) => future::ok(Some(t)),
            Some(Err(e)) => future::err(e),
            None => future::ok(None),
        }
    })
}

pub(crate) fn optional2<T>() -> impl Filter<Extract = One<Option<T>>, Error = Infallible> + Copy
where
    T: Header + Send + 'static,
{
    filter_fn_one(move |route| future::ready(Ok(route.headers().typed_get())))
}

/* TODO
pub fn exact2<T>(header: T) -> impl FilterClone<Extract=(), Error=Rejection>
where
    T: Header + PartialEq + Clone + Send,
{
    filter_fn(move |route| {
        tracing::trace!("exact2({:?})", T::NAME);
        route.headers()
            .typed_get::<T>()
            .and_then(|val| if val == header {
                Some(())
            } else {
                None
            })
            .ok_or_else(|| reject::bad_request())
    })
}
*/












pub fn exact(
    name: &'static str,
    value: &'static str,
) -> impl Filter<Extract = (), Error = Rejection> + Copy {
    filter_fn(move |route| {
        tracing::trace!("exact?({:?}, {:?})", name, value);
        let route = route
            .headers()
            .get(name)
            .ok_or_else(|| reject::missing_header(name))
            .and_then(|val| {
                if val == value {
                    Ok(())
                } else {
                    Err(reject::invalid_header(name))
                }
            });
        future::ready(route)
    })
}












pub fn exact_ignore_case(
    name: &'static str,
    value: &'static str,
) -> impl Filter<Extract = (), Error = Rejection> + Copy {
    filter_fn(move |route| {
        tracing::trace!("exact_ignore_case({:?}, {:?})", name, value);
        let route = route
            .headers()
            .get(name)
            .ok_or_else(|| reject::missing_header(name))
            .and_then(|val| {
                if val.as_bytes().eq_ignore_ascii_case(value.as_bytes()) {
                    Ok(())
                } else {
                    Err(reject::invalid_header(name))
                }
            });
        future::ready(route)
    })
}













pub fn value(
    name: &'static str,
) -> impl Filter<Extract = One<HeaderValue>, Error = Rejection> + Copy {
    filter_fn_one(move |route| {
        tracing::trace!("value({:?})", name);
        let route = route
            .headers()
            .get(name)
            .cloned()
            .ok_or_else(|| reject::missing_header(name));
        future::ready(route)
    })
}













pub fn headers_cloned() -> impl Filter<Extract = One<HeaderMap>, Error = Infallible> + Copy {
    filter_fn_one(|route| future::ok(route.headers().clone()))
}
