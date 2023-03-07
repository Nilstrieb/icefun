use bytes::{Buf, Bytes};
use h2::{RecvStream, SendStream};

use http::HeaderMap;
use pin_project_lite::pin_project;
use std::error::Error as StdError;
use std::io::{self, Cursor, IoSlice};

use std::task::Context;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use crate::body::HttpBody;
use crate::common::{task, Future, Pin, Poll};
use crate::proto::h2::ping::Recorder;
pub(crate) mod ping;
cfg_client! {
    pub (crate) mod client; pub (crate) use self::client::ClientTask;
}
cfg_server! {
    pub (crate) mod server; pub (crate) use self::server::Server;
}

pub(crate) const SPEC_WINDOW_SIZE: u32 = 65_535;
fn strip_connection_headers(headers: &mut HeaderMap, is_request: bool) {
    loop {}
}
pin_project! {
    struct PipeToSendStream < S > where S : HttpBody, { body_tx : SendStream < SendBuf <
    S::Data >>, data_done : bool, #[pin] stream : S, }
}
impl<S> PipeToSendStream<S>
where
    S: HttpBody,
{
    fn new(stream: S, tx: SendStream<SendBuf<S::Data>>) -> PipeToSendStream<S> {
        loop {}
    }
}
impl<S> Future for PipeToSendStream<S>
where
    S: HttpBody,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Output = crate::Result<()>;
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
trait SendStreamExt {
    fn on_user_err<E>(&mut self, err: E) -> crate::Error
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>;
    fn send_eos_frame(&mut self) -> crate::Result<()>;
}
impl<B: Buf> SendStreamExt for SendStream<SendBuf<B>> {
    fn on_user_err<E>(&mut self, err: E) -> crate::Error
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        loop {}
    }
    fn send_eos_frame(&mut self) -> crate::Result<()> {
        loop {}
    }
}
#[repr(usize)]
enum SendBuf<B> {
    Buf(B),
    Cursor(Cursor<Box<[u8]>>),
    None,
}
impl<B: Buf> Buf for SendBuf<B> {
    #[inline]
    fn remaining(&self) -> usize {
        loop {}
    }
    #[inline]
    fn chunk(&self) -> &[u8] {
        loop {}
    }
    #[inline]
    fn advance(&mut self, cnt: usize) {
        loop {}
    }
    fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
        loop {}
    }
}
struct H2Upgraded<B>
where
    B: Buf,
{
    ping: Recorder,
    send_stream: UpgradedSendStream<B>,
    recv_stream: RecvStream,
    buf: Bytes,
}
impl<B> AsyncRead for H2Upgraded<B>
where
    B: Buf,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        read_buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), io::Error>> {
        loop {}
    }
}
impl<B> AsyncWrite for H2Upgraded<B>
where
    B: Buf,
{
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        loop {}
    }
    fn poll_flush(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        loop {}
    }
    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        loop {}
    }
}
fn h2_to_io_error(e: h2::Error) -> io::Error {
    loop {}
}
struct UpgradedSendStream<B>(SendStream<SendBuf<Neutered<B>>>);
impl<B> UpgradedSendStream<B>
where
    B: Buf,
{
    unsafe fn new(inner: SendStream<SendBuf<B>>) -> Self {
        loop {}
    }
    fn reserve_capacity(&mut self, cnt: usize) {
        loop {}
    }
    fn poll_capacity(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<usize, h2::Error>>> {
        loop {}
    }
    fn poll_reset(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<h2::Reason, h2::Error>> {
        loop {}
    }
    fn write(&mut self, buf: &[u8], end_of_stream: bool) -> Result<(), io::Error> {
        loop {}
    }
    unsafe fn as_inner_unchecked(&mut self) -> &mut SendStream<SendBuf<B>> {
        loop {}
    }
}
#[repr(transparent)]
struct Neutered<B> {
    _inner: B,
    impossible: Impossible,
}
enum Impossible {}
unsafe impl<B> Send for Neutered<B> {}
impl<B> Buf for Neutered<B> {
    fn remaining(&self) -> usize {
        loop {}
    }
    fn chunk(&self) -> &[u8] {
        loop {}
    }
    fn advance(&mut self, _cnt: usize) {
        loop {}
    }
}
