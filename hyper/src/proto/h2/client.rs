use std::error::Error as StdError;
#[cfg(feature = "runtime")]
use std::time::Duration;
use bytes::Bytes;
use futures_channel::{mpsc, oneshot};
use futures_util::future::{self, Either, FutureExt as _, TryFutureExt as _};
use futures_util::stream::StreamExt as _;
use h2::client::{Builder, SendRequest};
use h2::SendStream;
use http::{Method, StatusCode};
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::{debug, trace, warn};
use super::{ping, H2Upgraded, PipeToSendStream, SendBuf};
use crate::body::HttpBody;
use crate::client::dispatch::Callback;
use crate::common::{exec::Exec, task, Future, Never, Pin, Poll};
use crate::ext::Protocol;
use crate::headers;
use crate::proto::h2::UpgradedSendStream;
use crate::proto::Dispatched;
use crate::upgrade::Upgraded;
use crate::{Body, Request, Response};
use h2::client::ResponseFuture;
type ClientRx<B> = crate::client::dispatch::Receiver<Request<B>, Response<Body>>;
type ConnDropRef = mpsc::Sender<Never>;
type ConnEof = oneshot::Receiver<Never>;
const DEFAULT_CONN_WINDOW: u32 = 1024 * 1024 * 5;
const DEFAULT_STREAM_WINDOW: u32 = 1024 * 1024 * 2;
const DEFAULT_MAX_FRAME_SIZE: u32 = 1024 * 16;
const DEFAULT_MAX_SEND_BUF_SIZE: usize = 1024 * 1024;
#[derive(Clone, Debug)]
pub(crate) struct Config {
    pub(crate) adaptive_window: bool,
    pub(crate) initial_conn_window_size: u32,
    pub(crate) initial_stream_window_size: u32,
    pub(crate) max_frame_size: u32,
    #[cfg(feature = "runtime")]
    pub(crate) keep_alive_interval: Option<Duration>,
    #[cfg(feature = "runtime")]
    pub(crate) keep_alive_timeout: Duration,
    #[cfg(feature = "runtime")]
    pub(crate) keep_alive_while_idle: bool,
    pub(crate) max_concurrent_reset_streams: Option<usize>,
    pub(crate) max_send_buffer_size: usize,
}
impl Default for Config {
    fn default() -> Config {
        loop {}
    }
}
fn new_builder(config: &Config) -> Builder {
    loop {}
}
fn new_ping_config(config: &Config) -> ping::Config {
    loop {}
}
pub(crate) async fn handshake<T, B>(
    io: T,
    req_rx: ClientRx<B>,
    config: &Config,
    exec: Exec,
) -> crate::Result<ClientTask<B>>
where
    T: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    B: HttpBody,
    B::Data: Send + 'static,
{
    loop {}
}
async fn conn_task<C, D>(conn: C, drop_rx: D, cancel_tx: oneshot::Sender<Never>)
where
    C: Future + Unpin,
    D: Future<Output = ()> + Unpin,
{
    loop {}
}
struct FutCtx<B>
where
    B: HttpBody,
{
    is_connect: bool,
    eos: bool,
    fut: ResponseFuture,
    body_tx: SendStream<SendBuf<B::Data>>,
    body: B,
    cb: Callback<Request<B>, Response<Body>>,
}
impl<B: HttpBody> Unpin for FutCtx<B> {}
pub(crate) struct ClientTask<B>
where
    B: HttpBody,
{
    ping: ping::Recorder,
    conn_drop_ref: ConnDropRef,
    conn_eof: ConnEof,
    executor: Exec,
    h2_tx: SendRequest<SendBuf<B::Data>>,
    req_rx: ClientRx<B>,
    fut_ctx: Option<FutCtx<B>>,
}
impl<B> ClientTask<B>
where
    B: HttpBody + 'static,
{
    pub(crate) fn is_extended_connect_protocol_enabled(&self) -> bool {
        loop {}
    }
}
impl<B> ClientTask<B>
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    fn poll_pipe(&mut self, f: FutCtx<B>, cx: &mut task::Context<'_>) {
        loop {}
    }
}
impl<B> Future for ClientTask<B>
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Output = crate::Result<Dispatched>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
