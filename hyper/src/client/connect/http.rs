use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use std::io;
use std::marker::PhantomData;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{self, Poll};
use std::time::Duration;
use futures_util::future::Either;
use http::uri::{Scheme, Uri};
use pin_project_lite::pin_project;
use tokio::net::{TcpSocket, TcpStream};
use tokio::time::Sleep;
use tracing::{debug, trace, warn};
use super::dns::{self, resolve, GaiResolver, Resolve};
use super::{Connected, Connection};
/// A connector for the `http` scheme.
///
/// Performs DNS resolution in a thread pool, and then connects over TCP.
///
/// # Note
///
/// Sets the [`HttpInfo`](HttpInfo) value on responses, which includes
/// transport information such as the remote socket address used.
#[cfg_attr(docsrs, doc(cfg(feature = "tcp")))]
#[derive(Clone)]
pub struct HttpConnector<R = GaiResolver> {
    config: Arc<Config>,
    resolver: R,
}
/// Extra information about the transport when an HttpConnector is used.
///
/// # Example
///
/// ```
/// # async fn doc() -> hyper::Result<()> {
/// use hyper::Uri;
/// use hyper::client::{Client, connect::HttpInfo};
///
/// let client = Client::new();
/// let uri = Uri::from_static("http://example.com");
///
/// let res = client.get(uri).await?;
/// res
///     .extensions()
///     .get::<HttpInfo>()
///     .map(|info| {
///         println!("remote addr = {}", info.remote_addr());
///     });
/// # Ok(())
/// # }
/// ```
///
/// # Note
///
/// If a different connector is used besides [`HttpConnector`](HttpConnector),
/// this value will not exist in the extensions. Consult that specific
/// connector to see what "extra" information it might provide to responses.
#[derive(Clone, Debug)]
pub struct HttpInfo {
    remote_addr: SocketAddr,
    local_addr: SocketAddr,
}
#[derive(Clone)]
struct Config {
    connect_timeout: Option<Duration>,
    enforce_http: bool,
    happy_eyeballs_timeout: Option<Duration>,
    keep_alive_timeout: Option<Duration>,
    local_address_ipv4: Option<Ipv4Addr>,
    local_address_ipv6: Option<Ipv6Addr>,
    nodelay: bool,
    reuse_address: bool,
    send_buffer_size: Option<usize>,
    recv_buffer_size: Option<usize>,
}
impl HttpConnector {
    /// Construct a new HttpConnector.
    pub(crate) fn new() -> HttpConnector {
        loop {}
    }
}
impl<R> HttpConnector<R> {
    /// Construct a new HttpConnector.
    ///
    /// Takes a [`Resolver`](crate::client::connect::dns#resolvers-are-services) to handle DNS lookups.
    pub(crate) fn new_with_resolver(resolver: R) -> HttpConnector<R> {
        loop {}
    }
    /// Option to enforce all `Uri`s have the `http` scheme.
    ///
    /// Enabled by default.
    #[inline]
    pub(crate) fn enforce_http(&mut self, is_enforced: bool) {
        loop {}
    }
    /// Set that all sockets have `SO_KEEPALIVE` set with the supplied duration.
    ///
    /// If `None`, the option will not be set.
    ///
    /// Default is `None`.
    #[inline]
    pub(crate) fn set_keepalive(&mut self, dur: Option<Duration>) {
        loop {}
    }
    /// Set that all sockets have `SO_NODELAY` set to the supplied value `nodelay`.
    ///
    /// Default is `false`.
    #[inline]
    pub(crate) fn set_nodelay(&mut self, nodelay: bool) {
        loop {}
    }
    /// Sets the value of the SO_SNDBUF option on the socket.
    #[inline]
    pub(crate) fn set_send_buffer_size(&mut self, size: Option<usize>) {
        loop {}
    }
    /// Sets the value of the SO_RCVBUF option on the socket.
    #[inline]
    pub(crate) fn set_recv_buffer_size(&mut self, size: Option<usize>) {
        loop {}
    }
    /// Set that all sockets are bound to the configured address before connection.
    ///
    /// If `None`, the sockets will not be bound.
    ///
    /// Default is `None`.
    #[inline]
    pub(crate) fn set_local_address(&mut self, addr: Option<IpAddr>) {
        loop {}
    }
    /// Set that all sockets are bound to the configured IPv4 or IPv6 address (depending on host's
    /// preferences) before connection.
    #[inline]
    pub(crate) fn set_local_addresses(
        &mut self,
        addr_ipv4: Ipv4Addr,
        addr_ipv6: Ipv6Addr,
    ) {
        loop {}
    }
    /// Set the connect timeout.
    ///
    /// If a domain resolves to multiple IP addresses, the timeout will be
    /// evenly divided across them.
    ///
    /// Default is `None`.
    #[inline]
    pub(crate) fn set_connect_timeout(&mut self, dur: Option<Duration>) {
        loop {}
    }
    /// Set timeout for [RFC 6555 (Happy Eyeballs)][RFC 6555] algorithm.
    ///
    /// If hostname resolves to both IPv4 and IPv6 addresses and connection
    /// cannot be established using preferred address family before timeout
    /// elapses, then connector will in parallel attempt connection using other
    /// address family.
    ///
    /// If `None`, parallel connection attempts are disabled.
    ///
    /// Default is 300 milliseconds.
    ///
    /// [RFC 6555]: https://tools.ietf.org/html/rfc6555
    #[inline]
    pub(crate) fn set_happy_eyeballs_timeout(&mut self, dur: Option<Duration>) {
        loop {}
    }
    /// Set that all socket have `SO_REUSEADDR` set to the supplied value `reuse_address`.
    ///
    /// Default is `false`.
    #[inline]
    pub(crate) fn set_reuse_address(&mut self, reuse_address: bool) -> &mut Self {
        loop {}
    }
    fn config_mut(&mut self) -> &mut Config {
        loop {}
    }
}
static INVALID_NOT_HTTP: &str = "invalid URL, scheme is not http";
static INVALID_MISSING_SCHEME: &str = "invalid URL, scheme is missing";
static INVALID_MISSING_HOST: &str = "invalid URL, host is missing";
impl<R: fmt::Debug> fmt::Debug for HttpConnector<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<R> tower_service::Service<Uri> for HttpConnector<R>
where
    R: Resolve + Clone + Send + Sync + 'static,
    R::Future: Send,
{
    type Response = TcpStream;
    type Error = ConnectError;
    type Future = HttpConnecting<R>;
    fn poll_ready(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn call(&mut self, dst: Uri) -> Self::Future {
        loop {}
    }
}
fn get_host_port<'u>(
    config: &Config,
    dst: &'u Uri,
) -> Result<(&'u str, u16), ConnectError> {
    loop {}
}
impl<R> HttpConnector<R>
where
    R: Resolve,
{
    async fn call_async(&mut self, dst: Uri) -> Result<TcpStream, ConnectError> {
        loop {}
    }
}
impl Connection for TcpStream {
    fn connected(&self) -> Connected {
        loop {}
    }
}
impl HttpInfo {
    /// Get the remote address of the transport used.
    pub(crate) fn remote_addr(&self) -> SocketAddr {
        loop {}
    }
    /// Get the local address of the transport used.
    pub(crate) fn local_addr(&self) -> SocketAddr {
        loop {}
    }
}
pin_project! {
    #[must_use = "futures do nothing unless polled"]
    #[allow(missing_debug_implementations)] pub struct HttpConnecting < R > { #[pin] fut
    : BoxConnecting, _marker : PhantomData < R >, }
}
type ConnectResult = Result<TcpStream, ConnectError>;
type BoxConnecting = Pin<Box<dyn Future<Output = ConnectResult> + Send>>;
impl<R: Resolve> Future for HttpConnecting<R> {
    type Output = ConnectResult;
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
pub struct ConnectError {
    msg: Box<str>,
    cause: Option<Box<dyn StdError + Send + Sync>>,
}
impl ConnectError {
    fn new<S, E>(msg: S, cause: E) -> ConnectError
    where
        S: Into<Box<str>>,
        E: Into<Box<dyn StdError + Send + Sync>>,
    {
        loop {}
    }
    fn dns<E>(cause: E) -> ConnectError
    where
        E: Into<Box<dyn StdError + Send + Sync>>,
    {
        loop {}
    }
    fn m<S, E>(msg: S) -> impl FnOnce(E) -> ConnectError
    where
        S: Into<Box<str>>,
        E: Into<Box<dyn StdError + Send + Sync>>,
    {
        move |cause| ConnectError::new(msg, cause)
    }
}
impl fmt::Debug for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for ConnectError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        loop {}
    }
}
struct ConnectingTcp<'a> {
    preferred: ConnectingTcpRemote,
    fallback: Option<ConnectingTcpFallback>,
    config: &'a Config,
}
impl<'a> ConnectingTcp<'a> {
    fn new(remote_addrs: dns::SocketAddrs, config: &'a Config) -> Self {
        loop {}
    }
}
struct ConnectingTcpFallback {
    delay: Sleep,
    remote: ConnectingTcpRemote,
}
struct ConnectingTcpRemote {
    addrs: dns::SocketAddrs,
    connect_timeout: Option<Duration>,
}
impl ConnectingTcpRemote {
    fn new(addrs: dns::SocketAddrs, connect_timeout: Option<Duration>) -> Self {
        loop {}
    }
}
impl ConnectingTcpRemote {
    async fn connect(&mut self, config: &Config) -> Result<TcpStream, ConnectError> {
        loop {}
    }
}
fn bind_local_address(
    socket: &socket2::Socket,
    dst_addr: &SocketAddr,
    local_addr_ipv4: &Option<Ipv4Addr>,
    local_addr_ipv6: &Option<Ipv6Addr>,
) -> io::Result<()> {
    loop {}
}
fn connect(
    addr: &SocketAddr,
    config: &Config,
    connect_timeout: Option<Duration>,
) -> Result<impl Future<Output = Result<TcpStream, ConnectError>>, ConnectError> {
    use socket2::{Domain, Protocol, Socket, TcpKeepalive, Type};
    use std::convert::TryInto;
    let domain = Domain::for_address(*addr);
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))
        .map_err(ConnectError::m("tcp open error"))?;
    socket.set_nonblocking(true).map_err(ConnectError::m("tcp set_nonblocking error"))?;
    if let Some(dur) = config.keep_alive_timeout {
        let conf = TcpKeepalive::new().with_time(dur);
        if let Err(e) = socket.set_tcp_keepalive(&conf) {
            warn!("tcp set_keepalive error: {}", e);
        }
    }
    bind_local_address(
            &socket,
            addr,
            &config.local_address_ipv4,
            &config.local_address_ipv6,
        )
        .map_err(ConnectError::m("tcp bind local error"))?;
    #[cfg(unix)]
    let socket = unsafe {
        use std::os::unix::io::{FromRawFd, IntoRawFd};
        TcpSocket::from_raw_fd(socket.into_raw_fd())
    };
    #[cfg(windows)]
    let socket = unsafe {
        use std::os::windows::io::{FromRawSocket, IntoRawSocket};
        TcpSocket::from_raw_socket(socket.into_raw_socket())
    };
    if config.reuse_address {
        if let Err(e) = socket.set_reuseaddr(true) {
            warn!("tcp set_reuse_address error: {}", e);
        }
    }
    if let Some(size) = config.send_buffer_size {
        if let Err(e)
            = socket.set_send_buffer_size(size.try_into().unwrap_or(std::u32::MAX))
        {
            warn!("tcp set_buffer_size error: {}", e);
        }
    }
    if let Some(size) = config.recv_buffer_size {
        if let Err(e)
            = socket.set_recv_buffer_size(size.try_into().unwrap_or(std::u32::MAX))
        {
            warn!("tcp set_recv_buffer_size error: {}", e);
        }
    }
    let connect = socket.connect(*addr);
    Ok(async move {
        match connect_timeout {
            Some(dur) => {
                match tokio::time::timeout(dur, connect).await {
                    Ok(Ok(s)) => Ok(s),
                    Ok(Err(e)) => Err(e),
                    Err(e) => Err(io::Error::new(io::ErrorKind::TimedOut, e)),
                }
            }
            None => connect.await,
        }
            .map_err(ConnectError::m("tcp connect error"))
    })
}
impl ConnectingTcp<'_> {
    async fn connect(mut self) -> Result<TcpStream, ConnectError> {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use std::io;
    use ::http::Uri;
    use super::super::sealed::{Connect, ConnectSvc};
    use super::{Config, ConnectError, HttpConnector};
    async fn connect<C>(
        connector: C,
        dst: Uri,
    ) -> Result<<C::_Svc as ConnectSvc>::Connection, <C::_Svc as ConnectSvc>::Error>
    where
        C: Connect,
    {
        loop {}
    }
    #[tokio::test]
    async fn test_errors_enforce_http() {
        loop {}
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn get_local_ips() -> (Option<std::net::Ipv4Addr>, Option<std::net::Ipv6Addr>) {
        loop {}
    }
    #[tokio::test]
    async fn test_errors_missing_scheme() {
        loop {}
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[tokio::test]
    async fn local_address() {
        loop {}
    }
    #[test]
    #[cfg_attr(not(feature = "__internal_happy_eyeballs_tests"), ignore)]
    fn client_happy_eyeballs() {
        loop {}
    }
}
