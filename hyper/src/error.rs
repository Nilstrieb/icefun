//! Error and Result module.
use std::error::Error as StdError;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;
type Cause = Box<dyn StdError + Send + Sync>;

pub struct Error {
    inner: Box<ErrorImpl>,
}
struct ErrorImpl {
    kind: Kind,
    cause: Option<Cause>,
}
#[derive(Debug)]
pub(super) enum Kind {
    Parse(Parse),
    User(User),
    
    #[allow(unused)]
    IncompleteMessage,
    
    #[cfg(feature = "http1")]
    UnexpectedMessage,
    
    Canceled,
    
    ChannelClosed,
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    Io,
    
    #[allow(unused)]
    Connect,
    
    #[cfg(all(feature = "tcp", feature = "server"))]
    Listen,
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    Accept,
    
    #[cfg(all(feature = "http1", feature = "server", feature = "runtime"))]
    HeaderTimeout,
    
    #[cfg(any(feature = "http1", feature = "http2", feature = "stream"))]
    Body,
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    BodyWrite,
    
    #[cfg(feature = "http1")]
    Shutdown,
    
    #[cfg(feature = "http2")]
    Http2,
}
#[derive(Debug)]
pub(super) enum Parse {
    Method,
    Version,
    #[cfg(feature = "http1")]
    VersionH2,
    Uri,
    #[cfg_attr(not(all(feature = "http1", feature = "server")), allow(unused))]
    UriTooLong,
    Header(Header),
    TooLarge,
    Status,
    #[cfg_attr(debug_assertions, allow(unused))]
    Internal,
}
#[derive(Debug)]
pub(super) enum Header {
    Token,
    #[cfg(feature = "http1")]
    ContentLengthInvalid,
    #[cfg(all(feature = "http1", feature = "server"))]
    TransferEncodingInvalid,
    #[cfg(feature = "http1")]
    TransferEncodingUnexpected,
}
#[derive(Debug)]
pub(super) enum User {
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    Body,
    
    BodyWriteAborted,
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    MakeService,
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    Service,
    
    
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    UnexpectedHeader,
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    UnsupportedVersion,
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    UnsupportedRequestMethod,
    
    #[cfg(feature = "http1")]
    #[cfg(feature = "server")]
    UnsupportedStatusCode,
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    AbsoluteUriRequired,
    
    NoUpgrade,
    
    #[cfg(feature = "http1")]
    ManualUpgrade,
    
    #[cfg(feature = "server")]
    WithoutShutdownNonHttp1,
    
    #[cfg(feature = "client")]
    DispatchGone,
    
    #[cfg(feature = "ffi")]
    AbortedByCallback,
}
#[derive(Debug)]
pub(super) struct TimedOut;
impl Error {
    
    pub(crate) fn is_parse(&self) -> bool {
        loop {}
    }
    
    pub(crate) fn is_parse_too_large(&self) -> bool {
        loop {}
    }
    
    
    pub(crate) fn is_parse_status(&self) -> bool {
        loop {}
    }
    
    pub(crate) fn is_user(&self) -> bool {
        loop {}
    }
    
    pub(crate) fn is_canceled(&self) -> bool {
        loop {}
    }
    
    pub(crate) fn is_closed(&self) -> bool {
        loop {}
    }
    
    pub(crate) fn is_connect(&self) -> bool {
        loop {}
    }
    
    pub(crate) fn is_incomplete_message(&self) -> bool {
        loop {}
    }
    
    pub(crate) fn is_body_write_aborted(&self) -> bool {
        loop {}
    }
    
    pub(crate) fn is_timeout(&self) -> bool {
        loop {}
    }
    
    pub(crate) fn into_cause(self) -> Option<Box<dyn StdError + Send + Sync>> {
        loop {}
    }
    pub(super) fn new(kind: Kind) -> Error {
        loop {}
    }
    pub(super) fn with<C: Into<Cause>>(mut self, cause: C) -> Error {
        loop {}
    }
    #[cfg(any(all(feature = "http1", feature = "server"), feature = "ffi"))]
    pub(super) fn kind(&self) -> &Kind {
        loop {}
    }
    pub(crate) fn find_source<E: StdError + 'static>(&self) -> Option<&E> {
        loop {}
    }
    #[cfg(feature = "http2")]
    pub(super) fn h2_reason(&self) -> h2::Reason {
        loop {}
    }
    pub(super) fn new_canceled() -> Error {
        loop {}
    }
    #[cfg(feature = "http1")]
    pub(super) fn new_incomplete() -> Error {
        loop {}
    }
    #[cfg(feature = "http1")]
    pub(super) fn new_too_large() -> Error {
        loop {}
    }
    #[cfg(feature = "http1")]
    pub(super) fn new_version_h2() -> Error {
        loop {}
    }
    #[cfg(feature = "http1")]
    pub(super) fn new_unexpected_message() -> Error {
        loop {}
    }
    #[cfg(any(feature = "http1", feature = "http2"))]
    pub(super) fn new_io(cause: std::io::Error) -> Error {
        loop {}
    }
    #[cfg(all(feature = "server", feature = "tcp"))]
    pub(super) fn new_listen<E: Into<Cause>>(cause: E) -> Error {
        loop {}
    }
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    pub(super) fn new_accept<E: Into<Cause>>(cause: E) -> Error {
        loop {}
    }
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    pub(super) fn new_connect<E: Into<Cause>>(cause: E) -> Error {
        loop {}
    }
    pub(super) fn new_closed() -> Error {
        loop {}
    }
    #[cfg(any(feature = "http1", feature = "http2", feature = "stream"))]
    pub(super) fn new_body<E: Into<Cause>>(cause: E) -> Error {
        loop {}
    }
    #[cfg(any(feature = "http1", feature = "http2"))]
    pub(super) fn new_body_write<E: Into<Cause>>(cause: E) -> Error {
        loop {}
    }
    pub(super) fn new_body_write_aborted() -> Error {
        loop {}
    }
    fn new_user(user: User) -> Error {
        loop {}
    }
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    pub(super) fn new_user_header() -> Error {
        loop {}
    }
    #[cfg(all(feature = "http1", feature = "server", feature = "runtime"))]
    pub(super) fn new_header_timeout() -> Error {
        loop {}
    }
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    pub(super) fn new_user_unsupported_version() -> Error {
        loop {}
    }
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    pub(super) fn new_user_unsupported_request_method() -> Error {
        loop {}
    }
    #[cfg(feature = "http1")]
    #[cfg(feature = "server")]
    pub(super) fn new_user_unsupported_status_code() -> Error {
        loop {}
    }
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    pub(super) fn new_user_absolute_uri_required() -> Error {
        loop {}
    }
    pub(super) fn new_user_no_upgrade() -> Error {
        loop {}
    }
    #[cfg(feature = "http1")]
    pub(super) fn new_user_manual_upgrade() -> Error {
        loop {}
    }
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    pub(super) fn new_user_make_service<E: Into<Cause>>(cause: E) -> Error {
        loop {}
    }
    #[cfg(any(feature = "http1", feature = "http2"))]
    pub(super) fn new_user_service<E: Into<Cause>>(cause: E) -> Error {
        loop {}
    }
    #[cfg(any(feature = "http1", feature = "http2"))]
    pub(super) fn new_user_body<E: Into<Cause>>(cause: E) -> Error {
        loop {}
    }
    #[cfg(feature = "server")]
    pub(super) fn new_without_shutdown_not_h1() -> Error {
        loop {}
    }
    #[cfg(feature = "http1")]
    pub(super) fn new_shutdown(cause: std::io::Error) -> Error {
        loop {}
    }
    #[cfg(feature = "ffi")]
    pub(super) fn new_user_aborted_by_callback() -> Error {
        loop {}
    }
    #[cfg(feature = "client")]
    pub(super) fn new_user_dispatch_gone() -> Error {
        loop {}
    }
    #[cfg(feature = "http2")]
    pub(super) fn new_h2(cause: ::h2::Error) -> Error {
        loop {}
    }
    
    pub(crate) fn message(&self) -> impl fmt::Display + '_ {
        self.description()
    }
    fn description(&self) -> &str {
        loop {}
    }
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        loop {}
    }
}
#[doc(hidden)]
impl From<Parse> for Error {
    fn from(err: Parse) -> Error {
        loop {}
    }
}
#[cfg(feature = "http1")]
impl Parse {
    pub(crate) fn content_length_invalid() -> Self {
        loop {}
    }
    #[cfg(all(feature = "http1", feature = "server"))]
    pub(crate) fn transfer_encoding_invalid() -> Self {
        loop {}
    }
    pub(crate) fn transfer_encoding_unexpected() -> Self {
        loop {}
    }
}
impl From<httparse::Error> for Parse {
    fn from(err: httparse::Error) -> Parse {
        loop {}
    }
}
impl From<http::method::InvalidMethod> for Parse {
    fn from(_: http::method::InvalidMethod) -> Parse {
        loop {}
    }
}
impl From<http::status::InvalidStatusCode> for Parse {
    fn from(_: http::status::InvalidStatusCode) -> Parse {
        loop {}
    }
}
impl From<http::uri::InvalidUri> for Parse {
    fn from(_: http::uri::InvalidUri) -> Parse {
        loop {}
    }
}
impl From<http::uri::InvalidUriParts> for Parse {
    fn from(_: http::uri::InvalidUriParts) -> Parse {
        loop {}
    }
}
#[doc(hidden)]
trait AssertSendSync: Send + Sync + 'static {}
#[doc(hidden)]
impl AssertSendSync for Error {}
impl fmt::Display for TimedOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for TimedOut {}
#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    #[test]
    fn error_size_of() {
        loop {}
    }
    #[cfg(feature = "http2")]
    #[test]
    fn h2_reason_unknown() {
        loop {}
    }
    #[cfg(feature = "http2")]
    #[test]
    fn h2_reason_one_level() {
        loop {}
    }
    #[cfg(feature = "http2")]
    #[test]
    fn h2_reason_nested() {
        loop {}
    }
}
