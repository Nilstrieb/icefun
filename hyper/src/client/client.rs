use std::error::Error as StdError;
use std::fmt;
use std::mem;
use std::time::Duration;
use futures_channel::oneshot;
use futures_util::future::{self, Either, FutureExt as _, TryFutureExt as _};
use http::header::{HeaderValue, HOST};
use http::uri::{Port, Scheme};
use http::{Method, Request, Response, Uri, Version};
use tracing::{debug, trace, warn};
use super::conn;
use super::connect::{self, sealed::Connect, Alpn, Connected, Connection};
use super::pool::{
    self, CheckoutIsClosedError, Key as PoolKey, Pool, Poolable, Pooled, Reservation,
};
#[cfg(feature = "tcp")]
use super::HttpConnector;
use crate::body::{Body, HttpBody};
use crate::common::{
    exec::BoxSendFuture, sync_wrapper::SyncWrapper, lazy as hyper_lazy, task, Future,
    Lazy, Pin, Poll,
};
use crate::rt::Executor;
/// A Client to make outgoing HTTP requests.
///
/// `Client` is cheap to clone and cloning is the recommended way to share a `Client`. The
/// underlying connection pool will be reused.
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
pub struct Client<C, B = Body> {
    config: Config,
    conn_builder: conn::Builder,
    connector: C,
    pool: Pool<PoolClient<B>>,
}
#[derive(Clone, Copy, Debug)]
struct Config {
    retry_canceled_requests: bool,
    set_host: bool,
    ver: Ver,
}
/// A `Future` that will resolve to an HTTP Response.
///
/// This is returned by `Client::request` (and `Client::get`).
#[must_use = "futures do nothing unless polled"]
pub struct ResponseFuture {
    inner: SyncWrapper<
        Pin<Box<dyn Future<Output = crate::Result<Response<Body>>> + Send>>,
    >,
}
#[cfg(feature = "tcp")]
impl Client<HttpConnector, Body> {
    /// Create a new Client with the default [config](Builder).
    ///
    /// # Note
    ///
    /// The default connector does **not** handle TLS. Speaking to `https`
    /// destinations will require [configuring a connector that implements
    /// TLS](https://hyper.rs/guides/client/configuration).
    #[cfg_attr(docsrs, doc(cfg(feature = "tcp")))]
    #[inline]
    pub(crate) fn new() -> Client<HttpConnector, Body> {
        loop {}
    }
}
#[cfg(feature = "tcp")]
impl Default for Client<HttpConnector, Body> {
    fn default() -> Client<HttpConnector, Body> {
        loop {}
    }
}
impl Client<(), Body> {
    /// Create a builder to configure a new `Client`.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature  = "runtime")]
    /// # fn run () {
    /// use std::time::Duration;
    /// use hyper::Client;
    ///
    /// let client = Client::builder()
    ///     .pool_idle_timeout(Duration::from_secs(30))
    ///     .http2_only(true)
    ///     .build_http();
    /// # let infer: Client<_, hyper::Body> = client;
    /// # drop(infer);
    /// # }
    /// # fn main() {}
    /// ```
    #[inline]
    pub(crate) fn builder() -> Builder {
        loop {}
    }
}
impl<C, B> Client<C, B>
where
    C: Connect + Clone + Send + Sync + 'static,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    /// Send a `GET` request to the supplied `Uri`.
    ///
    /// # Note
    ///
    /// This requires that the `HttpBody` type have a `Default` implementation.
    /// It *should* return an "empty" version of itself, such that
    /// `HttpBody::is_end_stream` is `true`.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature  = "runtime")]
    /// # fn run () {
    /// use hyper::{Client, Uri};
    ///
    /// let client = Client::new();
    ///
    /// let future = client.get(Uri::from_static("http://httpbin.org/ip"));
    /// # }
    /// # fn main() {}
    /// ```
    pub(crate) fn get(&self, uri: Uri) -> ResponseFuture
    where
        B: Default,
    {
        loop {}
    }
    /// Send a constructed `Request` using this `Client`.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature  = "runtime")]
    /// # fn run () {
    /// use hyper::{Body, Method, Client, Request};
    ///
    /// let client = Client::new();
    ///
    /// let req = Request::builder()
    ///     .method(Method::POST)
    ///     .uri("http://httpbin.org/post")
    ///     .body(Body::from("Hallo!"))
    ///     .expect("request builder");
    ///
    /// let future = client.request(req);
    /// # }
    /// # fn main() {}
    /// ```
    pub(crate) fn request(&self, mut req: Request<B>) -> ResponseFuture {
        loop {}
    }
    async fn retryably_send_request(
        self,
        mut req: Request<B>,
        pool_key: PoolKey,
    ) -> crate::Result<Response<Body>> {
        loop {}
    }
    async fn send_request(
        &self,
        mut req: Request<B>,
        pool_key: PoolKey,
    ) -> Result<Response<Body>, ClientError<B>> {
        loop {}
    }
    async fn connection_for(
        &self,
        pool_key: PoolKey,
    ) -> Result<Pooled<PoolClient<B>>, ClientConnectError> {
        loop {}
    }
    fn connect_to(
        &self,
        pool_key: PoolKey,
    ) -> impl Lazy<Output = crate::Result<Pooled<PoolClient<B>>>> + Unpin {
        let executor = self.conn_builder.exec.clone();
        let pool = self.pool.clone();
        #[cfg(not(feature = "http2"))]
        let conn_builder = self.conn_builder.clone();
        #[cfg(feature = "http2")]
        let mut conn_builder = self.conn_builder.clone();
        let ver = self.config.ver;
        let is_ver_h2 = ver == Ver::Http2;
        let connector = self.connector.clone();
        let dst = domain_as_uri(pool_key.clone());
        hyper_lazy(move || {
            let connecting = match pool.connecting(&pool_key, ver) {
                Some(lock) => lock,
                None => {
                    let canceled = crate::Error::new_canceled()
                        .with("HTTP/2 connection in progress");
                    return Either::Right(future::err(canceled));
                }
            };
            Either::Left(
                connector
                    .connect(connect::sealed::Internal, dst)
                    .map_err(crate::Error::new_connect)
                    .and_then(move |io| {
                        let connected = io.connected();
                        let connecting = if connected.alpn == Alpn::H2 && !is_ver_h2 {
                            match connecting.alpn_h2(&pool) {
                                Some(lock) => {
                                    trace!("ALPN negotiated h2, updating pool");
                                    lock
                                }
                                None => {
                                    let canceled = crate::Error::new_canceled()
                                        .with("ALPN upgraded to HTTP/2");
                                    return Either::Right(future::err(canceled));
                                }
                            }
                        } else {
                            connecting
                        };
                        #[cfg_attr(not(feature = "http2"), allow(unused))]
                        let is_h2 = is_ver_h2 || connected.alpn == Alpn::H2;
                        #[cfg(feature = "http2")]
                        {
                            conn_builder.http2_only(is_h2);
                        }
                        Either::Left(
                            Box::pin(async move {
                                let (tx, conn) = conn_builder.handshake(io).await?;
                                trace!(
                                    "handshake complete, spawning background dispatcher task"
                                );
                                executor
                                    .execute(
                                        conn
                                            .map_err(|e| debug!("client connection error: {}", e))
                                            .map(|_| ()),
                                    );
                                let tx = tx.when_ready().await?;
                                let tx = {
                                    #[cfg(feature = "http2")]
                                    {
                                        if is_h2 {
                                            PoolTx::Http2(tx.into_http2())
                                        } else {
                                            PoolTx::Http1(tx)
                                        }
                                    }
                                    #[cfg(not(feature = "http2"))] PoolTx::Http1(tx)
                                };
                                Ok(
                                    pool
                                        .pooled(
                                            connecting,
                                            PoolClient {
                                                conn_info: connected,
                                                tx,
                                            },
                                        ),
                                )
                            }),
                        )
                    }),
            )
        })
    }
}
impl<C, B> tower_service::Service<Request<B>> for Client<C, B>
where
    C: Connect + Clone + Send + Sync + 'static,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Response = Response<Body>;
    type Error = crate::Error;
    type Future = ResponseFuture;
    fn poll_ready(
        &mut self,
        _: &mut task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn call(&mut self, req: Request<B>) -> Self::Future {
        loop {}
    }
}
impl<C, B> tower_service::Service<Request<B>> for &'_ Client<C, B>
where
    C: Connect + Clone + Send + Sync + 'static,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Response = Response<Body>;
    type Error = crate::Error;
    type Future = ResponseFuture;
    fn poll_ready(
        &mut self,
        _: &mut task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn call(&mut self, req: Request<B>) -> Self::Future {
        loop {}
    }
}
impl<C: Clone, B> Clone for Client<C, B> {
    fn clone(&self) -> Client<C, B> {
        loop {}
    }
}
impl<C, B> fmt::Debug for Client<C, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl ResponseFuture {
    fn new<F>(value: F) -> Self
    where
        F: Future<Output = crate::Result<Response<Body>>> + Send + 'static,
    {
        loop {}
    }
    fn error_version(ver: Version) -> Self {
        loop {}
    }
}
impl fmt::Debug for ResponseFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Future for ResponseFuture {
    type Output = crate::Result<Response<Body>>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
struct PoolClient<B> {
    conn_info: Connected,
    tx: PoolTx<B>,
}
enum PoolTx<B> {
    Http1(conn::SendRequest<B>),
    #[cfg(feature = "http2")]
    Http2(conn::Http2SendRequest<B>),
}
impl<B> PoolClient<B> {
    fn poll_ready(&mut self, cx: &mut task::Context<'_>) -> Poll<crate::Result<()>> {
        loop {}
    }
    fn is_http1(&self) -> bool {
        loop {}
    }
    fn is_http2(&self) -> bool {
        loop {}
    }
    fn is_ready(&self) -> bool {
        loop {}
    }
    fn is_closed(&self) -> bool {
        loop {}
    }
}
impl<B: HttpBody + 'static> PoolClient<B> {
    fn send_request_retryable(
        &mut self,
        req: Request<B>,
    ) -> impl Future<Output = Result<Response<Body>, (crate::Error, Option<Request<B>>)>>
    where
        B: Send,
    {
        match self.tx {
            #[cfg(not(feature = "http2"))]
            PoolTx::Http1(ref mut tx) => tx.send_request_retryable(req),
            #[cfg(feature = "http2")]
            PoolTx::Http1(ref mut tx) => Either::Left(tx.send_request_retryable(req)),
            #[cfg(feature = "http2")]
            PoolTx::Http2(ref mut tx) => Either::Right(tx.send_request_retryable(req)),
        }
    }
}
impl<B> Poolable for PoolClient<B>
where
    B: Send + 'static,
{
    fn is_open(&self) -> bool {
        loop {}
    }
    fn reserve(self) -> Reservation<Self> {
        loop {}
    }
    fn can_share(&self) -> bool {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
enum ClientError<B> {
    Normal(crate::Error),
    Canceled { connection_reused: bool, req: Request<B>, reason: crate::Error },
}
impl<B> ClientError<B> {
    fn map_with_reused(
        conn_reused: bool,
    ) -> impl Fn((crate::Error, Option<Request<B>>)) -> Self {
        move |(err, orig_req)| {
            if let Some(req) = orig_req {
                ClientError::Canceled {
                    connection_reused: conn_reused,
                    reason: err,
                    req,
                }
            } else {
                ClientError::Normal(err)
            }
        }
    }
}
enum ClientConnectError {
    Normal(crate::Error),
    H2CheckoutIsClosed(crate::Error),
}
/// A marker to identify what version a pooled connection is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Ver {
    Auto,
    Http2,
}
fn origin_form(uri: &mut Uri) {
    loop {}
}
fn absolute_form(uri: &mut Uri) {
    loop {}
}
fn authority_form(uri: &mut Uri) {
    loop {}
}
fn extract_domain(uri: &mut Uri, is_http_connect: bool) -> crate::Result<PoolKey> {
    loop {}
}
fn domain_as_uri((scheme, auth): PoolKey) -> Uri {
    loop {}
}
fn set_scheme(uri: &mut Uri, scheme: Scheme) {
    loop {}
}
fn get_non_default_port(uri: &Uri) -> Option<Port<&str>> {
    loop {}
}
fn is_schema_secure(uri: &Uri) -> bool {
    loop {}
}
/// A builder to configure a new [`Client`](Client).
///
/// # Example
///
/// ```
/// # #[cfg(feature  = "runtime")]
/// # fn run () {
/// use std::time::Duration;
/// use hyper::Client;
///
/// let client = Client::builder()
///     .pool_idle_timeout(Duration::from_secs(30))
///     .http2_only(true)
///     .build_http();
/// # let infer: Client<_, hyper::Body> = client;
/// # drop(infer);
/// # }
/// # fn main() {}
/// ```
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
#[derive(Clone)]
pub struct Builder {
    client_config: Config,
    conn_builder: conn::Builder,
    pool_config: pool::Config,
}
impl Default for Builder {
    fn default() -> Self {
        loop {}
    }
}
impl Builder {
    #[doc(hidden)]
    #[deprecated(
        note = "name is confusing, to disable the connection pool, call pool_max_idle_per_host(0)"
    )]
    pub(crate) fn keep_alive(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    #[doc(hidden)]
    #[deprecated(note = "renamed to `pool_idle_timeout`")]
    pub(crate) fn keep_alive_timeout<D>(&mut self, val: D) -> &mut Self
    where
        D: Into<Option<Duration>>,
    {
        loop {}
    }
    /// Set an optional timeout for idle sockets being kept-alive.
    ///
    /// Pass `None` to disable timeout.
    ///
    /// Default is 90 seconds.
    pub(crate) fn pool_idle_timeout<D>(&mut self, val: D) -> &mut Self
    where
        D: Into<Option<Duration>>,
    {
        loop {}
    }
    #[doc(hidden)]
    #[deprecated(note = "renamed to `pool_max_idle_per_host`")]
    pub(crate) fn max_idle_per_host(&mut self, max_idle: usize) -> &mut Self {
        loop {}
    }
    /// Sets the maximum idle connection per host allowed in the pool.
    ///
    /// Default is `usize::MAX` (no limit).
    pub(crate) fn pool_max_idle_per_host(&mut self, max_idle: usize) -> &mut Self {
        loop {}
    }
    /// Sets the exact size of the read buffer to *always* use.
    ///
    /// Note that setting this option unsets the `http1_max_buf_size` option.
    ///
    /// Default is an adaptive read buffer.
    pub(crate) fn http1_read_buf_exact_size(&mut self, sz: usize) -> &mut Self {
        loop {}
    }
    /// Set the maximum buffer size for the connection.
    ///
    /// Default is ~400kb.
    ///
    /// Note that setting this option unsets the `http1_read_exact_buf_size` option.
    ///
    /// # Panics
    ///
    /// The minimum value allowed is 8192. This method panics if the passed `max` is less than the minimum.
    #[cfg(feature = "http1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http1")))]
    pub(crate) fn http1_max_buf_size(&mut self, max: usize) -> &mut Self {
        loop {}
    }
    /// Set whether HTTP/1 connections will accept spaces between header names
    /// and the colon that follow them in responses.
    ///
    /// Newline codepoints (`\r` and `\n`) will be transformed to spaces when
    /// parsing.
    ///
    /// You probably don't need this, here is what [RFC 7230 Section 3.2.4.] has
    /// to say about it:
    ///
    /// > No whitespace is allowed between the header field-name and colon. In
    /// > the past, differences in the handling of such whitespace have led to
    /// > security vulnerabilities in request routing and response handling. A
    /// > server MUST reject any received request message that contains
    /// > whitespace between a header field-name and colon with a response code
    /// > of 400 (Bad Request). A proxy MUST remove any such whitespace from a
    /// > response message before forwarding the message downstream.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    ///
    /// [RFC 7230 Section 3.2.4.]: https://tools.ietf.org/html/rfc7230#section-3.2.4
    pub(crate) fn http1_allow_spaces_after_header_name_in_responses(
        &mut self,
        val: bool,
    ) -> &mut Self {
        loop {}
    }
    /// Set whether HTTP/1 connections will accept obsolete line folding for
    /// header values.
    ///
    /// You probably don't need this, here is what [RFC 7230 Section 3.2.4.] has
    /// to say about it:
    ///
    /// > A server that receives an obs-fold in a request message that is not
    /// > within a message/http container MUST either reject the message by
    /// > sending a 400 (Bad Request), preferably with a representation
    /// > explaining that obsolete line folding is unacceptable, or replace
    /// > each received obs-fold with one or more SP octets prior to
    /// > interpreting the field value or forwarding the message downstream.
    ///
    /// > A proxy or gateway that receives an obs-fold in a response message
    /// > that is not within a message/http container MUST either discard the
    /// > message and replace it with a 502 (Bad Gateway) response, preferably
    /// > with a representation explaining that unacceptable line folding was
    /// > received, or replace each received obs-fold with one or more SP
    /// > octets prior to interpreting the field value or forwarding the
    /// > message downstream.
    ///
    /// > A user agent that receives an obs-fold in a response message that is
    /// > not within a message/http container MUST replace each received
    /// > obs-fold with one or more SP octets prior to interpreting the field
    /// > value.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    ///
    /// [RFC 7230 Section 3.2.4.]: https://tools.ietf.org/html/rfc7230#section-3.2.4
    pub(crate) fn http1_allow_obsolete_multiline_headers_in_responses(
        &mut self,
        val: bool,
    ) -> &mut Self {
        loop {}
    }
    /// Sets whether invalid header lines should be silently ignored in HTTP/1 responses.
    ///
    /// This mimicks the behaviour of major browsers. You probably don't want this.
    /// You should only want this if you are implementing a proxy whose main
    /// purpose is to sit in front of browsers whose users access arbitrary content
    /// which may be malformed, and they expect everything that works without
    /// the proxy to keep working with the proxy.
    ///
    /// This option will prevent Hyper's client from returning an error encountered
    /// when parsing a header, except if the error was caused by the character NUL
    /// (ASCII code 0), as Chrome specifically always reject those.
    ///
    /// The ignorable errors are:
    /// * empty header names;
    /// * characters that are not allowed in header names, except for `\0` and `\r`;
    /// * when `allow_spaces_after_header_name_in_responses` is not enabled,
    ///   spaces and tabs between the header name and the colon;
    /// * missing colon between header name and colon;
    /// * characters that are not allowed in header values except for `\0` and `\r`.
    ///
    /// If an ignorable error is encountered, the parser tries to find the next
    /// line in the input to resume parsing the rest of the headers. An error
    /// will be emitted nonetheless if it finds `\0` or a lone `\r` while
    /// looking for the next line.
    pub(crate) fn http1_ignore_invalid_headers_in_responses(
        &mut self,
        val: bool,
    ) -> &mut Builder {
        loop {}
    }
    /// Set whether HTTP/1 connections should try to use vectored writes,
    /// or always flatten into a single buffer.
    ///
    /// Note that setting this to false may mean more copies of body data,
    /// but may also improve performance when an IO transport doesn't
    /// support vectored writes well, such as most TLS implementations.
    ///
    /// Setting this to true will force hyper to use queued strategy
    /// which may eliminate unnecessary cloning on some TLS backends
    ///
    /// Default is `auto`. In this mode hyper will try to guess which
    /// mode to use
    pub(crate) fn http1_writev(&mut self, enabled: bool) -> &mut Builder {
        loop {}
    }
    /// Set whether HTTP/1 connections will write header names as title case at
    /// the socket level.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    pub(crate) fn http1_title_case_headers(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    /// Set whether to support preserving original header cases.
    ///
    /// Currently, this will record the original cases received, and store them
    /// in a private extension on the `Response`. It will also look for and use
    /// such an extension in any provided `Request`.
    ///
    /// Since the relevant extension is still private, there is no way to
    /// interact with the original cases. The only effect this can have now is
    /// to forward the cases in a proxy-like fashion.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    pub(crate) fn http1_preserve_header_case(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    /// Set whether HTTP/0.9 responses should be tolerated.
    ///
    /// Default is false.
    pub(crate) fn http09_responses(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    /// Set whether the connection **must** use HTTP/2.
    ///
    /// The destination must either allow HTTP2 Prior Knowledge, or the
    /// `Connect` should be configured to do use ALPN to upgrade to `h2`
    /// as part of the connection process. This will not make the `Client`
    /// utilize ALPN by itself.
    ///
    /// Note that setting this to true prevents HTTP/1 from being allowed.
    ///
    /// Default is false.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_only(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    /// Sets the [`SETTINGS_INITIAL_WINDOW_SIZE`][spec] option for HTTP2
    /// stream-level flow control.
    ///
    /// Passing `None` will do nothing.
    ///
    /// If not set, hyper will use a default.
    ///
    /// [spec]: https://http2.github.io/http2-spec/#SETTINGS_INITIAL_WINDOW_SIZE
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_initial_stream_window_size(
        &mut self,
        sz: impl Into<Option<u32>>,
    ) -> &mut Self {
        loop {}
    }
    /// Sets the max connection-level flow control for HTTP2
    ///
    /// Passing `None` will do nothing.
    ///
    /// If not set, hyper will use a default.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_initial_connection_window_size(
        &mut self,
        sz: impl Into<Option<u32>>,
    ) -> &mut Self {
        loop {}
    }
    /// Sets whether to use an adaptive flow control.
    ///
    /// Enabling this will override the limits set in
    /// `http2_initial_stream_window_size` and
    /// `http2_initial_connection_window_size`.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_adaptive_window(&mut self, enabled: bool) -> &mut Self {
        loop {}
    }
    /// Sets the maximum frame size to use for HTTP2.
    ///
    /// Passing `None` will do nothing.
    ///
    /// If not set, hyper will use a default.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_max_frame_size(
        &mut self,
        sz: impl Into<Option<u32>>,
    ) -> &mut Self {
        loop {}
    }
    /// Sets an interval for HTTP2 Ping frames should be sent to keep a
    /// connection alive.
    ///
    /// Pass `None` to disable HTTP2 keep-alive.
    ///
    /// Default is currently disabled.
    ///
    /// # Cargo Feature
    ///
    /// Requires the `runtime` cargo feature to be enabled.
    #[cfg(feature = "runtime")]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_keep_alive_interval(
        &mut self,
        interval: impl Into<Option<Duration>>,
    ) -> &mut Self {
        loop {}
    }
    /// Sets a timeout for receiving an acknowledgement of the keep-alive ping.
    ///
    /// If the ping is not acknowledged within the timeout, the connection will
    /// be closed. Does nothing if `http2_keep_alive_interval` is disabled.
    ///
    /// Default is 20 seconds.
    ///
    /// # Cargo Feature
    ///
    /// Requires the `runtime` cargo feature to be enabled.
    #[cfg(feature = "runtime")]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_keep_alive_timeout(&mut self, timeout: Duration) -> &mut Self {
        loop {}
    }
    /// Sets whether HTTP2 keep-alive should apply while the connection is idle.
    ///
    /// If disabled, keep-alive pings are only sent while there are open
    /// request/responses streams. If enabled, pings are also sent when no
    /// streams are active. Does nothing if `http2_keep_alive_interval` is
    /// disabled.
    ///
    /// Default is `false`.
    ///
    /// # Cargo Feature
    ///
    /// Requires the `runtime` cargo feature to be enabled.
    #[cfg(feature = "runtime")]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_keep_alive_while_idle(&mut self, enabled: bool) -> &mut Self {
        loop {}
    }
    /// Sets the maximum number of HTTP2 concurrent locally reset streams.
    ///
    /// See the documentation of [`h2::client::Builder::max_concurrent_reset_streams`] for more
    /// details.
    ///
    /// The default value is determined by the `h2` crate.
    ///
    /// [`h2::client::Builder::max_concurrent_reset_streams`]: https://docs.rs/h2/client/struct.Builder.html#method.max_concurrent_reset_streams
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_max_concurrent_reset_streams(
        &mut self,
        max: usize,
    ) -> &mut Self {
        loop {}
    }
    /// Set the maximum write buffer size for each HTTP/2 stream.
    ///
    /// Default is currently 1MB, but may change.
    ///
    /// # Panics
    ///
    /// The value must be no larger than `u32::MAX`.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_max_send_buf_size(&mut self, max: usize) -> &mut Self {
        loop {}
    }
    /// Set whether to retry requests that get disrupted before ever starting
    /// to write.
    ///
    /// This means a request that is queued, and gets given an idle, reused
    /// connection, and then encounters an error immediately as the idle
    /// connection was found to be unusable.
    ///
    /// When this is set to `false`, the related `ResponseFuture` would instead
    /// resolve to an `Error::Cancel`.
    ///
    /// Default is `true`.
    #[inline]
    pub(crate) fn retry_canceled_requests(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    /// Set whether to automatically add the `Host` header to requests.
    ///
    /// If true, and a request does not include a `Host` header, one will be
    /// added automatically, derived from the authority of the `Uri`.
    ///
    /// Default is `true`.
    #[inline]
    pub(crate) fn set_host(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    /// Provide an executor to execute background `Connection` tasks.
    pub(crate) fn executor<E>(&mut self, exec: E) -> &mut Self
    where
        E: Executor<BoxSendFuture> + Send + Sync + 'static,
    {
        loop {}
    }
    /// Builder a client with this configuration and the default `HttpConnector`.
    #[cfg(feature = "tcp")]
    pub(crate) fn build_http<B>(&self) -> Client<HttpConnector, B>
    where
        B: HttpBody + Send,
        B::Data: Send,
    {
        loop {}
    }
    /// Combine the configuration of this builder with a connector to create a `Client`.
    pub(crate) fn build<C, B>(&self, connector: C) -> Client<C, B>
    where
        C: Connect + Clone,
        B: HttpBody + Send,
        B::Data: Send,
    {
        loop {}
    }
}
impl fmt::Debug for Builder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
#[cfg(test)]
mod unit_tests {
    use super::*;
    #[test]
    fn response_future_is_sync() {
        loop {}
    }
    #[test]
    fn set_relative_uri_with_implicit_path() {
        loop {}
    }
    #[test]
    fn test_origin_form() {
        loop {}
    }
    #[test]
    fn test_absolute_form() {
        loop {}
    }
    #[test]
    fn test_authority_form() {
        loop {}
    }
    #[test]
    fn test_extract_domain_connect_no_port() {
        loop {}
    }
    #[test]
    fn test_is_secure() {
        loop {}
    }
    #[test]
    fn test_get_non_default_port() {
        loop {}
    }
}
