use std::error::Error as StdError;
use std::marker::Unpin;
#[cfg(feature = "runtime")]
use std::time::Duration;
use bytes::Bytes;
use h2::server::{Connection, Handshake, SendResponse};
use h2::{Reason, RecvStream};
use http::{Method, Request};
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::{debug, trace, warn};
use super::{ping, PipeToSendStream, SendBuf};
use crate::body::HttpBody;
use crate::common::exec::ConnStreamExec;
use crate::common::{date, task, Future, Pin, Poll};
use crate::ext::Protocol;
use crate::headers;
use crate::proto::h2::ping::Recorder;
use crate::proto::h2::{H2Upgraded, UpgradedSendStream};
use crate::proto::Dispatched;
use crate::service::HttpService;
use crate::upgrade::{OnUpgrade, Pending, Upgraded};
use crate::{Body, Response};
const DEFAULT_CONN_WINDOW: u32 = 1024 * 1024;
const DEFAULT_STREAM_WINDOW: u32 = 1024 * 1024;
const DEFAULT_MAX_FRAME_SIZE: u32 = 1024 * 16;
const DEFAULT_MAX_SEND_BUF_SIZE: usize = 1024 * 400;
const DEFAULT_SETTINGS_MAX_HEADER_LIST_SIZE: u32 = 16 << 20;
#[derive(Clone, Debug)]
pub(crate) struct Config {
    pub(crate) adaptive_window: bool,
    pub(crate) initial_conn_window_size: u32,
    pub(crate) initial_stream_window_size: u32,
    pub(crate) max_frame_size: u32,
    pub(crate) enable_connect_protocol: bool,
    pub(crate) max_concurrent_streams: Option<u32>,
    #[cfg(feature = "runtime")]
    pub(crate) keep_alive_interval: Option<Duration>,
    #[cfg(feature = "runtime")]
    pub(crate) keep_alive_timeout: Duration,
    pub(crate) max_send_buffer_size: usize,
    pub(crate) max_header_list_size: u32,
}
impl Default for Config {
    fn default() -> Config {
        loop {}
    }
}
pin_project! {
    pub (crate) struct Server < T, S, B, E > where S : HttpService < Body >, B :
    HttpBody, { exec : E, service : S, state : State < T, B >, }
}
enum State<T, B>
where
    B: HttpBody,
{
    Handshaking { ping_config: ping::Config, hs: Handshake<T, SendBuf<B::Data>> },
    Serving(Serving<T, B>),
    Closed,
}
struct Serving<T, B>
where
    B: HttpBody,
{
    ping: Option<(ping::Recorder, ping::Ponger)>,
    conn: Connection<T, SendBuf<B::Data>>,
    closing: Option<crate::Error>,
}
impl<T, S, B, E> Server<T, S, B, E>
where
    T: AsyncRead + AsyncWrite + Unpin,
    S: HttpService<Body, ResBody = B>,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    B: HttpBody + 'static,
    E: ConnStreamExec<S::Future, B>,
{
    pub(crate) fn new(
        io: T,
        service: S,
        config: &Config,
        exec: E,
    ) -> Server<T, S, B, E> {
        loop {}
    }
    pub(crate) fn graceful_shutdown(&mut self) {
        loop {}
    }
}
impl<T, S, B, E> Future for Server<T, S, B, E>
where
    T: AsyncRead + AsyncWrite + Unpin,
    S: HttpService<Body, ResBody = B>,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    B: HttpBody + 'static,
    E: ConnStreamExec<S::Future, B>,
{
    type Output = crate::Result<Dispatched>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
impl<T, B> Serving<T, B>
where
    T: AsyncRead + AsyncWrite + Unpin,
    B: HttpBody + 'static,
{
    fn poll_server<S, E>(
        &mut self,
        cx: &mut task::Context<'_>,
        service: &mut S,
        exec: &mut E,
    ) -> Poll<crate::Result<()>>
    where
        S: HttpService<Body, ResBody = B>,
        S::Error: Into<Box<dyn StdError + Send + Sync>>,
        E: ConnStreamExec<S::Future, B>,
    {
        loop {}
    }
    fn poll_ping(&mut self, cx: &mut task::Context<'_>) {
        loop {}
    }
}
pin_project! {
    #[allow(missing_debug_implementations)] pub struct H2Stream < F, B > where B :
    HttpBody, { reply : SendResponse < SendBuf < B::Data >>, #[pin] state : H2StreamState
    < F, B >, }
}
pin_project! {
    #[project = H2StreamStateProj] enum H2StreamState < F, B > where B : HttpBody, {
    Service { #[pin] fut : F, connect_parts : Option < ConnectParts >, }, Body { #[pin]
    pipe : PipeToSendStream < B >, }, }
}
struct ConnectParts {
    pending: Pending,
    ping: Recorder,
    recv_stream: RecvStream,
}
impl<F, B> H2Stream<F, B>
where
    B: HttpBody,
{
    fn new(
        fut: F,
        connect_parts: Option<ConnectParts>,
        respond: SendResponse<SendBuf<B::Data>>,
    ) -> H2Stream<F, B> {
        loop {}
    }
}
macro_rules! reply {
    ($me:expr, $res:expr, $eos:expr) => {
        { match $me .reply.send_response($res, $eos) { Ok(tx) => tx, Err(e) => {
        debug!("send response error: {}", e); $me .reply
        .send_reset(Reason::INTERNAL_ERROR); return Poll::Ready(Err(crate
        ::Error::new_h2(e))); } } }
    };
}
impl<F, B, E> H2Stream<F, B>
where
    F: Future<Output = Result<Response<B>, E>>,
    B: HttpBody,
    B::Data: 'static,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
    E: Into<Box<dyn StdError + Send + Sync>>,
{
    fn poll2(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<crate::Result<()>> {
        loop {}
    }
}
impl<F, B, E> Future for H2Stream<F, B>
where
    F: Future<Output = Result<Response<B>, E>>,
    B: HttpBody,
    B::Data: 'static,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
    E: Into<Box<dyn StdError + Send + Sync>>,
{
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
