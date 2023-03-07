use std::fmt;
use std::io;
use std::marker::PhantomData;
#[cfg(all(feature = "server", feature = "runtime"))]
use std::time::Duration;
use bytes::{Buf, Bytes};
use http::header::{HeaderValue, CONNECTION};
use http::{HeaderMap, Method, Version};
use httparse::ParserConfig;
use tokio::io::{AsyncRead, AsyncWrite};
#[cfg(all(feature = "server", feature = "runtime"))]
use tokio::time::Sleep;
use tracing::{debug, error, trace};
use super::io::Buffered;
use super::{Decoder, Encode, EncodedBuf, Encoder, Http1Transaction, ParseContext, Wants};
use crate::body::DecodedLength;
use crate::common::{task, Pin, Poll, Unpin};
use crate::headers::connection_keep_alive;
use crate::proto::{BodyLength, MessageHead};
const H2_PREFACE: &[u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
/// This handles a connection, which will have been established over an
/// `AsyncRead + AsyncWrite` (like a socket), and will likely include multiple
/// `Transaction`s over HTTP.
///
/// The connection will determine when a message begins and ends as well as
/// determine if this connection can be kept alive after the message,
/// or if it is complete.
pub(crate) struct Conn<I, B, T> {
    io: Buffered<I, EncodedBuf<B>>,
    state: State,
    _marker: PhantomData<fn(T)>,
}
impl<I, B, T> Conn<I, B, T>
where
    I: AsyncRead + AsyncWrite + Unpin,
    B: Buf,
    T: Http1Transaction,
{
    pub(crate) fn new(io: I) -> Conn<I, B, T> {
        loop {}
    }
    #[cfg(feature = "server")]
    pub(crate) fn set_flush_pipeline(&mut self, enabled: bool) {
        loop {}
    }
    pub(crate) fn set_write_strategy_queue(&mut self) {
        loop {}
    }
    pub(crate) fn set_max_buf_size(&mut self, max: usize) {
        loop {}
    }
    #[cfg(feature = "client")]
    pub(crate) fn set_read_buf_exact_size(&mut self, sz: usize) {
        loop {}
    }
    pub(crate) fn set_write_strategy_flatten(&mut self) {
        loop {}
    }
    #[cfg(feature = "client")]
    pub(crate) fn set_h1_parser_config(&mut self, parser_config: ParserConfig) {
        loop {}
    }
    pub(crate) fn set_title_case_headers(&mut self) {
        loop {}
    }
    pub(crate) fn set_preserve_header_case(&mut self) {
        loop {}
    }
    #[cfg(feature = "ffi")]
    pub(crate) fn set_preserve_header_order(&mut self) {
        loop {}
    }
    #[cfg(feature = "client")]
    pub(crate) fn set_h09_responses(&mut self) {
        loop {}
    }
    #[cfg(all(feature = "server", feature = "runtime"))]
    pub(crate) fn set_http1_header_read_timeout(&mut self, val: Duration) {
        loop {}
    }
    #[cfg(feature = "server")]
    pub(crate) fn set_allow_half_close(&mut self) {
        loop {}
    }
    #[cfg(feature = "ffi")]
    pub(crate) fn set_raw_headers(&mut self, enabled: bool) {
        loop {}
    }
    pub(crate) fn into_inner(self) -> (I, Bytes) {
        loop {}
    }
    pub(crate) fn pending_upgrade(&mut self) -> Option<crate::upgrade::Pending> {
        loop {}
    }
    pub(crate) fn is_read_closed(&self) -> bool {
        loop {}
    }
    pub(crate) fn is_write_closed(&self) -> bool {
        loop {}
    }
    pub(crate) fn can_read_head(&self) -> bool {
        loop {}
    }
    pub(crate) fn can_read_body(&self) -> bool {
        loop {}
    }
    fn should_error_on_eof(&self) -> bool {
        loop {}
    }
    fn has_h2_prefix(&self) -> bool {
        loop {}
    }
    pub(super) fn poll_read_head(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<crate::Result<(MessageHead<T::Incoming>, DecodedLength, Wants)>>> {
        loop {}
    }
    fn on_read_head_error<Z>(
        &mut self,
        e: crate::Error,
    ) -> Poll<Option<crate::Result<Z>>> {
        loop {}
    }
    pub(crate) fn poll_read_body(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<io::Result<Bytes>>> {
        loop {}
    }
    pub(crate) fn wants_read_again(&mut self) -> bool {
        loop {}
    }
    pub(crate) fn poll_read_keep_alive(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<crate::Result<()>> {
        loop {}
    }
    fn is_mid_message(&self) -> bool {
        loop {}
    }
    fn require_empty_read(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<crate::Result<()>> {
        loop {}
    }
    fn mid_message_detect_eof(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<crate::Result<()>> {
        loop {}
    }
    fn force_io_read(&mut self, cx: &mut task::Context<'_>) -> Poll<io::Result<usize>> {
        loop {}
    }
    fn maybe_notify(&mut self, cx: &mut task::Context<'_>) {
        loop {}
    }
    fn try_keep_alive(&mut self, cx: &mut task::Context<'_>) {
        loop {}
    }
    pub(crate) fn can_write_head(&self) -> bool {
        loop {}
    }
    pub(crate) fn can_write_body(&self) -> bool {
        loop {}
    }
    pub(crate) fn can_buffer_body(&self) -> bool {
        loop {}
    }
    pub(crate) fn write_head(
        &mut self,
        head: MessageHead<T::Outgoing>,
        body: Option<BodyLength>,
    ) {
        loop {}
    }
    pub(crate) fn write_full_msg(&mut self, head: MessageHead<T::Outgoing>, body: B) {
        loop {}
    }
    fn encode_head(
        &mut self,
        mut head: MessageHead<T::Outgoing>,
        body: Option<BodyLength>,
    ) -> Option<Encoder> {
        loop {}
    }
    fn fix_keep_alive(&mut self, head: &mut MessageHead<T::Outgoing>) {
        loop {}
    }
    fn enforce_version(&mut self, head: &mut MessageHead<T::Outgoing>) {
        loop {}
    }
    pub(crate) fn write_body(&mut self, chunk: B) {
        loop {}
    }
    pub(crate) fn write_body_and_end(&mut self, chunk: B) {
        loop {}
    }
    pub(crate) fn end_body(&mut self) -> crate::Result<()> {
        loop {}
    }
    fn on_parse_error(&mut self, err: crate::Error) -> crate::Result<()> {
        loop {}
    }
    pub(crate) fn poll_flush(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        loop {}
    }
    pub(crate) fn poll_shutdown(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        loop {}
    }
    /// If the read side can be cheaply drained, do so. Otherwise, close.
    pub(super) fn poll_drain_or_close_read(&mut self, cx: &mut task::Context<'_>) {
        loop {}
    }
    pub(crate) fn close_read(&mut self) {
        loop {}
    }
    pub(crate) fn close_write(&mut self) {
        loop {}
    }
    #[cfg(feature = "server")]
    pub(crate) fn disable_keep_alive(&mut self) {
        loop {}
    }
    pub(crate) fn take_error(&mut self) -> crate::Result<()> {
        loop {}
    }
    pub(super) fn on_upgrade(&mut self) -> crate::upgrade::OnUpgrade {
        loop {}
    }
}
impl<I, B: Buf, T> fmt::Debug for Conn<I, B, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<I: Unpin, B, T> Unpin for Conn<I, B, T> {}
struct State {
    allow_half_close: bool,
    /// Re-usable HeaderMap to reduce allocating new ones.
    cached_headers: Option<HeaderMap>,
    /// If an error occurs when there wasn't a direct way to return it
    /// back to the user, this is set.
    error: Option<crate::Error>,
    /// Current keep-alive status.
    keep_alive: KA,
    /// If mid-message, the HTTP Method that started it.
    ///
    /// This is used to know things such as if the message can include
    /// a body or not.
    method: Option<Method>,
    h1_parser_config: ParserConfig,
    #[cfg(all(feature = "server", feature = "runtime"))]
    h1_header_read_timeout: Option<Duration>,
    #[cfg(all(feature = "server", feature = "runtime"))]
    h1_header_read_timeout_fut: Option<Pin<Box<Sleep>>>,
    #[cfg(all(feature = "server", feature = "runtime"))]
    h1_header_read_timeout_running: bool,
    preserve_header_case: bool,
    #[cfg(feature = "ffi")]
    preserve_header_order: bool,
    title_case_headers: bool,
    h09_responses: bool,
    /// If set, called with each 1xx informational response received for
    /// the current request. MUST be unset after a non-1xx response is
    /// received.
    #[cfg(feature = "ffi")]
    on_informational: Option<crate::ffi::OnInformational>,
    #[cfg(feature = "ffi")]
    raw_headers: bool,
    /// Set to true when the Dispatcher should poll read operations
    /// again. See the `maybe_notify` method for more.
    notify_read: bool,
    /// State of allowed reads
    reading: Reading,
    /// State of allowed writes
    writing: Writing,
    /// An expected pending HTTP upgrade.
    upgrade: Option<crate::upgrade::Pending>,
    /// Either HTTP/1.0 or 1.1 connection
    version: Version,
}
#[derive(Debug)]
enum Reading {
    Init,
    Continue(Decoder),
    Body(Decoder),
    KeepAlive,
    Closed,
}
enum Writing {
    Init,
    Body(Encoder),
    KeepAlive,
    Closed,
}
impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl fmt::Debug for Writing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl std::ops::BitAndAssign<bool> for KA {
    fn bitand_assign(&mut self, enabled: bool) {
        loop {}
    }
}
#[derive(Clone, Copy, Debug)]
enum KA {
    Idle,
    Busy,
    Disabled,
}
impl Default for KA {
    fn default() -> KA {
        loop {}
    }
}
impl KA {
    fn idle(&mut self) {
        loop {}
    }
    fn busy(&mut self) {
        loop {}
    }
    fn disable(&mut self) {
        loop {}
    }
    fn status(&self) -> KA {
        loop {}
    }
}
impl State {
    fn close(&mut self) {
        loop {}
    }
    fn close_read(&mut self) {
        loop {}
    }
    fn close_write(&mut self) {
        loop {}
    }
    fn wants_keep_alive(&self) -> bool {
        loop {}
    }
    fn try_keep_alive<T: Http1Transaction>(&mut self) {
        loop {}
    }
    fn disable_keep_alive(&mut self) {
        loop {}
    }
    fn busy(&mut self) {
        loop {}
    }
    fn idle<T: Http1Transaction>(&mut self) {
        loop {}
    }
    fn is_idle(&self) -> bool {
        loop {}
    }
    fn is_read_closed(&self) -> bool {
        loop {}
    }
    fn is_write_closed(&self) -> bool {
        loop {}
    }
    fn prepare_upgrade(&mut self) -> crate::upgrade::OnUpgrade {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_read_head_short(b: &mut ::test::Bencher) {
        loop {}
    }
}
