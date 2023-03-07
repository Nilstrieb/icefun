//! Lower-level Server connection API.
//!
//! The types in this module are to provide a lower-level API based around a
//! single connection. Accepting a connection and binding it with a service
//! are not handled at this level. This module provides the building blocks to
//! customize those things externally.
//!
//! If you don't have need to manage connections yourself, consider using the
//! higher-level [Server](super) API.
//!
//! ## Example
//! A simple example that uses the `Http` struct to talk HTTP over a Tokio TCP stream
//! ```no_run
//! # #[cfg(all(feature = "http1", feature = "runtime"))]
//! # mod rt {
//! use http::{Request, Response, StatusCode};
//! use hyper::{server::conn::Http, service::service_fn, Body};
//! use std::{net::SocketAddr, convert::Infallible};
//! use tokio::net::TcpListener;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();
//!
//!     let mut tcp_listener = TcpListener::bind(addr).await?;
//!     loop {
//!         let (tcp_stream, _) = tcp_listener.accept().await?;
//!         tokio::task::spawn(async move {
//!             if let Err(http_err) = Http::new()
//!                     .http1_only(true)
//!                     .http1_keep_alive(true)
//!                     .serve_connection(tcp_stream, service_fn(hello))
//!                     .await {
//!                 eprintln!("Error while serving HTTP connection: {}", http_err);
//!             }
//!         });
//!     }
//! }
//!
//! async fn hello(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//!    Ok(Response::new(Body::from("Hello World!")))
//! }
//! # }
//! ```
#[cfg(
    all(
        any(feature = "http1", feature = "http2"),
        not(all(feature = "http1", feature = "http2"))
    )
)]
use std::marker::PhantomData;
#[cfg(all(any(feature = "http1", feature = "http2"), feature = "runtime"))]
use std::time::Duration;
#[cfg(feature = "http2")]
use crate::common::io::Rewind;
cfg_feature! {
    #![any(feature = "http1", feature = "http2")] use std::error::Error as StdError; use
    std::fmt; use bytes::Bytes; use pin_project_lite::pin_project; use tokio::io:: {
    AsyncRead, AsyncWrite }; use tracing::trace; pub use super::server::Connecting; use
    crate ::body:: { Body, HttpBody }; use crate ::common:: { task, Future, Pin, Poll,
    Unpin }; #[cfg(not(all(feature = "http1", feature = "http2")))] use crate
    ::common::Never; use crate ::common::exec:: { ConnStreamExec, Exec }; use crate
    ::proto; use crate ::service::HttpService; pub (super) use
    self::upgrades::UpgradeableConnection;
}
#[cfg(feature = "tcp")]
pub use super::tcp::{AddrIncoming, AddrStream};
/// A lower-level configuration of the HTTP protocol.
///
/// This structure is used to configure options for an HTTP server connection.
///
/// If you don't have need to manage connections yourself, consider using the
/// higher-level [Server](super) API.
#[derive(Clone, Debug)]
#[cfg(any(feature = "http1", feature = "http2"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
pub(crate) struct Http<E = Exec> {
    pub(crate) exec: E,
    h1_half_close: bool,
    h1_keep_alive: bool,
    h1_title_case_headers: bool,
    h1_preserve_header_case: bool,
    #[cfg(all(feature = "http1", feature = "runtime"))]
    h1_header_read_timeout: Option<Duration>,
    h1_writev: Option<bool>,
    #[cfg(feature = "http2")]
    h2_builder: proto::h2::server::Config,
    mode: ConnectionMode,
    max_buf_size: Option<usize>,
    pipeline_flush: bool,
}
/// The internal mode of HTTP protocol which indicates the behavior when a parse error occurs.
#[cfg(any(feature = "http1", feature = "http2"))]
#[derive(Clone, Debug, PartialEq)]
enum ConnectionMode {
    /// Always use HTTP/1 and do not upgrade when a parse error occurs.
    #[cfg(feature = "http1")]
    H1Only,
    /// Always use HTTP/2.
    #[cfg(feature = "http2")]
    H2Only,
    /// Use HTTP/1 and try to upgrade to h2 when a parse error occurs.
    #[cfg(all(feature = "http1", feature = "http2"))]
    Fallback,
}
#[cfg(any(feature = "http1", feature = "http2"))]
pin_project! {
    #[doc = " A future binding a connection with a Service."] #[doc = ""] #[doc =
    " Polling this future will drive HTTP forward."] #[must_use =
    "futures do nothing unless polled"] #[cfg_attr(docsrs, doc(cfg(any(feature = "http1",
    feature = "http2"))))] pub struct Connection < T, S, E = Exec > where S : HttpService
    < Body >, { pub (super) conn : Option < ProtoServer < T, S::ResBody, S, E >>,
    fallback : Fallback < E >, }
}
#[cfg(feature = "http1")]
type Http1Dispatcher<T, B, S> = proto::h1::Dispatcher<
    proto::h1::dispatch::Server<S, Body>,
    B,
    T,
    proto::ServerTransaction,
>;
#[cfg(all(not(feature = "http1"), feature = "http2"))]
type Http1Dispatcher<T, B, S> = (Never, PhantomData<(T, Box<Pin<B>>, Box<Pin<S>>)>);
#[cfg(feature = "http2")]
type Http2Server<T, B, S, E> = proto::h2::Server<Rewind<T>, S, B, E>;
#[cfg(all(not(feature = "http2"), feature = "http1"))]
type Http2Server<T, B, S, E> = (
    Never,
    PhantomData<(T, Box<Pin<S>>, Box<Pin<B>>, Box<Pin<E>>)>,
);
#[cfg(any(feature = "http1", feature = "http2"))]
pin_project! {
    #[project = ProtoServerProj] pub (super) enum ProtoServer < T, B, S, E = Exec > where
    S : HttpService < Body >, B : HttpBody, { H1 { #[pin] h1 : Http1Dispatcher < T, B, S
    >, }, H2 { #[pin] h2 : Http2Server < T, B, S, E >, }, }
}
#[cfg(all(feature = "http1", feature = "http2"))]
#[derive(Clone, Debug)]
enum Fallback<E> {
    ToHttp2(proto::h2::server::Config, E),
    Http1Only,
}
#[cfg(
    all(
        any(feature = "http1", feature = "http2"),
        not(all(feature = "http1", feature = "http2"))
    )
)]
type Fallback<E> = PhantomData<E>;
#[cfg(any(feature = "http1", feature = "http2"))]
impl Http {}
#[cfg(any(feature = "http1", feature = "http2"))]
impl Default for ConnectionMode {
    #[cfg(all(feature = "http1", feature = "http2"))]
    fn default() -> ConnectionMode {
        loop {}
    }
    #[cfg(all(feature = "http1", not(feature = "http2")))]
    fn default() -> ConnectionMode {
        loop {}
    }
    #[cfg(all(not(feature = "http1"), feature = "http2"))]
    fn default() -> ConnectionMode {
        loop {}
    }
}
#[cfg(any(feature = "http1", feature = "http2"))]
mod upgrades {
    use super::*;
    #[must_use = "futures do nothing unless polled"]
    #[allow(missing_debug_implementations)]
    pub struct UpgradeableConnection<T, S, E>
    where
        S: HttpService<Body>,
    {
        pub(super) inner: Connection<T, S, E>,
    }
    impl<I, B, S, E> UpgradeableConnection<I, S, E>
    where
        S: HttpService<Body, ResBody = B>,
        S::Error: Into<Box<dyn StdError + Send + Sync>>,
        I: AsyncRead + AsyncWrite + Unpin,
        B: HttpBody + 'static,
        B::Error: Into<Box<dyn StdError + Send + Sync>>,
        E: ConnStreamExec<S::Future, B>,
    {}
    impl<I, B, S, E> Future for UpgradeableConnection<I, S, E>
    where
        S: HttpService<Body, ResBody = B>,
        S::Error: Into<Box<dyn StdError + Send + Sync>>,
        I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
        B: HttpBody + 'static,
        B::Error: Into<Box<dyn StdError + Send + Sync>>,
        E: ConnStreamExec<S::Future, B>,
    {
        type Output = crate::Result<()>;
        fn poll(
            mut self: Pin<&mut Self>,
            cx: &mut task::Context<'_>,
        ) -> Poll<Self::Output> {
            loop {}
        }
    }
}
