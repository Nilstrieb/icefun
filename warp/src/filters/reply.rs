//! Reply Filters
//!
//! These "filters" behave a little differently than the rest. Instead of
//! being used directly on requests, these filters "wrap" other filters.
//!
//!
//! ## Wrapping a `Filter` (`with`)
//!
//! ```
//! use warp::Filter;
//!
//! let with_server = warp::reply::with::header("server", "warp");
//!
//! let route = warp::any()
//!     .map(warp::reply)
//!     .with(with_server);
//! ```
//!
//! Wrapping allows adding in conditional logic *before* the request enters
//! the inner filter (though the `with::header` wrapper does not).
use std::convert::TryFrom;
use std::sync::Arc;
use http::header::{HeaderMap, HeaderName, HeaderValue};
use self::sealed::{WithDefaultHeader_, WithHeader_, WithHeaders_};
use crate::filter::{Filter, Map, WrapSealed};
use crate::reply::Reply;
/// Wrap a [`Filter`](crate::Filter) that adds a header to the reply.
///
/// # Note
///
/// This **only** adds a header if the underlying filter is successful, and
/// returns a [`Reply`](Reply). If the underlying filter was rejected, the
/// header is not added.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// // Always set `foo: bar` header.
/// let route = warp::any()
///     .map(warp::reply)
///     .with(warp::reply::with::header("foo", "bar"));
/// ```
pub fn header<K, V>(name: K, value: V) -> WithHeader
where
    HeaderName: TryFrom<K>,
    <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
{
    loop {}
}
/// Wrap a [`Filter`](crate::Filter) that adds multiple headers to the reply.
///
/// # Note
///
/// This **only** adds a header if the underlying filter is successful, and
/// returns a [`Reply`](Reply). If the underlying filter was rejected, the
/// header is not added.
///
/// # Example
///
/// ```
/// use warp::http::header::{HeaderMap, HeaderValue};
/// use warp::Filter;
///
/// let mut headers = HeaderMap::new();
/// headers.insert("server", HeaderValue::from_static("wee/0"));
/// headers.insert("foo", HeaderValue::from_static("bar"));
///
/// // Always set `server: wee/0` and `foo: bar` headers.
/// let route = warp::any()
///     .map(warp::reply)
///     .with(warp::reply::with::headers(headers));
/// ```
pub fn headers(headers: HeaderMap) -> WithHeaders {
    loop {}
}
/// Wrap a [`Filter`](crate::Filter) that adds a header to the reply, if they
/// aren't already set.
///
/// # Note
///
/// This **only** adds a header if the underlying filter is successful, and
/// returns a [`Reply`](Reply). If the underlying filter was rejected, the
/// header is not added.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// // Set `server: warp` if not already set.
/// let route = warp::any()
///     .map(warp::reply)
///     .with(warp::reply::with::default_header("server", "warp"));
/// ```
pub fn default_header<K, V>(name: K, value: V) -> WithDefaultHeader
where
    HeaderName: TryFrom<K>,
    <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
{
    loop {}
}
/// Wrap a `Filter` to always set a header.
#[derive(Clone, Debug)]
pub struct WithHeader {
    name: HeaderName,
    value: HeaderValue,
}
impl<F, R> WrapSealed<F> for WithHeader
where
    F: Filter<Extract = (R,)>,
    R: Reply,
{
    type Wrapped = Map<F, WithHeader_>;
    fn wrap(&self, filter: F) -> Self::Wrapped {
        loop {}
    }
}
/// Wrap a `Filter` to always set multiple headers.
#[derive(Clone, Debug)]
pub struct WithHeaders {
    headers: Arc<HeaderMap>,
}
impl<F, R> WrapSealed<F> for WithHeaders
where
    F: Filter<Extract = (R,)>,
    R: Reply,
{
    type Wrapped = Map<F, WithHeaders_>;
    fn wrap(&self, filter: F) -> Self::Wrapped {
        loop {}
    }
}
/// Wrap a `Filter` to set a header if it is not already set.
#[derive(Clone, Debug)]
pub struct WithDefaultHeader {
    name: HeaderName,
    value: HeaderValue,
}
impl<F, R> WrapSealed<F> for WithDefaultHeader
where
    F: Filter<Extract = (R,)>,
    R: Reply,
{
    type Wrapped = Map<F, WithDefaultHeader_>;
    fn wrap(&self, filter: F) -> Self::Wrapped {
        loop {}
    }
}
fn assert_name_and_value<K, V>(name: K, value: V) -> (HeaderName, HeaderValue)
where
    HeaderName: TryFrom<K>,
    <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
{
    loop {}
}
mod sealed {
    use super::{WithDefaultHeader, WithHeader, WithHeaders};
    use crate::generic::{Func, One};
    use crate::reply::{Reply, Reply_};
    #[derive(Clone)]
    #[allow(missing_debug_implementations)]
    pub struct WithHeader_ {
        pub(super) with: WithHeader,
    }
    impl<R: Reply> Func<One<R>> for WithHeader_ {
        type Output = Reply_;
        fn call(&self, args: One<R>) -> Self::Output {
            loop {}
        }
    }
    #[derive(Clone)]
    #[allow(missing_debug_implementations)]
    pub struct WithHeaders_ {
        pub(super) with: WithHeaders,
    }
    impl<R: Reply> Func<One<R>> for WithHeaders_ {
        type Output = Reply_;
        fn call(&self, args: One<R>) -> Self::Output {
            loop {}
        }
    }
    #[derive(Clone)]
    #[allow(missing_debug_implementations)]
    pub struct WithDefaultHeader_ {
        pub(super) with: WithDefaultHeader,
    }
    impl<R: Reply> Func<One<R>> for WithDefaultHeader_ {
        type Output = Reply_;
        fn call(&self, args: One<R>) -> Self::Output {
            loop {}
        }
    }
}
