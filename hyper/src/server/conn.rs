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
#[cfg(feature = "http2")]
use crate::common::io::Rewind;
#[cfg(all(
    any(feature = "http1", feature = "http2"),
    not(all(feature = "http1", feature = "http2"))
))]
use std::marker::PhantomData;
#[cfg(all(any(feature = "http1", feature = "http2"), feature = "runtime"))]
use std::time::Duration;
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

#[derive(Clone, Debug)]
#[cfg(any(feature = "http1", feature = "http2"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
pub(crate) struct Http<E = Exec> {
    pub(crate) exec: E,
}

#[cfg(any(feature = "http1", feature = "http2"))]
#[derive(Clone, Debug, PartialEq)]
enum ConnectionMode {
    #[cfg(feature = "http1")]
    H1Only,

    #[cfg(feature = "http2")]
    H2Only,

    #[cfg(all(feature = "http1", feature = "http2"))]
    Fallback,
}
#[cfg(any(feature = "http1", feature = "http2"))]
pin_project! {

     pub struct Connection < T, S, E = Exec > { pub (super) conn : Option < ProtoServer < T, S, S, E >>,
    fallback : Fallback < E >, }
}
#[cfg(feature = "http1")]
type Http1Dispatcher<T, B, S> =
    proto::h1::Dispatcher<proto::h1::dispatch::Server<S, Body>, B, T, proto::ServerTransaction>;
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
    #[project = ProtoServerProj] pub (super) enum ProtoServer < T, B, S, E = Exec > {

         H1 { #[pin] h1 : (T, B, S), }, H2 { #[pin] h2 : (T, B, S ,E), },

}
}
#[cfg(all(feature = "http1", feature = "http2"))]
#[derive(Clone, Debug)]
enum Fallback<E> {
    ToHttp2(proto::h2::server::Config, E),
    Http1Only,
}
#[cfg(all(
    any(feature = "http1", feature = "http2"),
    not(all(feature = "http1", feature = "http2"))
))]
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
    pub struct UpgradeableConnection<T, S, E> {
        pub(super) inner: (T, S, E),
    }
    impl<I, B, S, E> UpgradeableConnection<I, S, E>
    where
        S: HttpService<Body, ResBody = B>,
        S::Error: Into<Box<dyn StdError + Send + Sync>>,
        I: AsyncRead + AsyncWrite + Unpin,
        B: HttpBody + 'static,
        B::Error: Into<Box<dyn StdError + Send + Sync>>,
        E: ConnStreamExec<S::Future, B>,
    {
    }
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
        fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
            loop {}
        }
    }
}
