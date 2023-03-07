use std::marker::Unpin;
use std::{io};
use bytes::{Bytes};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use crate::common::{task, Pin, Poll};

#[derive(Debug)]
pub(crate) struct Rewind<T> {
    pre: Option<Bytes>,
    inner: T,
}
impl<T> Rewind<T> {
    #[cfg(any(all(feature = "http2", feature = "server"), test))]
    pub(crate) fn new(io: T) -> Self {
        loop {}
    }
    pub(crate) fn new_buffered(io: T, buf: Bytes) -> Self {
        loop {}
    }
    #[cfg(any(all(feature = "http1", feature = "http2", feature = "server"), test))]
    pub(crate) fn rewind(&mut self, bs: Bytes) {
        loop {}
    }
    pub(crate) fn into_inner(self) -> (T, Bytes) {
        loop {}
    }
}
impl<T> AsyncRead for Rewind<T>
where
    T: AsyncRead + Unpin,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        loop {}
    }
}
impl<T> AsyncWrite for Rewind<T>
where
    T: AsyncWrite + Unpin,
{
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
#[cfg(test)]
mod tests {
    use super::Rewind;
    use bytes::Bytes;
    use tokio::io::AsyncReadExt;
    #[tokio::test]
    async fn partial_rewind() {
        loop {}
    }
    #[tokio::test]
    async fn full_rewind() {
        loop {}
    }
}
