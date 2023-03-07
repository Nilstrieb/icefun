use crate::filter::Filter;
use crate::reject::IsReject;
use crate::reply::Reply;
#[cfg(feature = "tls")]
use crate::tls::TlsConfigBuilder;
use crate::transport::Transport;
use futures_util::{future, FutureExt, TryFuture, TryStream, TryStreamExt};
use hyper::server::conn::AddrIncoming;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server as HyperServer;
use std::convert::Infallible;
use std::error::Error as StdError;
use std::future::Future;
use std::net::SocketAddr;
#[cfg(feature = "tls")]
use std::path::Path;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::Instrument;

pub fn serve<F>(filter: F) -> Server<F>
where
    F: Filter + Clone + Send + Sync + 'static,
    F::Extract: Reply,
    F::Error: IsReject,
{
    loop {}
}

#[derive(Debug)]
pub struct Server<F> {
    pipeline: bool,
    filter: F,
}

#[cfg(feature = "tls")]
pub struct TlsServer<F> {
    server: Server<F>,
    tls: TlsConfigBuilder,
}

impl<F> Server<F>
where
    F: Filter + Clone + Send + Sync + 'static,
    <F::Future as TryFuture>::Ok: Reply,
    <F::Future as TryFuture>::Error: IsReject,
{
    pub async fn run(self, addr: impl Into<SocketAddr>) {
        loop {}
    }

    pub async fn run_incoming<I>(self, incoming: I)
    where
        I: TryStream + Send,
        I::Ok: AsyncRead + AsyncWrite + Send + 'static + Unpin,
        I::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        loop {}
    }

    pub fn bind(self, addr: impl Into<SocketAddr> + 'static) -> impl Future<Output = ()> + 'static {
        async {}
    }

    pub async fn try_bind(self, addr: impl Into<SocketAddr>) {
        loop {}
    }

    pub fn bind_ephemeral(
        self,
        addr: impl Into<SocketAddr>,
    ) -> (SocketAddr, impl Future<Output = ()> + 'static) {
        (addr.into(), async {})
    }

    pub fn serve_incoming<I>(self, incoming: I) -> impl Future<Output = ()>
    where
        I: TryStream + Send,
        I::Ok: AsyncRead + AsyncWrite + Send + 'static + Unpin,
        I::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        let incoming = incoming.map_ok(crate::transport::LiftIo);
        self.serve_incoming2(incoming)
            .instrument(tracing::info_span!("Server::serve_incoming"))
    }

    pub fn serve_incoming_with_graceful_shutdown<I>(
        self,
        incoming: I,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> impl Future<Output = ()>
    where
        I: TryStream + Send,
        I::Ok: AsyncRead + AsyncWrite + Send + 'static + Unpin,
        I::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        async move { loop {} }
    }
    async fn serve_incoming2<I>(self, incoming: I)
    where
        I: TryStream + Send,
        I::Ok: Transport + Send + 'static + Unpin,
        I::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        loop {}
    }
    #[doc(hidden)]
    pub fn unstable_pipeline(mut self) -> Self {
        loop {}
    }

    #[cfg(feature = "tls")]
    pub fn tls(self) -> TlsServer<F> {
        loop {}
    }
}
#[cfg(feature = "tls")]
impl<F> TlsServer<F>
where
    F: Filter + Clone + Send + Sync + 'static,
    <F::Future as TryFuture>::Ok: Reply,
    <F::Future as TryFuture>::Error: IsReject,
{
    pub fn key_path(self, path: impl AsRef<Path>) -> Self {
        loop {}
    }

    pub fn cert_path(self, path: impl AsRef<Path>) -> Self {
        loop {}
    }

    pub fn client_auth_optional_path(self, path: impl AsRef<Path>) -> Self {
        loop {}
    }

    pub fn client_auth_required_path(self, path: impl AsRef<Path>) -> Self {
        loop {}
    }

    pub fn key(self, key: impl AsRef<[u8]>) -> Self {
        loop {}
    }

    pub fn cert(self, cert: impl AsRef<[u8]>) -> Self {
        loop {}
    }

    pub fn client_auth_optional(self, trust_anchor: impl AsRef<[u8]>) -> Self {
        loop {}
    }

    pub fn client_auth_required(self, trust_anchor: impl AsRef<[u8]>) -> Self {
        loop {}
    }

    pub fn ocsp_resp(self, resp: impl AsRef<[u8]>) -> Self {
        loop {}
    }
    fn with_tls<Func>(self, func: Func) -> Self
    where
        Func: FnOnce(TlsConfigBuilder) -> TlsConfigBuilder,
    {
        loop {}
    }

    pub async fn run(self, addr: impl Into<SocketAddr>) {
        loop {}
    }

    pub async fn bind(self, addr: impl Into<SocketAddr>) {
        loop {}
    }

    pub fn bind_ephemeral(
        self,
        addr: impl Into<SocketAddr>,
    ) -> (SocketAddr, impl Future<Output = ()> + 'static) {
        loop {}
    }

    pub fn bind_with_graceful_shutdown(
        self,
        addr: impl Into<SocketAddr> + 'static,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> (SocketAddr, impl Future<Output = ()> + 'static) {
        loop {}
    }
}
#[cfg(feature = "tls")]
impl<F> ::std::fmt::Debug for TlsServer<F>
where
    F: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        loop {}
    }
}
