//! [`tracing`] filters.
//!
//! [`tracing`] is a framework for instrumenting Rust programs to
//! collect scoped, structured, and async-aware diagnostics. This module
//! provides a set of filters for instrumenting Warp applications with `tracing`
//! spans. [`Spans`] can be used to associate individual events  with a request,
//! and track contexts through the application.
//!
//! [`tracing`]: https://crates.io/crates/tracing
//! [`Spans`]: https://docs.rs/tracing/latest/tracing/#spans
use tracing::Span;
use std::net::SocketAddr;
use http::{self};
use crate::filter::{Filter, WrapSealed};
use crate::reject::IsReject;
use crate::reply::Reply;
use crate::route::Route;
use self::internal::WithTrace;
/// Create a wrapping filter that instruments every request with a `tracing`
/// [`Span`] at the [`INFO`] level, containing a summary of the request.
/// Additionally, if the [`DEBUG`] level is enabled, the span will contain an
/// event recording the request's headers.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::any()
///     .map(warp::reply)
///     .with(warp::trace::request());
/// ```
///
/// [`Span`]: https://docs.rs/tracing/latest/tracing/#spans
/// [`INFO`]: https://docs.rs/tracing/0.1.16/tracing/struct.Level.html#associatedconstant.INFO
/// [`DEBUG`]: https://docs.rs/tracing/0.1.16/tracing/struct.Level.html#associatedconstant.DEBUG
pub fn request() -> Trace<impl Fn(Info<'_>) -> Span + Clone> {
    trace(|info: Info<'_>| {
        loop {}
    })
}
/// Create a wrapping filter that instruments every request with a custom
/// `tracing` [`Span`] provided by a function.
///
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::any()
///     .map(warp::reply)
///     .with(warp::trace(|info| {
///         // Create a span using tracing macros
///         tracing::info_span!(
///             "request",
///             method = %info.method(),
///             path = %info.path(),
///         )
///     }));
/// ```
///
/// [`Span`]: https://docs.rs/tracing/latest/tracing/#spans
pub fn trace<F>(func: F) -> Trace<F>
where
    F: Fn(Info<'_>) -> Span + Clone,
{
    loop {}
}
/// Create a wrapping filter that instruments every request with a `tracing`
/// [`Span`] at the [`DEBUG`] level representing a named context.
///
/// This can be used to instrument multiple routes with their own sub-spans in a
/// per-request trace.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let hello = warp::path("hello")
///     .map(warp::reply)
///     .with(warp::trace::named("hello"));
///
/// let goodbye = warp::path("goodbye")
///     .map(warp::reply)
///     .with(warp::trace::named("goodbye"));
///
/// let routes = hello.or(goodbye);
/// ```
///
/// [`Span`]: https://docs.rs/tracing/latest/tracing/#spans
/// [`DEBUG`]: https://docs.rs/tracing/0.1.16/tracing/struct.Level.html#associatedconstant.DEBUG
pub fn named(name: &'static str) -> Trace<impl Fn(Info<'_>) -> Span + Copy> {
    trace(move |_| tracing::debug_span!("context", "{}", name,))
}
/// Decorates a [`Filter`](crate::Filter) to create a [`tracing`] [span] for
/// requests and responses.
///
/// [`tracing`]: https://crates.io/crates/tracing
/// [span]: https://docs.rs/tracing/latest/tracing/#spans
#[derive(Clone, Copy, Debug)]
pub struct Trace<F> {
    func: F,
}
/// Information about the request/response that can be used to prepare log lines.
#[allow(missing_debug_implementations)]
pub struct Info<'a> {
    route: &'a Route,
}
impl<FN, F> WrapSealed<F> for Trace<FN>
where
    FN: Fn(Info<'_>) -> Span + Clone + Send,
    F: Filter + Clone + Send,
    F::Extract: Reply,
    F::Error: IsReject,
{
    type Wrapped = WithTrace<FN, F>;
    fn wrap(&self, filter: F) -> Self::Wrapped {
        loop {}
    }
}
impl<'a> Info<'a> {
    /// View the remote `SocketAddr` of the request.
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        loop {}
    }
    /// View the `http::Method` of the request.
    pub fn method(&self) -> &http::Method {
        loop {}
    }
    /// View the URI path of the request.
    pub fn path(&self) -> &str {
        loop {}
    }
    /// View the `http::Version` of the request.
    pub fn version(&self) -> http::Version {
        loop {}
    }
    /// View the referer of the request.
    pub fn referer(&self) -> Option<&str> {
        loop {}
    }
    /// View the user agent of the request.
    pub fn user_agent(&self) -> Option<&str> {
        loop {}
    }
    /// View the host of the request
    pub fn host(&self) -> Option<&str> {
        loop {}
    }
    /// View the request headers.
    pub fn request_headers(&self) -> &http::HeaderMap {
        loop {}
    }
}
mod internal {
    use futures_util::{future::Inspect, future::MapOk};
    use super::{Info, Trace};
    use crate::filter::{Filter, FilterBase, Internal};
    use crate::reject::IsReject;
    use crate::reply::Reply;
    use crate::reply::Response;
    
    #[allow(missing_debug_implementations)]
    pub struct Traced(pub(super) Response);
    impl Reply for Traced {
        #[inline]
        fn into_response(self) -> Response {
            loop {}
        }
    }
    #[allow(missing_debug_implementations)]
    #[derive(Clone, Copy)]
    pub struct WithTrace<FN, F> {
        pub(super) filter: F,
        pub(super) trace: Trace<FN>,
    }
    use tracing::instrument::{Instrumented};
    use tracing::Span;
    fn finished_logger<E: IsReject>(reply: &Result<(Traced,), E>) {
        loop {}
    }
    fn convert_reply<R: Reply>(reply: R) -> (Traced,) {
        loop {}
    }
    impl<FN, F> FilterBase for WithTrace<FN, F>
    where
        FN: Fn(Info<'_>) -> Span + Clone + Send,
        F: Filter + Clone + Send,
        F::Extract: Reply,
        F::Error: IsReject,
    {
        type Extract = (Traced,);
        type Error = F::Error;
        type Future = Instrumented<
            Inspect<
                MapOk<F::Future, fn(F::Extract) -> Self::Extract>,
                fn(&Result<Self::Extract, F::Error>),
            >,
        >;
        fn filter(&self, _: Internal) -> Self::Future {
            loop {}
        }
    }
}
