//! Error and Result module.
use std::error::Error as StdError;
use std::fmt;
/// Result type often returned from methods that can have hyper `Error`s.
pub type Result<T> = std::result::Result<T, Error>;
type Cause = Box<dyn StdError + Send + Sync>;
/// Represents errors that can occur handling HTTP streams.
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
    /// A message reached EOF, but is not complete.
    #[allow(unused)]
    IncompleteMessage,
    /// A connection received a message (or bytes) when not waiting for one.
    #[cfg(feature = "http1")]
    UnexpectedMessage,
    /// A pending item was dropped before ever being processed.
    Canceled,
    /// Indicates a channel (client or body sender) is closed.
    ChannelClosed,
    /// An `io::Error` that occurred while trying to read or write to a network stream.
    #[cfg(any(feature = "http1", feature = "http2"))]
    Io,
    /// Error occurred while connecting.
    #[allow(unused)]
    Connect,
    /// Error creating a TcpListener.
    #[cfg(all(feature = "tcp", feature = "server"))]
    Listen,
    /// Error accepting on an Incoming stream.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    Accept,
    /// User took too long to send headers
    #[cfg(all(feature = "http1", feature = "server", feature = "runtime"))]
    HeaderTimeout,
    /// Error while reading a body from connection.
    #[cfg(any(feature = "http1", feature = "http2", feature = "stream"))]
    Body,
    /// Error while writing a body to connection.
    #[cfg(any(feature = "http1", feature = "http2"))]
    BodyWrite,
    /// Error calling AsyncWrite::shutdown()
    #[cfg(feature = "http1")]
    Shutdown,
    /// A general error from h2.
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
    /// Error calling user's HttpBody::poll_data().
    #[cfg(any(feature = "http1", feature = "http2"))]
    Body,
    /// The user aborted writing of the outgoing body.
    BodyWriteAborted,
    /// Error calling user's MakeService.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    MakeService,
    /// Error from future of user's Service.
    #[cfg(any(feature = "http1", feature = "http2"))]
    Service,
    /// User tried to send a certain header in an unexpected context.
    ///
    /// For example, sending both `content-length` and `transfer-encoding`.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "server")]
    UnexpectedHeader,
    /// User tried to create a Request with bad version.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    UnsupportedVersion,
    /// User tried to create a CONNECT Request with the Client.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    UnsupportedRequestMethod,
    /// User tried to respond with a 1xx (not 101) response code.
    #[cfg(feature = "http1")]
    #[cfg(feature = "server")]
    UnsupportedStatusCode,
    /// User tried to send a Request with Client with non-absolute URI.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    AbsoluteUriRequired,
    /// User tried polling for an upgrade that doesn't exist.
    NoUpgrade,
    /// User polled for an upgrade, but low-level API is not using upgrades.
    #[cfg(feature = "http1")]
    ManualUpgrade,
    /// User called `server::Connection::without_shutdown()` on an HTTP/2 conn.
    #[cfg(feature = "server")]
    WithoutShutdownNonHttp1,
    /// The dispatch task is gone.
    #[cfg(feature = "client")]
    DispatchGone,
    /// User aborted in an FFI callback.
    #[cfg(feature = "ffi")]
    AbortedByCallback,
}
#[derive(Debug)]
pub(super) struct TimedOut;
impl Error {
    /// Returns true if this was an HTTP parse error.
    pub(crate) fn is_parse(&self) -> bool {
        loop {}
    }
    /// Returns true if this was an HTTP parse error caused by a message that was too large.
    pub(crate) fn is_parse_too_large(&self) -> bool {
        loop {}
    }
    /// Returns true if this was an HTTP parse error caused by an invalid response status code or
    /// reason phrase.
    pub(crate) fn is_parse_status(&self) -> bool {
        loop {}
    }
    /// Returns true if this error was caused by user code.
    pub(crate) fn is_user(&self) -> bool {
        loop {}
    }
    /// Returns true if this was about a `Request` that was canceled.
    pub(crate) fn is_canceled(&self) -> bool {
        loop {}
    }
    /// Returns true if a sender's channel is closed.
    pub(crate) fn is_closed(&self) -> bool {
        loop {}
    }
    /// Returns true if this was an error from `Connect`.
    pub(crate) fn is_connect(&self) -> bool {
        loop {}
    }
    /// Returns true if the connection closed before a message could complete.
    pub(crate) fn is_incomplete_message(&self) -> bool {
        loop {}
    }
    /// Returns true if the body write was aborted.
    pub(crate) fn is_body_write_aborted(&self) -> bool {
        loop {}
    }
    /// Returns true if the error was caused by a timeout.
    pub(crate) fn is_timeout(&self) -> bool {
        loop {}
    }
    /// Consumes the error, returning its cause.
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
    /// The error's standalone message, without the message from the source.
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
