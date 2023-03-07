use std::fmt;
use std::io;
use std::net::SocketAddr;
use std::time::Duration;

#[allow(unreachable_pub)]
pub use self::addr_stream::AddrStream;
use super::accept::Accept;
use crate::common::{task, Pin, Poll};
use tokio::net::TcpListener;
use tokio::time::Sleep;
#[derive(Default, Debug, Clone, Copy)]
struct TcpKeepaliveConfig {
    time: Option<Duration>,
    interval: Option<Duration>,
    retries: Option<u32>,
}
impl TcpKeepaliveConfig {
    #[cfg(not(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "fuchsia",
        target_os = "illumos",
        target_os = "linux",
        target_os = "netbsd",
        target_vendor = "apple",
        windows,
    )))]
    fn ka_with_interval(ka: TcpKeepalive, _: Duration, _: &mut bool) -> TcpKeepalive {
        loop {}
    }
    #[cfg(not(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "fuchsia",
        target_os = "illumos",
        target_os = "linux",
        target_os = "netbsd",
        target_vendor = "apple",
    )))]
    fn ka_with_retries(ka: TcpKeepalive, _: u32, _: &mut bool) -> TcpKeepalive {
        loop {}
    }
}

#[must_use = "streams do nothing unless polled"]
pub struct AddrIncoming {}
impl AddrIncoming {
    pub fn bind(addr: &SocketAddr) -> crate::Result<Self> {
        loop {}
    }

    pub fn local_addr(&self) -> SocketAddr {
        loop {}
    }

    pub fn set_nodelay(&mut self, enabled: bool) -> &mut Self {
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
impl fmt::Debug for AddrIncoming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
mod addr_stream {
    use crate::common::{task, Pin, Poll};
    use std::io;
    use std::net::SocketAddr;
    #[cfg(unix)]
    use std::os::unix::io::{AsRawFd, RawFd};
    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
    use tokio::net::TcpStream;
    pin_project_lite::pin_project! {
        #[doc = " A transport returned yieled by `AddrIncoming`."] #[derive(Debug)] pub
        struct AddrStream { #[pin] inner : TcpStream, pub (super) remote_addr :
        SocketAddr, pub (super) local_addr : SocketAddr }
    }
    impl AddrStream {}
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
        fn poll_flush(self: Pin<&mut Self>, _cx: &mut task::Context<'_>) -> Poll<io::Result<()>> {
            loop {}
        }
        #[inline]
        fn poll_shutdown(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<io::Result<()>> {
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
