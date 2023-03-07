//! Lower-level client connection API.
//!
//! The types in this module are to provide a lower-level API based around a
//! single connection. Connecting to a host, pooling connections, and the like
//! are not handled at this level. This module provides the building blocks to
//! customize those things externally.
//!
//! If don't have need to manage connections yourself, consider using the
//! higher-level [Client](super) API.
//!
//! ## Example
//! A simple example that uses the `SendRequest` struct to talk HTTP over a Tokio TCP stream
//! ```no_run
//! # #[cfg(all(feature = "client", feature = "http1", feature = "runtime"))]
//! # mod rt {
//! use tower::ServiceExt;
//! use http::{Request, StatusCode};
//! use hyper::{client::conn, Body};
//! use tokio::net::TcpStream;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let target_stream = TcpStream::connect("example.com:80").await?;
//!
//!     let (mut request_sender, connection) = conn::handshake(target_stream).await?;
//!
//!     // spawn a task to poll the connection and drive the HTTP state
//!     tokio::spawn(async move {
//!         if let Err(e) = connection.await {
//!             eprintln!("Error in connection: {}", e);
//!         }
//!     });
//!
//!     let request = Request::builder()
//!         // We need to manually add the host header because SendRequest does not
//!         .header("Host", "example.com")
//!         .method("GET")
//!         .body(Body::from(""))?;
//!     let response = request_sender.send_request(request).await?;
//!     assert!(response.status() == StatusCode::OK);
//!
//!     // To send via the same connection again, it may not work as it may not be ready,
//!     // so we have to wait until the request_sender becomes ready.
//!     request_sender.ready().await?;
//!     let request = Request::builder()
//!         .header("Host", "example.com")
//!         .method("GET")
//!         .body(Body::from(""))?;
//!     let response = request_sender.send_request(request).await?;
//!     assert!(response.status() == StatusCode::OK);
//!     Ok(())
//! }
//!
//! # }
//! ```
use std::error::Error as StdError;
use std::fmt;
#[cfg(not(all(feature = "http1", feature = "http2")))]
use std::marker::PhantomData;

#[cfg(all(feature = "runtime", feature = "http2"))]
use std::time::Duration;
use bytes::Bytes;
use futures_util::future::{self, Either, FutureExt as _};
use httparse::ParserConfig;
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service::Service;
use tracing::{debug, trace};
use super::dispatch;
use crate::body::HttpBody;
#[cfg(not(all(feature = "http1", feature = "http2")))]
use crate::common::Never;
use crate::common::{
    exec::{BoxSendFuture, Exec},
    task, Future, Pin, Poll,
};
use crate::proto;
use crate::rt::Executor;

use crate::{Body, Request, Response};
#[cfg(feature = "http1")]
type Http1Dispatcher<T, B> = proto::dispatch::Dispatcher<
    proto::dispatch::Client<B>,
    B,
    T,
    proto::h1::ClientTransaction,
>;
#[cfg(not(feature = "http1"))]
type Http1Dispatcher<T, B> = (Never, PhantomData<(T, Pin<Box<B>>)>);
#[cfg(feature = "http2")]
type Http2ClientTask<B> = proto::h2::ClientTask<B>;
#[cfg(not(feature = "http2"))]
type Http2ClientTask<B> = (Never, PhantomData<Pin<Box<B>>>);
pin_project! {
    #[project = ProtoClientProj] enum ProtoClient < T, B > where B : HttpBody, { H1 {
    #[pin] h1 : Http1Dispatcher < T, B >, }, H2 { #[pin] h2 : Http2ClientTask < B >, }, }
}




pub(crate) async fn handshake<T>(
    io: T,
) -> crate::Result<(SendRequest<crate::Body>, Connection<T, crate::Body>)>
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    loop {}
}

pub struct SendRequest<B> {
    dispatch: dispatch::Sender<Request<B>, Response<Body>>,
}




#[must_use = "futures do nothing unless polled"]
pub struct Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Send + 'static,
    B: HttpBody + 'static,
{
    inner: Option<ProtoClient<T, B>>,
}



#[derive(Clone, Debug)]
pub struct Builder {
    pub(super) exec: Exec,
    h09_responses: bool,
    h1_parser_config: ParserConfig,
    h1_writev: Option<bool>,
    h1_title_case_headers: bool,
    h1_preserve_header_case: bool,
    #[cfg(feature = "ffi")]
    h1_preserve_header_order: bool,
    h1_read_buf_exact_size: Option<usize>,
    h1_max_buf_size: Option<usize>,
    #[cfg(feature = "ffi")]
    h1_headers_raw: bool,
    #[cfg(feature = "http2")]
    h2_builder: proto::h2::client::Config,
    version: Proto,
}
#[derive(Clone, Debug)]
enum Proto {
    #[cfg(feature = "http1")]
    Http1,
    #[cfg(feature = "http2")]
    Http2,
}



#[must_use = "futures do nothing unless polled"]
pub struct ResponseFuture {
    inner: ResponseFutureState,
}
enum ResponseFutureState {
    Waiting(dispatch::Promise<Response<Body>>),
    Error(Option<crate::Error>),
}




#[derive(Debug)]
pub struct Parts<T> {
    
    pub(crate) io: T,
    
    
    
    
    
    
    
    
    pub(crate) read_buf: Bytes,
    _inner: (),
}
#[must_use = "futures do nothing unless polled"]
#[cfg(feature = "http2")]
pub(super) struct Http2SendRequest<B> {
    dispatch: dispatch::UnboundedSender<Request<B>, Response<Body>>,
}
impl<B> SendRequest<B> {
    
    
    
    pub(crate) fn poll_ready(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<crate::Result<()>> {
        loop {}
    }
    pub(super) async fn when_ready(self) -> crate::Result<Self> {
        loop {}
    }
    pub(super) fn is_ready(&self) -> bool {
        loop {}
    }
    pub(super) fn is_closed(&self) -> bool {
        loop {}
    }
    #[cfg(feature = "http2")]
    pub(super) fn into_http2(self) -> Http2SendRequest<B> {
        loop {}
    }
}
impl<B> SendRequest<B>
where
    B: HttpBody + 'static,
{
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn send_request(&mut self, req: Request<B>) -> ResponseFuture {
        loop {}
    }
    pub(super) fn send_request_retryable(
        &mut self,
        req: Request<B>,
    ) -> impl Future<
        Output = Result<Response<Body>, (crate::Error, Option<Request<B>>)>,
    > + Unpin
    where
        B: Send,
    {
        match self.dispatch.try_send(req) {
            Ok(rx) => {
                Either::Left(
                    rx
                        .then(move |res| {
                            match res {
                                Ok(Ok(res)) => future::ok(res),
                                Ok(Err(err)) => future::err(err),
                                Err(_) => panic!("dispatch dropped without returning error"),
                            }
                        }),
                )
            }
            Err(req) => {
                debug!("connection was not ready");
                let err = crate::Error::new_canceled().with("connection was not ready");
                Either::Right(future::err((err, Some(req))))
            }
        }
    }
}
impl<B> Service<Request<B>> for SendRequest<B>
where
    B: HttpBody + 'static,
{
    type Response = Response<Body>;
    type Error = crate::Error;
    type Future = ResponseFuture;
    fn poll_ready(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn call(&mut self, req: Request<B>) -> Self::Future {
        loop {}
    }
}
impl<B> fmt::Debug for SendRequest<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
#[cfg(feature = "http2")]
impl<B> Http2SendRequest<B> {
    pub(super) fn is_ready(&self) -> bool {
        loop {}
    }
    pub(super) fn is_closed(&self) -> bool {
        loop {}
    }
}
#[cfg(feature = "http2")]
impl<B> Http2SendRequest<B>
where
    B: HttpBody + 'static,
{
    pub(super) fn send_request_retryable(
        &mut self,
        req: Request<B>,
    ) -> impl Future<Output = Result<Response<Body>, (crate::Error, Option<Request<B>>)>>
    where
        B: Send,
    {
        match self.dispatch.try_send(req) {
            Ok(rx) => {
                Either::Left(
                    rx
                        .then(move |res| {
                            match res {
                                Ok(Ok(res)) => future::ok(res),
                                Ok(Err(err)) => future::err(err),
                                Err(_) => panic!("dispatch dropped without returning error"),
                            }
                        }),
                )
            }
            Err(req) => {
                debug!("connection was not ready");
                let err = crate::Error::new_canceled().with("connection was not ready");
                Either::Right(future::err((err, Some(req))))
            }
        }
    }
}
#[cfg(feature = "http2")]
impl<B> fmt::Debug for Http2SendRequest<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
#[cfg(feature = "http2")]
impl<B> Clone for Http2SendRequest<B> {
    fn clone(&self) -> Self {
        loop {}
    }
}
impl<T, B> Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    B: HttpBody + Unpin + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    
    
    
    pub(crate) fn into_parts(self) -> Parts<T> {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn poll_without_shutdown(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<crate::Result<()>> {
        loop {}
    }
    
    
    pub(crate) fn without_shutdown(
        self,
    ) -> impl Future<Output = crate::Result<Parts<T>>> {
        let mut conn = Some(self);
        future::poll_fn(move |cx| -> Poll<crate::Result<Parts<T>>> {
            ready!(conn.as_mut().unwrap().poll_without_shutdown(cx))?;
            Poll::Ready(Ok(conn.take().unwrap().into_parts()))
        })
    }
    
    
    
    
    
    
    
    
    
    #[cfg(feature = "http2")]
    pub(crate) fn http2_is_extended_connect_protocol_enabled(&self) -> bool {
        loop {}
    }
}
impl<T, B> Future for Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Output = crate::Result<()>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
impl<T, B> fmt::Debug for Connection<T, B>
where
    T: AsyncRead + AsyncWrite + fmt::Debug + Send + 'static,
    B: HttpBody + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Builder {
    
    #[inline]
    pub(crate) fn new() -> Builder {
        loop {}
    }
    
    pub(crate) fn executor<E>(&mut self, exec: E) -> &mut Builder
    where
        E: Executor<BoxSendFuture> + Send + Sync + 'static,
    {
        loop {}
    }
    
    
    
    pub(crate) fn http09_responses(&mut self, enabled: bool) -> &mut Builder {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn http1_allow_spaces_after_header_name_in_responses(
        &mut self,
        enabled: bool,
    ) -> &mut Builder {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn http1_allow_obsolete_multiline_headers_in_responses(
        &mut self,
        enabled: bool,
    ) -> &mut Builder {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    pub(crate) fn http1_ignore_invalid_headers_in_responses(
        &mut self,
        enabled: bool,
    ) -> &mut Builder {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn http1_writev(&mut self, enabled: bool) -> &mut Builder {
        loop {}
    }
    
    
    
    
    
    
    pub(crate) fn http1_title_case_headers(&mut self, enabled: bool) -> &mut Builder {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn http1_preserve_header_case(&mut self, enabled: bool) -> &mut Builder {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    #[cfg(feature = "ffi")]
    pub(crate) fn http1_preserve_header_order(&mut self, enabled: bool) -> &mut Builder {
        loop {}
    }
    
    
    
    
    
    pub(crate) fn http1_read_buf_exact_size(
        &mut self,
        sz: Option<usize>,
    ) -> &mut Builder {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    #[cfg(feature = "http1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http1")))]
    pub(crate) fn http1_max_buf_size(&mut self, max: usize) -> &mut Self {
        loop {}
    }
    #[cfg(feature = "ffi")]
    pub(crate) fn http1_headers_raw(&mut self, enabled: bool) -> &mut Self {
        loop {}
    }
    
    
    
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub(crate) fn http2_only(&mut self, enabled: bool) -> &mut Builder {
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
    
    
    
    
    
    pub(crate) fn handshake<T, B>(
        &self,
        io: T,
    ) -> impl Future<Output = crate::Result<(SendRequest<B>, Connection<T, B>)>>
    where
        T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
        B: HttpBody + 'static,
        B::Data: Send,
        B::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        let opts = self.clone();
        async move {
            trace!("client handshake {:?}", opts.version);
            let (tx, rx) = dispatch::channel();
            let proto = match opts.version {
                #[cfg(feature = "http1")]
                Proto::Http1 => {
                    let mut conn = proto::Conn::new(io);
                    conn.set_h1_parser_config(opts.h1_parser_config);
                    if let Some(writev) = opts.h1_writev {
                        if writev {
                            conn.set_write_strategy_queue();
                        } else {
                            conn.set_write_strategy_flatten();
                        }
                    }
                    if opts.h1_title_case_headers {
                        conn.set_title_case_headers();
                    }
                    if opts.h1_preserve_header_case {
                        conn.set_preserve_header_case();
                    }
                    #[cfg(feature = "ffi")]
                    if opts.h1_preserve_header_order {
                        conn.set_preserve_header_order();
                    }
                    if opts.h09_responses {
                        conn.set_h09_responses();
                    }
                    #[cfg(feature = "ffi")] conn.set_raw_headers(opts.h1_headers_raw);
                    if let Some(sz) = opts.h1_read_buf_exact_size {
                        conn.set_read_buf_exact_size(sz);
                    }
                    if let Some(max) = opts.h1_max_buf_size {
                        conn.set_max_buf_size(max);
                    }
                    let cd = proto::h1::dispatch::Client::new(rx);
                    let dispatch = proto::h1::Dispatcher::new(cd, conn);
                    ProtoClient::H1 { h1: dispatch }
                }
                #[cfg(feature = "http2")]
                Proto::Http2 => {
                    let h2 = proto::h2::client::handshake(
                            io,
                            rx,
                            &opts.h2_builder,
                            opts.exec.clone(),
                        )
                        .await?;
                    ProtoClient::H2 { h2 }
                }
            };
            Ok((SendRequest { dispatch: tx }, Connection { inner: Some(proto) }))
        }
    }
}
impl Future for ResponseFuture {
    type Output = crate::Result<Response<Body>>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
impl fmt::Debug for ResponseFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<T, B> Future for ProtoClient<T, B>
where
    T: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Output = crate::Result<proto::Dispatched>;
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
trait AssertSend: Send {}
trait AssertSendSync: Send + Sync {}
#[doc(hidden)]
impl<B: Send> AssertSendSync for SendRequest<B> {}
#[doc(hidden)]
impl<T: Send, B: Send> AssertSend for Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Send + 'static,
    B: HttpBody + 'static,
    B::Data: Send,
{}
#[doc(hidden)]
impl<T: Send + Sync, B: Send + Sync> AssertSendSync for Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Send + 'static,
    B: HttpBody + 'static,
    B::Data: Send + Sync + 'static,
{}
#[doc(hidden)]
impl AssertSendSync for Builder {}
#[doc(hidden)]
impl AssertSend for ResponseFuture {}
