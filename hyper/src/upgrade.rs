//! HTTP Upgrades
//!
//! This module deals with managing [HTTP Upgrades][mdn] in hyper. Since
//! several concepts in HTTP allow for first talking HTTP, and then converting
//! to a different protocol, this module conflates them into a single API.
//! Those include:
//!
//! - HTTP/1.1 Upgrades
//! - HTTP `CONNECT`
//!
//! You are responsible for any other pre-requisites to establish an upgrade,
//! such as sending the appropriate headers, methods, and status codes. You can
//! then use [`on`][] to grab a `Future` which will resolve to the upgraded
//! connection object, or an error if the upgrade fails.
//!
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Protocol_upgrade_mechanism
//!
//! # Client
//!
//! Sending an HTTP upgrade from the [`client`](super::client) involves setting
//! either the appropriate method, if wanting to `CONNECT`, or headers such as
//! `Upgrade` and `Connection`, on the `http::Request`. Once receiving the
//! `http::Response` back, you must check for the specific information that the
//! upgrade is agreed upon by the server (such as a `101` status code), and then
//! get the `Future` from the `Response`.
//!
//! # Server
//!
//! Receiving upgrade requests in a server requires you to check the relevant
//! headers in a `Request`, and if an upgrade should be done, you then send the
//! corresponding headers in a response. To then wait for hyper to finish the
//! upgrade, you call `on()` with the `Request`, and then can spawn a task
//! awaiting it.
//!
//! # Example
//!
//! See [this example][example] showing how upgrades work with both
//! Clients and Servers.
//!
//! [example]: https://github.com/hyperium/hyper/blob/master/examples/upgrades.rs
use std::any::TypeId;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::marker::Unpin;
use bytes::Bytes;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::sync::oneshot;
#[cfg(any(feature = "http1", feature = "http2"))]
use tracing::trace;
use crate::common::io::Rewind;
use crate::common::{task, Future, Pin, Poll};
/// An upgraded HTTP connection.
///
/// This type holds a trait object internally of the original IO that
/// was used to speak HTTP before the upgrade. It can be used directly
/// as a `Read` or `Write` for convenience.
///
/// Alternatively, if the exact type is known, this can be deconstructed
/// into its parts.
pub(crate) struct Upgraded {
    io: Rewind<Box<dyn Io + Send>>,
}
/// A future for a possible HTTP upgrade.
///
/// If no upgrade was available, or it doesn't succeed, yields an `Error`.
pub(crate) struct OnUpgrade {
    rx: Option<oneshot::Receiver<crate::Result<Upgraded>>>,
}
/// The deconstructed parts of an [`Upgraded`](Upgraded) type.
///
/// Includes the original IO type, and a read buffer of bytes that the
/// HTTP state machine may have already read before completing an upgrade.
#[derive(Debug)]
pub(crate) struct Parts<T> {
    /// The original IO object used before the upgrade.
    pub(crate) io: T,
    /// A buffer of bytes that have been read but not processed as HTTP.
    ///
    /// For instance, if the `Connection` is used for an HTTP upgrade request,
    /// it is possible the server sent back the first bytes of the new protocol
    /// along with the response upgrade.
    ///
    /// You will want to check for any existing bytes if you plan to continue
    /// communicating on the IO object.
    pub(crate) read_buf: Bytes,
    _inner: (),
}
/// Gets a pending HTTP upgrade from this message.
///
/// This can be called on the following types:
///
/// - `http::Request<B>`
/// - `http::Response<B>`
/// - `&mut http::Request<B>`
/// - `&mut http::Response<B>`
pub(crate) fn on<T: sealed::CanUpgrade>(msg: T) -> OnUpgrade {
    loop {}
}
#[cfg(any(feature = "http1", feature = "http2"))]
pub(super) struct Pending {
    tx: oneshot::Sender<crate::Result<Upgraded>>,
}
#[cfg(any(feature = "http1", feature = "http2"))]
pub(super) fn pending() -> (Pending, OnUpgrade) {
    loop {}
}
impl Upgraded {
    #[cfg(any(feature = "http1", feature = "http2", test))]
    pub(super) fn new<T>(io: T, read_buf: Bytes) -> Self
    where
        T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        loop {}
    }
    /// Tries to downcast the internal trait object to the type passed.
    ///
    /// On success, returns the downcasted parts. On error, returns the
    /// `Upgraded` back.
    pub(crate) fn downcast<T: AsyncRead + AsyncWrite + Unpin + 'static>(
        self,
    ) -> Result<Parts<T>, Self> {
        loop {}
    }
}
impl AsyncRead for Upgraded {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        loop {}
    }
}
impl AsyncWrite for Upgraded {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        loop {}
    }
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        loop {}
    }
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        loop {}
    }
    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        loop {}
    }
    fn is_write_vectored(&self) -> bool {
        loop {}
    }
}
impl fmt::Debug for Upgraded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl OnUpgrade {
    pub(super) fn none() -> Self {
        loop {}
    }
    #[cfg(feature = "http1")]
    pub(super) fn is_none(&self) -> bool {
        loop {}
    }
}
impl Future for OnUpgrade {
    type Output = Result<Upgraded, crate::Error>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
impl fmt::Debug for OnUpgrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
#[cfg(any(feature = "http1", feature = "http2"))]
impl Pending {
    pub(super) fn fulfill(self, upgraded: Upgraded) {
        loop {}
    }
    #[cfg(feature = "http1")]
    /// Don't fulfill the pending Upgrade, but instead signal that
    /// upgrades are handled manually.
    pub(super) fn manual(self) {
        loop {}
    }
}
/// Error cause returned when an upgrade was expected but canceled
/// for whatever reason.
///
/// This likely means the actual `Conn` future wasn't polled and upgraded.
#[derive(Debug)]
struct UpgradeExpected;
impl fmt::Display for UpgradeExpected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for UpgradeExpected {}
pub(super) trait Io: AsyncRead + AsyncWrite + Unpin + 'static {
    fn __hyper_type_id(&self) -> TypeId {
        loop {}
    }
}
impl<T: AsyncRead + AsyncWrite + Unpin + 'static> Io for T {}
impl dyn Io + Send {
    fn __hyper_is<T: Io>(&self) -> bool {
        loop {}
    }
    fn __hyper_downcast<T: Io>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        loop {}
    }
}
mod sealed {
    use super::OnUpgrade;
    pub(crate) trait CanUpgrade {
        fn on_upgrade(self) -> OnUpgrade;
    }
    impl<B> CanUpgrade for http::Request<B> {
        fn on_upgrade(mut self) -> OnUpgrade {
            loop {}
        }
    }
    impl<B> CanUpgrade for &'_ mut http::Request<B> {
        fn on_upgrade(self) -> OnUpgrade {
            loop {}
        }
    }
    impl<B> CanUpgrade for http::Response<B> {
        fn on_upgrade(mut self) -> OnUpgrade {
            loop {}
        }
    }
    impl<B> CanUpgrade for &'_ mut http::Response<B> {
        fn on_upgrade(self) -> OnUpgrade {
            loop {}
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn upgraded_downcast() {
        loop {}
    }
    struct Mock;
    impl AsyncRead for Mock {
        fn poll_read(
            self: Pin<&mut Self>,
            _cx: &mut task::Context<'_>,
            _buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            loop {}
        }
    }
    impl AsyncWrite for Mock {
        fn poll_write(
            self: Pin<&mut Self>,
            _: &mut task::Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            loop {}
        }
        fn poll_flush(
            self: Pin<&mut Self>,
            _cx: &mut task::Context<'_>,
        ) -> Poll<io::Result<()>> {
            loop {}
        }
        fn poll_shutdown(
            self: Pin<&mut Self>,
            _cx: &mut task::Context<'_>,
        ) -> Poll<io::Result<()>> {
            loop {}
        }
    }
}
