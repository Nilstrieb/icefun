//! CORS Filters
use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;
use headers::{
    AccessControlAllowHeaders, AccessControlAllowMethods, AccessControlExposeHeaders,
};
use http::{self, header::{HeaderName, HeaderValue}};
use crate::filter::{Filter, WrapSealed};
use crate::reject::{CombineRejection, Rejection};
use crate::reply::Reply;
use self::internal::{CorsFilter, IntoOrigin, Seconds};
/// Create a wrapping filter that exposes [CORS][] behavior for a wrapped
/// filter.
///
/// [CORS]: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let cors = warp::cors()
///     .allow_origin("https://hyper.rs")
///     .allow_methods(vec!["GET", "POST", "DELETE"]);
///
/// let route = warp::any()
///     .map(warp::reply)
///     .with(cors);
/// ```
/// If you want to allow any route:
/// ```
/// use warp::Filter;
/// let cors = warp::cors()
///     .allow_any_origin();
/// ```
/// You can find more usage examples [here](https://github.com/seanmonstar/warp/blob/7fa54eaecd0fe12687137372791ff22fc7995766/tests/cors.rs).
pub fn cors() -> Builder {
    loop {}
}
/// A wrapping filter constructed via `warp::cors()`.
#[derive(Clone, Debug)]
pub struct Cors {
    config: Arc<Configured>,
}
/// A constructed via `warp::cors()`.
#[derive(Clone, Debug)]
pub struct Builder {
    credentials: bool,
    allowed_headers: HashSet<HeaderName>,
    exposed_headers: HashSet<HeaderName>,
    max_age: Option<u64>,
    methods: HashSet<http::Method>,
    origins: Option<HashSet<HeaderValue>>,
}
impl Builder {
    /// Sets whether to add the `Access-Control-Allow-Credentials` header.
    pub fn allow_credentials(mut self, allow: bool) -> Self {
        loop {}
    }
    /// Adds a method to the existing list of allowed request methods.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument is not a valid `http::Method`.
    pub fn allow_method<M>(mut self, method: M) -> Self
    where
        http::Method: TryFrom<M>,
    {
        loop {}
    }
    /// Adds multiple methods to the existing list of allowed request methods.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument is not a valid `http::Method`.
    pub fn allow_methods<I>(mut self, methods: I) -> Self
    where
        I: IntoIterator,
        http::Method: TryFrom<I::Item>,
    {
        loop {}
    }
    /// Adds a header to the list of allowed request headers.
    ///
    /// **Note**: These should match the values the browser sends via `Access-Control-Request-Headers`, e.g. `content-type`.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument is not a valid `http::header::HeaderName`.
    pub fn allow_header<H>(mut self, header: H) -> Self
    where
        HeaderName: TryFrom<H>,
    {
        loop {}
    }
    /// Adds multiple headers to the list of allowed request headers.
    ///
    /// **Note**: These should match the values the browser sends via `Access-Control-Request-Headers`, e.g.`content-type`.
    ///
    /// # Panics
    ///
    /// Panics if any of the headers are not a valid `http::header::HeaderName`.
    pub fn allow_headers<I>(mut self, headers: I) -> Self
    where
        I: IntoIterator,
        HeaderName: TryFrom<I::Item>,
    {
        loop {}
    }
    /// Adds a header to the list of exposed headers.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument is not a valid `http::header::HeaderName`.
    pub fn expose_header<H>(mut self, header: H) -> Self
    where
        HeaderName: TryFrom<H>,
    {
        loop {}
    }
    /// Adds multiple headers to the list of exposed headers.
    ///
    /// # Panics
    ///
    /// Panics if any of the headers are not a valid `http::header::HeaderName`.
    pub fn expose_headers<I>(mut self, headers: I) -> Self
    where
        I: IntoIterator,
        HeaderName: TryFrom<I::Item>,
    {
        loop {}
    }
    /// Sets that *any* `Origin` header is allowed.
    ///
    /// # Warning
    ///
    /// This can allow websites you didn't intend to access this resource,
    /// it is usually better to set an explicit list.
    pub fn allow_any_origin(mut self) -> Self {
        loop {}
    }
    /// Add an origin to the existing list of allowed `Origin`s.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument is not a valid `Origin`.
    pub fn allow_origin(self, origin: impl IntoOrigin) -> Self {
        loop {}
    }
    /// Add multiple origins to the existing list of allowed `Origin`s.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument is not a valid `Origin`.
    pub fn allow_origins<I>(mut self, origins: I) -> Self
    where
        I: IntoIterator,
        I::Item: IntoOrigin,
    {
        loop {}
    }
    /// Sets the `Access-Control-Max-Age` header.
    ///
    /// # Example
    ///
    ///
    /// ```
    /// use std::time::Duration;
    /// use warp::Filter;
    ///
    /// let cors = warp::cors()
    ///     .max_age(30) // 30u32 seconds
    ///     .max_age(Duration::from_secs(30)); // or a Duration
    /// ```
    pub fn max_age(mut self, seconds: impl Seconds) -> Self {
        loop {}
    }
    /// Builds the `Cors` wrapper from the configured settings.
    ///
    /// This step isn't *required*, as the `Builder` itself can be passed
    /// to `Filter::with`. This just allows constructing once, thus not needing
    /// to pay the cost of "building" every time.
    pub fn build(self) -> Cors {
        loop {}
    }
}
impl<F> WrapSealed<F> for Builder
where
    F: Filter + Clone + Send + Sync + 'static,
    F::Extract: Reply,
    F::Error: CombineRejection<Rejection>,
    <F::Error as CombineRejection<Rejection>>::One: CombineRejection<Rejection>,
{
    type Wrapped = CorsFilter<F>;
    fn wrap(&self, inner: F) -> Self::Wrapped {
        loop {}
    }
}
impl<F> WrapSealed<F> for Cors
where
    F: Filter + Clone + Send + Sync + 'static,
    F::Extract: Reply,
    F::Error: CombineRejection<Rejection>,
    <F::Error as CombineRejection<Rejection>>::One: CombineRejection<Rejection>,
{
    type Wrapped = CorsFilter<F>;
    fn wrap(&self, inner: F) -> Self::Wrapped {
        loop {}
    }
}
/// An error used to reject requests that are forbidden by a `cors` filter.
pub struct CorsForbidden {
    kind: Forbidden,
}
#[derive(Debug)]
enum Forbidden {
    OriginNotAllowed,
    MethodNotAllowed,
    HeaderNotAllowed,
}
impl fmt::Debug for CorsForbidden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl fmt::Display for CorsForbidden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for CorsForbidden {}
#[derive(Clone, Debug)]
struct Configured {
    cors: Builder,
    allowed_headers_header: AccessControlAllowHeaders,
    expose_headers_header: Option<AccessControlExposeHeaders>,
    methods_header: AccessControlAllowMethods,
}
enum Validated {
    Preflight(HeaderValue),
    Simple(HeaderValue),
    NotCors,
}
impl Configured {}
mod internal {
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::task::{Context, Poll};
    use futures_util::{future, TryFuture};
    use headers::Origin;
    use http::header;
    use pin_project::pin_project;
    use super::Configured;
    use crate::filter::{Filter, FilterBase, Internal, One};
    use crate::generic::Either;
    use crate::reject::{CombineRejection, Rejection};
    #[derive(Clone, Debug)]
    pub struct CorsFilter<F> {
        pub(super) config: Arc<Configured>,
        pub(super) inner: F,
    }
    impl<F> FilterBase for CorsFilter<F>
    where
        F: Filter,
        F::Extract: Send,
        F::Future: Future,
        F::Error: CombineRejection<Rejection>,
    {
        type Extract = One<
            Either<One<Preflight>, One<Either<One<Wrapped<F::Extract>>, F::Extract>>>,
        >;
        type Error = <F::Error as CombineRejection<Rejection>>::One;
        type Future = future::Either<
            future::Ready<Result<Self::Extract, Self::Error>>,
            WrappedFuture<F::Future>,
        >;
        fn filter(&self, _: Internal) -> Self::Future {
            loop {}
        }
    }
    #[derive(Debug)]
    pub struct Preflight {
        config: Arc<Configured>,
        origin: header::HeaderValue,
    }
    impl crate::reply::Reply for Preflight {
        fn into_response(self) -> crate::reply::Response {
            loop {}
        }
    }
    #[derive(Debug)]
    pub struct Wrapped<R> {
        config: Arc<Configured>,
        inner: R,
        origin: header::HeaderValue,
    }
    impl<R> crate::reply::Reply for Wrapped<R>
    where
        R: crate::reply::Reply,
    {
        fn into_response(self) -> crate::reply::Response {
            loop {}
        }
    }
    #[pin_project]
    #[derive(Debug)]
    pub struct WrappedFuture<F> {
        #[pin]
        inner: F,
        wrapped: Option<(Arc<Configured>, header::HeaderValue)>,
    }
    impl<F> Future for WrappedFuture<F>
    where
        F: TryFuture,
        F::Error: CombineRejection<Rejection>,
    {
        type Output = Result<
            One<Either<One<Preflight>, One<Either<One<Wrapped<F::Ok>>, F::Ok>>>>,
            <F::Error as CombineRejection<Rejection>>::One,
        >;
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            loop {}
        }
    }
    pub trait Seconds {
        fn seconds(self) -> u64;
    }
    impl Seconds for u32 {
        fn seconds(self) -> u64 {
            loop {}
        }
    }
    impl Seconds for ::std::time::Duration {
        fn seconds(self) -> u64 {
            loop {}
        }
    }
    pub trait IntoOrigin {
        fn into_origin(self) -> Origin;
    }
    impl<'a> IntoOrigin for &'a str {
        fn into_origin(self) -> Origin {
            loop {}
        }
    }
}
