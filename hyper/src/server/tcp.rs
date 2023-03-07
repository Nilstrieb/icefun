use std::fmt;
use std::io;
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use std::time::Duration;
use socket2::TcpKeepalive;
use tokio::net::TcpListener;
use tokio::time::Sleep;
use crate::common::{task, Pin, Poll};
#[allow(unreachable_pub)]
pub use self::addr_stream::AddrStream;
use super::accept::Accept;
#[derive(Default, Debug, Clone, Copy)]
struct TcpKeepaliveConfig {
    time: Option<Duration>,
    interval: Option<Duration>,
    retries: Option<u32>,
}
impl TcpKeepaliveConfig {
    /// Converts into a `socket2::TcpKeealive` if there is any keep alive configuration.
    fn into_socket2(self) -> Option<TcpKeepalive> {
        loop {}
    }
    #[cfg(
        any(
            target_os = "android",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "illumos",
            target_os = "linux",
            target_os = "netbsd",
            target_vendor = "apple",
            windows,
        )
    )]
    fn ka_with_interval(
        ka: TcpKeepalive,
        interval: Duration,
        dirty: &mut bool,
    ) -> TcpKeepalive {
        loop {}
    }
    #[cfg(
        not(
            any(
                target_os = "android",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "fuchsia",
                target_os = "illumos",
                target_os = "linux",
                target_os = "netbsd",
                target_vendor = "apple",
                windows,
            )
        )
    )]
    fn ka_with_interval(ka: TcpKeepalive, _: Duration, _: &mut bool) -> TcpKeepalive {
        loop {}
    }
    #[cfg(
        any(
            target_os = "android",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "illumos",
            target_os = "linux",
            target_os = "netbsd",
            target_vendor = "apple",
        )
    )]
    fn ka_with_retries(
        ka: TcpKeepalive,
        retries: u32,
        dirty: &mut bool,
    ) -> TcpKeepalive {
        loop {}
    }
    #[cfg(
        not(
            any(
                target_os = "android",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "fuchsia",
                target_os = "illumos",
                target_os = "linux",
                target_os = "netbsd",
                target_vendor = "apple",
            )
        )
    )]
    fn ka_with_retries(ka: TcpKeepalive, _: u32, _: &mut bool) -> TcpKeepalive {
        loop {}
    }
}
/// A stream of connections from binding to an address.
#[must_use = "streams do nothing unless polled"]
pub struct AddrIncoming {
    addr: SocketAddr,
    listener: TcpListener,
    sleep_on_errors: bool,
    tcp_keepalive_config: TcpKeepaliveConfig,
    tcp_nodelay: bool,
    timeout: Option<Pin<Box<Sleep>>>,
}
impl AddrIncoming {
    pub(super) fn new(addr: &SocketAddr) -> crate::Result<Self> {
        loop {}
    }
    pub(super) fn from_std(std_listener: StdTcpListener) -> crate::Result<Self> {
        loop {}
    }
    /// Creates a new `AddrIncoming` binding to provided socket address.
    pub fn bind(addr: &SocketAddr) -> crate::Result<Self> {
        loop {}
    }
    /// Creates a new `AddrIncoming` from an existing `tokio::net::TcpListener`.
    pub fn from_listener(listener: TcpListener) -> crate::Result<Self> {
        loop {}
    }
    /// Get the local address bound to this listener.
    pub fn local_addr(&self) -> SocketAddr {
        loop {}
    }
    /// Set the duration to remain idle before sending TCP keepalive probes.
    ///
    /// If `None` is specified, keepalive is disabled.
    pub fn set_keepalive(&mut self, time: Option<Duration>) -> &mut Self {
        loop {}
    }
    /// Set the duration between two successive TCP keepalive retransmissions,
    /// if acknowledgement to the previous keepalive transmission is not received.
    pub fn set_keepalive_interval(&mut self, interval: Option<Duration>) -> &mut Self {
        loop {}
    }
    /// Set the number of retransmissions to be carried out before declaring that remote end is not available.
    pub fn set_keepalive_retries(&mut self, retries: Option<u32>) -> &mut Self {
        loop {}
    }
    /// Set the value of `TCP_NODELAY` option for accepted connections.
    pub fn set_nodelay(&mut self, enabled: bool) -> &mut Self {
        loop {}
    }
    /// Set whether to sleep on accept errors.
    ///
    /// A possible scenario is that the process has hit the max open files
    /// allowed, and so trying to accept a new connection will fail with
    /// `EMFILE`. In some cases, it's preferable to just wait for some time, if
    /// the application will likely close some files (or connections), and try
    /// to accept the connection again. If this option is `true`, the error
    /// will be logged at the `error` level, since it is still a big deal,
    /// and then the listener will sleep for 1 second.
    ///
    /// In other cases, hitting the max open files should be treat similarly
    /// to being out-of-memory, and simply error (and shutdown). Setting
    /// this option to `false` will allow that.
    ///
    /// Default is `true`.
    pub fn set_sleep_on_errors(&mut self, val: bool) {
        loop {}
    }
    fn poll_next_(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<io::Result<AddrStream>> {
        loop {}
    }
}
impl Accept for AddrIncoming {
    type Conn = AddrStream;
    type Error = io::Error;
    fn poll_accept(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        loop {}
    }
}
/// This function defines errors that are per-connection. Which basically
/// means that if we get this error from `accept()` system call it means
/// next connection might be ready to be accepted.
///
/// All other errors will incur a timeout before next `accept()` is performed.
/// The timeout is useful to handle resource exhaustion errors like ENFILE
/// and EMFILE. Otherwise, could enter into tight loop.
fn is_connection_error(e: &io::Error) -> bool {
    loop {}
}
impl fmt::Debug for AddrIncoming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
mod addr_stream {
    use std::io;
    use std::net::SocketAddr;
    #[cfg(unix)]
    use std::os::unix::io::{AsRawFd, RawFd};
    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
    use tokio::net::TcpStream;
    use crate::common::{task, Pin, Poll};
    pin_project_lite::pin_project! {
        #[doc = " A transport returned yieled by `AddrIncoming`."] #[derive(Debug)] pub
        struct AddrStream { #[pin] inner : TcpStream, pub (super) remote_addr :
        SocketAddr, pub (super) local_addr : SocketAddr }
    }
    impl AddrStream {
        pub(super) fn new(
            tcp: TcpStream,
            remote_addr: SocketAddr,
            local_addr: SocketAddr,
        ) -> AddrStream {
            loop {}
        }
        /// Returns the remote (peer) address of this connection.
        #[inline]
        pub fn remote_addr(&self) -> SocketAddr {
            loop {}
        }
        /// Returns the local address of this connection.
        #[inline]
        pub fn local_addr(&self) -> SocketAddr {
            loop {}
        }
        /// Consumes the AddrStream and returns the underlying IO object
        #[inline]
        pub fn into_inner(self) -> TcpStream {
            loop {}
        }
        /// Attempt to receive data on the socket, without removing that data
        /// from the queue, registering the current task for wakeup if data is
        /// not yet available.
        pub fn poll_peek(
            &mut self,
            cx: &mut task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> Poll<io::Result<usize>> {
            loop {}
        }
    }
    impl AsyncRead for AddrStream {
        #[inline]
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut task::Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            loop {}
        }
    }
    impl AsyncWrite for AddrStream {
        #[inline]
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut task::Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            loop {}
        }
        #[inline]
        fn poll_write_vectored(
            self: Pin<&mut Self>,
            cx: &mut task::Context<'_>,
            bufs: &[io::IoSlice<'_>],
        ) -> Poll<io::Result<usize>> {
            loop {}
        }
        #[inline]
        fn poll_flush(
            self: Pin<&mut Self>,
            _cx: &mut task::Context<'_>,
        ) -> Poll<io::Result<()>> {
            loop {}
        }
        #[inline]
        fn poll_shutdown(
            self: Pin<&mut Self>,
            cx: &mut task::Context<'_>,
        ) -> Poll<io::Result<()>> {
            loop {}
        }
        #[inline]
        fn is_write_vectored(&self) -> bool {
            loop {}
        }
    }
    #[cfg(unix)]
    impl AsRawFd for AddrStream {
        fn as_raw_fd(&self) -> RawFd {
            loop {}
        }
    }
}
