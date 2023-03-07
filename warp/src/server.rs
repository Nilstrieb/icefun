#[cfg(feature = "tls")]
use crate::tls::TlsConfigBuilder;
use std::convert::Infallible;
use std::error::Error as StdError;
use std::future::Future;
use std::net::SocketAddr;
#[cfg(feature = "tls")]
use std::path::Path;
use futures_util::{future, FutureExt, TryFuture, TryStream, TryStreamExt};
use hyper::server::conn::AddrIncoming;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server as HyperServer;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::Instrument;
use crate::filter::Filter;
use crate::reject::IsReject;
use crate::reply::Reply;
use crate::transport::Transport;

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
macro_rules! into_service {
    ($into:expr) => {
        { let inner = crate ::service($into); make_service_fn(move | transport | { let
        inner = inner.clone(); let remote_addr = Transport::remote_addr(transport);
        future::ok::< _, Infallible > (service_fn(move | req | { inner
        .call_with_addr(req, remote_addr) })) }) }
    };
}
macro_rules! addr_incoming {
    ($addr:expr) => {
        { let mut incoming = AddrIncoming::bind($addr) ?; incoming.set_nodelay(true); let
        addr = incoming.local_addr(); (addr, incoming) }
    };
}
macro_rules! bind_inner {
    ($this:ident, $addr:expr) => {
        { let service = into_service!($this .filter); let (addr, incoming) =
        addr_incoming!($addr); let srv = HyperServer::builder(incoming)
        .http1_pipeline_flush($this .pipeline).serve(service); Ok::< _, hyper::Error >
        ((addr, srv)) }
    };
    (tls : $this:ident, $addr:expr) => {
        { let service = into_service!($this .server.filter); let (addr, incoming) =
        addr_incoming!($addr); let tls = $this .tls.build() ?; let srv =
        HyperServer::builder(crate ::tls::TlsAcceptor::new(tls, incoming))
        .http1_pipeline_flush($this .server.pipeline).serve(service); Ok::< _, Box < dyn
        std::error::Error + Send + Sync >> ((addr, srv)) }
    };
}
macro_rules! bind {
    ($this:ident, $addr:expr) => {
        { let addr = $addr .into(); (| addr | bind_inner!($this, addr)) (& addr)
        .unwrap_or_else(| e | { panic!("error binding to {}: {}", addr, e); }) }
    };
    (tls : $this:ident, $addr:expr) => {
        { let addr = $addr .into(); (| addr | bind_inner!(tls : $this, addr)) (& addr)
        .unwrap_or_else(| e | { panic!("error binding to {}: {}", addr, e); }) }
    };
}
macro_rules! try_bind {
    ($this:ident, $addr:expr) => {
        { (| addr | bind_inner!($this, addr)) ($addr) }
    };
    (tls : $this:ident, $addr:expr) => {
        { (| addr | bind_inner!(tls : $this, addr)) ($addr) }
    };
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
    
    
    
    
    
    
    pub fn bind(
        self,
        addr: impl Into<SocketAddr> + 'static,
    ) -> impl Future<Output = ()> + 'static {
        let (_, fut) = self.bind_ephemeral(addr);
        fut
    }
    
    
    
    
    
    pub async fn try_bind(self, addr: impl Into<SocketAddr>) {
        loop {}
    }
    
    
    
    
    
    
    
    
    pub fn bind_ephemeral(
        self,
        addr: impl Into<SocketAddr>,
    ) -> (SocketAddr, impl Future<Output = ()> + 'static) {
        let (addr, srv) = bind!(self, addr);
        let srv = srv
            .map(|result| {
                if let Err(err) = result {
                    tracing::error!("server error: {}", err)
                }
            });
        (addr, srv)
    }
    
    
    
    
    
    
    
    pub fn try_bind_ephemeral(
        self,
        addr: impl Into<SocketAddr>,
    ) -> Result<(SocketAddr, impl Future<Output = ()> + 'static), crate::Error> {
        let addr = addr.into();
        let (addr, srv) = try_bind!(self, & addr).map_err(crate::Error::new)?;
        let srv = srv
            .map(|result| {
                if let Err(err) = result {
                    tracing::error!("server error: {}", err)
                }
            });
        Ok((addr, srv))
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
        let incoming = incoming.map_ok(crate::transport::LiftIo);
        let service = into_service!(self.filter);
        let pipeline = self.pipeline;
        async move {
            let srv = HyperServer::builder(
                    hyper::server::accept::from_stream(incoming.into_stream()),
                )
                .http1_pipeline_flush(pipeline)
                .serve(service)
                .await;
            if let Err(err) = srv {
                tracing::error!("server error: {}", err);
            }
        }
            .instrument(
                tracing::info_span!("Server::serve_incoming_with_graceful_shutdown"),
            )
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
