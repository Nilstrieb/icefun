use std::error::Error as StdError;
use std::fmt;

use std::time::Duration;

use futures_util::future::{self, Either, FutureExt as _, TryFutureExt as _};

use http::uri::{Port, Scheme};
use http::{Request, Response, Uri, Version};
use tracing::{debug, trace};
use super::conn;
use super::connect::{self, sealed::Connect, Alpn, Connected, Connection};
use super::pool::{
    self, Key as PoolKey, Pool, Poolable, Pooled, Reservation,
};
#[cfg(feature = "tcp")]
use super::HttpConnector;
use crate::body::{Body, HttpBody};
use crate::common::{
    exec::BoxSendFuture, sync_wrapper::SyncWrapper, lazy as hyper_lazy, task, Future,
    Lazy, Pin, Poll,
};
use crate::rt::Executor;




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



#[must_use = "futures do nothing unless polled"]
pub struct ResponseFuture {
    inner: SyncWrapper<
        Pin<Box<dyn Future<Output = crate::Result<Response<Body>>> + Send>>,
    >,
}
#[cfg(feature = "tcp")]
impl Client<HttpConnector, Body> {
    
    
    
    
    
    
    
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
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn get(&self, uri: Uri) -> ResponseFuture
    where
        B: Default,
    {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
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
    
    
    
    pub(crate) fn pool_max_idle_per_host(&mut self, max_idle: usize) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    pub(crate) fn http1_read_buf_exact_size(&mut self, sz: usize) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    #[cfg(feature = "http1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http1")))]
    pub(crate) fn http1_max_buf_size(&mut self, max: usize) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn http1_allow_spaces_after_header_name_in_responses(
        &mut self,
        val: bool,
    ) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn http1_allow_obsolete_multiline_headers_in_responses(
        &mut self,
        val: bool,
    ) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn http1_ignore_invalid_headers_in_responses(
        &mut self,
        val: bool,
    ) -> &mut Builder {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn http1_writev(&mut self, enabled: bool) -> &mut Builder {
        loop {}
    }
    
    
    
    
    
    
    pub(crate) fn http1_title_case_headers(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn http1_preserve_header_case(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    
    
    
    pub(crate) fn http09_responses(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_only(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_initial_stream_window_size(
        &mut self,
        sz: impl Into<Option<u32>>,
    ) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_initial_connection_window_size(
        &mut self,
        sz: impl Into<Option<u32>>,
    ) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_adaptive_window(&mut self, enabled: bool) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_max_frame_size(
        &mut self,
        sz: impl Into<Option<u32>>,
    ) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    #[cfg(feature = "runtime")]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_keep_alive_interval(
        &mut self,
        interval: impl Into<Option<Duration>>,
    ) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    #[cfg(feature = "runtime")]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_keep_alive_timeout(&mut self, timeout: Duration) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    #[cfg(feature = "runtime")]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_keep_alive_while_idle(&mut self, enabled: bool) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_max_concurrent_reset_streams(
        &mut self,
        max: usize,
    ) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_max_send_buf_size(&mut self, max: usize) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    #[inline]
    pub(crate) fn retry_canceled_requests(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    
    
    
    
    
    
    #[inline]
    pub(crate) fn set_host(&mut self, val: bool) -> &mut Self {
        loop {}
    }
    
    pub(crate) fn executor<E>(&mut self, exec: E) -> &mut Self
    where
        E: Executor<BoxSendFuture> + Send + Sync + 'static,
    {
        loop {}
    }
    
    #[cfg(feature = "tcp")]
    pub(crate) fn build_http<B>(&self) -> Client<HttpConnector, B>
    where
        B: HttpBody + Send,
        B::Data: Send,
    {
        loop {}
    }
    
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
