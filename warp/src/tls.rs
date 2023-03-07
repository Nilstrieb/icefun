use std::fmt;
use std::fs::File;
use std::future::Future;
use std::io::{self, BufReader, Cursor, Read};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use futures_util::ready;
use hyper::server::accept::Accept;
use hyper::server::conn::{AddrIncoming, AddrStream};
use crate::transport::Transport;
use tokio_rustls::rustls::{
    server::{
        AllowAnyAnonymousOrAuthenticatedClient, AllowAnyAuthenticatedClient, NoClientAuth,
    },
    Certificate, Error as TlsError, PrivateKey, RootCertStore, ServerConfig,
};

#[derive(Debug)]
pub(crate) enum TlsConfigError {
    Io(io::Error),
    
    CertParseError,
    
    Pkcs8ParseError,
    
    RsaParseError,
    
    EmptyKey,
    
    InvalidKey(TlsError),
}
impl fmt::Display for TlsConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl std::error::Error for TlsConfigError {}

pub(crate) enum TlsClientAuth {
    
    Off,
    
    Optional(Box<dyn Read + Send + Sync>),
    
    Required(Box<dyn Read + Send + Sync>),
}

pub(crate) struct TlsConfigBuilder {
    cert: Box<dyn Read + Send + Sync>,
    key: Box<dyn Read + Send + Sync>,
    client_auth: TlsClientAuth,
    ocsp_resp: Vec<u8>,
}
impl fmt::Debug for TlsConfigBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl TlsConfigBuilder {
    
    pub(crate) fn new() -> TlsConfigBuilder {
        loop {}
    }
    
    pub(crate) fn key_path(mut self, path: impl AsRef<Path>) -> Self {
        loop {}
    }
    
    pub(crate) fn key(mut self, key: &[u8]) -> Self {
        loop {}
    }
    
    pub(crate) fn cert_path(mut self, path: impl AsRef<Path>) -> Self {
        loop {}
    }
    
    pub(crate) fn cert(mut self, cert: &[u8]) -> Self {
        loop {}
    }
    
    
    
    
    pub(crate) fn client_auth_optional_path(mut self, path: impl AsRef<Path>) -> Self {
        loop {}
    }
    
    
    
    
    pub(crate) fn client_auth_optional(mut self, trust_anchor: &[u8]) -> Self {
        loop {}
    }
    
    
    
    
    pub(crate) fn client_auth_required_path(mut self, path: impl AsRef<Path>) -> Self {
        loop {}
    }
    
    
    
    
    pub(crate) fn client_auth_required(mut self, trust_anchor: &[u8]) -> Self {
        loop {}
    }
    
    pub(crate) fn ocsp_resp(mut self, ocsp_resp: &[u8]) -> Self {
        loop {}
    }
    pub(crate) fn build(mut self) -> Result<ServerConfig, TlsConfigError> {
        loop {}
    }
}
struct LazyFile {
    path: PathBuf,
    file: Option<File>,
}
impl LazyFile {
    fn lazy_read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {}
    }
}
impl Read for LazyFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {}
    }
}
impl Transport for TlsStream {
    fn remote_addr(&self) -> Option<SocketAddr> {
        loop {}
    }
}
enum State {
    Handshaking(tokio_rustls::Accept<AddrStream>),
    Streaming(tokio_rustls::server::TlsStream<AddrStream>),
}
pub(crate) struct TlsStream {
    state: State,
    remote_addr: SocketAddr,
}
impl TlsStream {
    fn new(stream: AddrStream, config: Arc<ServerConfig>) -> TlsStream {
        loop {}
    }
}
impl AsyncRead for TlsStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        loop {}
    }
}
impl AsyncWrite for TlsStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        loop {}
    }
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        loop {}
    }
    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        loop {}
    }
}
pub(crate) struct TlsAcceptor {
    config: Arc<ServerConfig>,
    incoming: AddrIncoming,
}
impl TlsAcceptor {
    pub(crate) fn new(config: ServerConfig, incoming: AddrIncoming) -> TlsAcceptor {
        loop {}
    }
}
impl Accept for TlsAcceptor {
    type Conn = TlsStream;
    type Error = io::Error;
    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn file_cert_key() {
        loop {}
    }
    #[test]
    fn bytes_cert_key() {
        loop {}
    }
}
