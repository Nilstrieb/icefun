//! Rejections
//!
//! Part of the power of the [`Filter`](../trait.Filter.html) system is being able to
//! reject a request from a filter chain. This allows for filters to be
//! combined with `or`, so that if one side of the chain finds that a request
//! doesn't fulfill its requirements, the other side can try to process
//! the request.
//!
//! Many of the built-in [`filters`](../filters) will automatically reject
//! the request with an appropriate rejection. However, you can also build
//! new custom [`Filter`](../trait.Filter.html)s and still want other routes to be
//! matchable in the case a predicate doesn't hold.
//!
//! As a request is processed by a Filter chain, the rejections are accumulated into
//! a list contained by the [`Rejection`](struct.Rejection.html) type. Rejections from
//! filters can be handled using [`Filter::recover`](../trait.Filter.html#method.recover).
//! This is a convenient way to map rejections into a [`Reply`](../reply/trait.Reply.html).
//!
//! For a more complete example see the
//! [Rejection Example](https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs)
//! from the repository.
//!
//! # Example
//!
//! ```
//! use warp::{reply, Reply, Filter, reject, Rejection, http::StatusCode};
//!
//! #[derive(Debug)]
//! struct InvalidParameter;
//!
//! impl reject::Reject for InvalidParameter {}
//!
//! // Custom rejection handler that maps rejections into responses.
//! async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
//!     if err.is_not_found() {
//!         Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
//!     } else if let Some(e) = err.find::<InvalidParameter>() {
//!         Ok(reply::with_status("BAD_REQUEST", StatusCode::BAD_REQUEST))
//!     } else {
//!         eprintln!("unhandled rejection: {:?}", err);
//!         Ok(reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR))
//!     }
//! }
//!
//!
//! // Filter on `/:id`, but reject with InvalidParameter if the `id` is `0`.
//! // Recover from this rejection using a custom rejection handler.
//! let route = warp::path::param()
//!     .and_then(|id: u32| async move {
//!         if id == 0 {
//!             Err(warp::reject::custom(InvalidParameter))
//!         } else {
//!             Ok("id is valid")
//!         }
//!     })
//!     .recover(handle_rejection);
//! ```
use std::any::Any;
use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt;
use http::{self, StatusCode};
pub(crate) use self::sealed::{CombineRejection, IsReject};

#[inline]
pub fn reject() -> Rejection {
    loop {}
}

#[inline]
pub fn not_found() -> Rejection {
    loop {}
}
#[inline]
pub(crate) fn invalid_query() -> Rejection {
    loop {}
}
#[inline]
pub(crate) fn missing_header(name: &'static str) -> Rejection {
    loop {}
}
#[inline]
pub(crate) fn invalid_header(name: &'static str) -> Rejection {
    loop {}
}
#[inline]
pub(crate) fn missing_cookie(name: &'static str) -> Rejection {
    loop {}
}
#[inline]
pub(crate) fn method_not_allowed() -> Rejection {
    loop {}
}
#[inline]
pub(crate) fn length_required() -> Rejection {
    loop {}
}
#[inline]
pub(crate) fn payload_too_large() -> Rejection {
    loop {}
}
#[inline]
pub(crate) fn unsupported_media_type() -> Rejection {
    loop {}
}






pub fn custom<T: Reject>(err: T) -> Rejection {
    loop {}
}







fn __reject_custom_compilefail() {}


















pub trait Reject: fmt::Debug + Sized + Send + Sync + 'static {}
trait Cause: fmt::Debug + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
}
impl<T> Cause for T
where
    T: fmt::Debug + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        loop {}
    }
}
impl dyn Cause {}
pub(crate) fn known<T: Into<Known>>(err: T) -> Rejection {
    loop {}
}



pub struct Rejection {
    reason: Reason,
}
enum Reason {
    NotFound,
    Other(Box<Rejections>),
}
enum Rejections {
    Known(Known),
    Custom(Box<dyn Cause>),
    Combined(Box<Rejections>, Box<Rejections>),
}
macro_rules! enum_known {
    ($($(#[$attr:meta])* $var:ident ($ty:path),)+) => {
        pub (crate) enum Known { $($(#[$attr])* $var ($ty),)+ } impl Known { fn
        inner_as_any(& self) -> & dyn Any { match * self { $($(#[$attr])* Known::$var
        (ref t) => t,)+ } } } impl fmt::Debug for Known { fn fmt(& self, f : & mut
        fmt::Formatter <'_ >) -> fmt::Result { match * self { $($(#[$attr])* Known::$var
        (ref t) => t.fmt(f),)+ } } } impl fmt::Display for Known { fn fmt(& self, f : &
        mut fmt::Formatter <'_ >) -> fmt::Result { match * self { $($(#[$attr])*
        Known::$var (ref t) => t.fmt(f),)+ } } } $(#[doc(hidden)] $(#[$attr])* impl From
        <$ty > for Known { fn from(ty : $ty) -> Known { Known::$var (ty) } })+
    };
}
enum_known! {
    MethodNotAllowed(MethodNotAllowed), InvalidHeader(InvalidHeader),
    MissingHeader(MissingHeader), MissingCookie(MissingCookie),
    InvalidQuery(InvalidQuery), LengthRequired(LengthRequired),
    PayloadTooLarge(PayloadTooLarge), UnsupportedMediaType(UnsupportedMediaType),
    FileOpenError(crate ::fs::FileOpenError), FilePermissionError(crate
    ::fs::FilePermissionError), BodyReadError(crate ::body::BodyReadError),
    BodyDeserializeError(crate ::body::BodyDeserializeError), CorsForbidden(crate
    ::cors::CorsForbidden), #[cfg(feature = "websocket")] MissingConnectionUpgrade(crate
    ::ws::MissingConnectionUpgrade), MissingExtension(crate ::ext::MissingExtension),
    BodyConsumedMultipleTimes(crate ::body::BodyConsumedMultipleTimes),
}
impl Rejection {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub fn find<T: 'static>(&self) -> Option<&T> {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    pub fn is_not_found(&self) -> bool {
        loop {}
    }
}
impl<T: Reject> From<T> for Rejection {
    #[inline]
    fn from(err: T) -> Rejection {
        loop {}
    }
}
impl From<Infallible> for Rejection {
    #[inline]
    fn from(infallible: Infallible) -> Rejection {
        loop {}
    }
}
impl IsReject for Infallible {
    fn status(&self) -> StatusCode {
        loop {}
    }
    fn into_response(&self) -> crate::reply::Response {
        loop {}
    }
}
impl IsReject for Rejection {
    fn status(&self) -> StatusCode {
        loop {}
    }
    fn into_response(&self) -> crate::reply::Response {
        loop {}
    }
}
impl fmt::Debug for Rejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl fmt::Debug for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Rejections {}
unit_error! {
    #[doc = " Invalid query"] pub InvalidQuery : "Invalid query string"
}
unit_error! {
    #[doc = " HTTP method not allowed"] pub MethodNotAllowed : "HTTP method not allowed"
}
unit_error! {
    #[doc = " A content-length header is required"] pub LengthRequired :
    "A content-length header is required"
}
unit_error! {
    #[doc = " The request payload is too large"] pub PayloadTooLarge :
    "The request payload is too large"
}
unit_error! {
    #[doc = " The request's content-type is not supported"] pub UnsupportedMediaType :
    "The request's content-type is not supported"
}

#[derive(Debug)]
pub struct MissingHeader {
    name: &'static str,
}
impl MissingHeader {
    
    pub fn name(&self) -> &str {
        loop {}
    }
}
impl fmt::Display for MissingHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for MissingHeader {}

#[derive(Debug)]
pub struct InvalidHeader {
    name: &'static str,
}
impl InvalidHeader {
    
    pub fn name(&self) -> &str {
        loop {}
    }
}
impl fmt::Display for InvalidHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for InvalidHeader {}

#[derive(Debug)]
pub struct MissingCookie {
    name: &'static str,
}
impl MissingCookie {
    
    pub fn name(&self) -> &str {
        loop {}
    }
}
impl fmt::Display for MissingCookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for MissingCookie {}
mod sealed {
    use super::Rejection;
    use http::StatusCode;
    use std::convert::Infallible;
    use std::fmt;
    pub trait IsReject: fmt::Debug + Send + Sync {
        fn status(&self) -> StatusCode;
        fn into_response(&self) -> crate::reply::Response;
    }
    fn _assert_object_safe() {
        loop {}
    }
    pub trait CombineRejection<E>: Send + Sized {
        
        
        
        
        
        
        
        
        
        
        
        
        
        type One: IsReject + From<Self> + From<E> + Into<Rejection>;
        
        
        type Combined: IsReject;
        fn combine(self, other: E) -> Self::Combined;
    }
    impl CombineRejection<Rejection> for Rejection {
        type One = Rejection;
        type Combined = Rejection;
        fn combine(self, other: Rejection) -> Self::Combined {
            loop {}
        }
    }
    impl CombineRejection<Infallible> for Rejection {
        type One = Rejection;
        type Combined = Infallible;
        fn combine(self, other: Infallible) -> Self::Combined {
            loop {}
        }
    }
    impl CombineRejection<Rejection> for Infallible {
        type One = Rejection;
        type Combined = Infallible;
        fn combine(self, _: Rejection) -> Self::Combined {
            loop {}
        }
    }
    impl CombineRejection<Infallible> for Infallible {
        type One = Infallible;
        type Combined = Infallible;
        fn combine(self, _: Infallible) -> Self::Combined {
            loop {}
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use http::StatusCode;
    #[derive(Debug, PartialEq)]
    struct Left;
    #[derive(Debug, PartialEq)]
    struct Right;
    impl Reject for Left {}
    impl Reject for Right {}
    #[test]
    fn rejection_status() {
        loop {}
    }
    #[tokio::test]
    async fn combine_rejection_causes_with_some_left_and_none_right() {
        loop {}
    }
    #[tokio::test]
    async fn combine_rejection_causes_with_none_left_and_some_right() {
        loop {}
    }
    #[tokio::test]
    async fn unhandled_customs() {
        loop {}
    }
    async fn response_body_string(resp: crate::reply::Response) -> String {
        loop {}
    }
    #[test]
    fn find_cause() {
        loop {}
    }
    #[test]
    fn size_of_rejection() {
        loop {}
    }
    #[derive(Debug)]
    struct X(u32);
    impl Reject for X {}
    fn combine_n<F, R>(n: u32, new_reject: F) -> Rejection
    where
        F: Fn(u32) -> R,
        R: Reject,
    {
        loop {}
    }
    #[test]
    fn test_debug() {
        loop {}
    }
}
