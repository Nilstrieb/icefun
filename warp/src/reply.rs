//! Reply to requests.
//!
//! A [`Reply`](./trait.Reply.html) is a type that can be converted into an HTTP
//! response to be sent to the client. These are typically the successful
//! counterpart to a [rejection](../reject).
//!
//! The functions in this module are helpers for quickly creating a reply.
//! Besides them, you can return a type that implements [`Reply`](./trait.Reply.html). This
//! could be any of the following:
//!
//! - [`http::Response<impl Into<hyper::Body>>`](https://docs.rs/http)
//! - `String`
//! - `&'static str`
//! - `http::StatusCode`
//!
//! # Example
//!
//! ```
//! use warp::{Filter, http::Response};
//!
//! // Returns an empty `200 OK` response.
//! let empty_200 = warp::any().map(warp::reply);
//!
//! // Returns a `200 OK` response with custom header and body.
//! let custom = warp::any().map(|| {
//!     Response::builder()
//!         .header("my-custom-header", "some-value")
//!         .body("and a custom body")
//! });
//!
//! // GET requests return the empty 200, POST return the custom.
//! let routes = warp::get().and(empty_200)
//!     .or(warp::post().and(custom));
//! ```
use std::borrow::Cow;
use std::convert::TryFrom;
use std::error::Error as StdError;
use std::fmt;
use crate::generic::{Either, One};
use http::header::{HeaderName, HeaderValue};
use http::StatusCode;
use hyper::Body;
use serde::Serialize;
pub(crate) use self::sealed::Reply_;
use self::sealed::BoxedReply;
#[doc(hidden)]
pub use crate::filters::reply as with;
/// Response type into which types implementing the `Reply` trait are convertable.
pub type Response = ::http::Response<Body>;
/// Returns an empty `Reply` with status code `200 OK`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// // GET /just-ok returns an empty `200 OK`.
/// let route = warp::path("just-ok")
///     .map(|| {
///         println!("got a /just-ok request!");
///         warp::reply()
///     });
/// ```
#[inline]
pub fn reply() -> impl Reply {
    StatusCode::OK
}
/// Convert the value into a `Reply` with the value encoded as JSON.
///
/// The passed value must implement [`Serialize`][ser]. Many
/// collections do, and custom domain types can have `Serialize` derived.
///
/// [ser]: https://serde.rs
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// // GET /ids returns a `200 OK` with a JSON array of ids:
/// // `[1, 3, 7, 13]`
/// let route = warp::path("ids")
///     .map(|| {
///         let our_ids = vec![1, 3, 7, 13];
///         warp::reply::json(&our_ids)
///     });
/// ```
///
/// # Note
///
/// If a type fails to be serialized into JSON, the error is logged at the
/// `error` level, and the returned `impl Reply` will be an empty
/// `500 Internal Server Error` response.
pub fn json<T>(val: &T) -> Json
where
    T: Serialize,
{
    loop {}
}
/// A JSON formatted reply.
#[allow(missing_debug_implementations)]
pub struct Json {
    inner: Result<Vec<u8>, ()>,
}
impl Reply for Json {
    #[inline]
    fn into_response(self) -> Response {
        loop {}
    }
}
#[derive(Debug)]
pub(crate) struct ReplyJsonError;
impl fmt::Display for ReplyJsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for ReplyJsonError {}
/// Reply with a body and `content-type` set to `text/html; charset=utf-8`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let body = r#"
/// <html>
///     <head>
///         <title>HTML with warp!</title>
///     </head>
///     <body>
///         <h1>warp + HTML = &hearts;</h1>
///     </body>
/// </html>
/// "#;
///
/// let route = warp::any()
///     .map(move || {
///         warp::reply::html(body)
///     });
/// ```
pub fn html<T>(body: T) -> Html<T>
where
    Body: From<T>,
    T: Send,
{
    loop {}
}
/// An HTML reply.
#[allow(missing_debug_implementations)]
pub struct Html<T> {
    body: T,
}
impl<T> Reply for Html<T>
where
    Body: From<T>,
    T: Send,
{
    #[inline]
    fn into_response(self) -> Response {
        loop {}
    }
}
/// Types that can be converted into a `Response`.
///
/// This trait is implemented for the following:
///
/// - `http::StatusCode`
/// - `http::Response<impl Into<hyper::Body>>`
/// - `String`
/// - `&'static str`
///
/// # Example
///
/// ```rust
/// use warp::{Filter, http::Response};
///
/// struct Message {
///     msg: String
/// }
///
/// impl warp::Reply for Message {
///     fn into_response(self) -> warp::reply::Response {
///         Response::new(format!("message: {}", self.msg).into())
///     }
/// }
///
/// fn handler() -> Message {
///     Message { msg: "Hello".to_string() }
/// }
///
/// let route = warp::any().map(handler);
/// ```
pub trait Reply: BoxedReply + Send {
    /// Converts the given value into a [`Response`].
    ///
    /// [`Response`]: type.Response.html
    fn into_response(self) -> Response;
}
impl<T: Reply + ?Sized> Reply for Box<T> {
    fn into_response(self) -> Response {
        loop {}
    }
}
fn _assert_object_safe() {
    loop {}
}
/// Wrap an `impl Reply` to change its `StatusCode`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::any()
///     .map(warp::reply)
///     .map(|reply| {
///         warp::reply::with_status(reply, warp::http::StatusCode::CREATED)
///     });
/// ```
pub fn with_status<T: Reply>(reply: T, status: StatusCode) -> WithStatus<T> {
    loop {}
}
/// Wrap an `impl Reply` to change its `StatusCode`.
///
/// Returned by `warp::reply::with_status`.
#[derive(Debug)]
pub struct WithStatus<T> {
    reply: T,
    status: StatusCode,
}
impl<T: Reply> Reply for WithStatus<T> {
    fn into_response(self) -> Response {
        loop {}
    }
}
/// Wrap an `impl Reply` to add a header when rendering.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::any()
///     .map(warp::reply)
///     .map(|reply| {
///         warp::reply::with_header(reply, "server", "warp")
///     });
/// ```
pub fn with_header<T: Reply, K, V>(reply: T, name: K, value: V) -> WithHeader<T>
where
    HeaderName: TryFrom<K>,
    <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
{
    loop {}
}
/// Wraps an `impl Reply` and adds a header when rendering.
///
/// Returned by `warp::reply::with_header`.
#[derive(Debug)]
pub struct WithHeader<T> {
    header: Option<(HeaderName, HeaderValue)>,
    reply: T,
}
impl<T: Reply> Reply for WithHeader<T> {
    fn into_response(self) -> Response {
        loop {}
    }
}
impl<T: Send> Reply for ::http::Response<T>
where
    Body: From<T>,
{
    #[inline]
    fn into_response(self) -> Response {
        loop {}
    }
}
impl Reply for ::http::StatusCode {
    #[inline]
    fn into_response(self) -> Response {
        loop {}
    }
}
impl<T> Reply for Result<T, ::http::Error>
where
    T: Reply + Send,
{
    #[inline]
    fn into_response(self) -> Response {
        loop {}
    }
}
impl Reply for String {
    #[inline]
    fn into_response(self) -> Response {
        loop {}
    }
}
impl Reply for Vec<u8> {
    #[inline]
    fn into_response(self) -> Response {
        loop {}
    }
}
impl Reply for &'static str {
    #[inline]
    fn into_response(self) -> Response {
        loop {}
    }
}
impl Reply for Cow<'static, str> {
    #[inline]
    fn into_response(self) -> Response {
        loop {}
    }
}
impl Reply for &'static [u8] {
    #[inline]
    fn into_response(self) -> Response {
        loop {}
    }
}
impl<T, U> Reply for Either<T, U>
where
    T: Reply,
    U: Reply,
{
    #[inline]
    fn into_response(self) -> Response {
        loop {}
    }
}
impl<T> Reply for One<T>
where
    T: Reply,
{
    #[inline]
    fn into_response(self) -> Response {
        loop {}
    }
}
impl Reply for std::convert::Infallible {
    #[inline(always)]
    fn into_response(self) -> Response {
        loop {}
    }
}
mod sealed {
    use super::{Reply, Response};
    #[allow(missing_debug_implementations)]
    pub struct Reply_(pub(crate) Response);
    impl Reply for Reply_ {
        #[inline]
        fn into_response(self) -> Response {
            loop {}
        }
    }
    #[allow(missing_debug_implementations)]
    pub struct Internal;
    pub trait BoxedReply {
        fn boxed_into_response(self: Box<Self>, internal: Internal) -> Response;
    }
    impl<T: Reply> BoxedReply for T {
        fn boxed_into_response(self: Box<Self>, _: Internal) -> Response {
            loop {}
        }
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    #[test]
    fn json_serde_error() {
        loop {}
    }
    #[test]
    fn response_builder_error() {
        loop {}
    }
    #[test]
    fn boxed_reply() {
        loop {}
    }
}
