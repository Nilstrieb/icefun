//! Websockets Filters
use std::borrow::Cow;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use super::header;
use crate::filter::{filter_fn_one, Filter, One};
use crate::reject::Rejection;
use crate::reply::{Reply, Response};
use futures_util::{future, ready, FutureExt, Sink, Stream, TryFutureExt};
use headers::{Connection, HeaderMapExt, SecWebsocketAccept, SecWebsocketKey, Upgrade};
use http;
use hyper::upgrade::OnUpgrade;
use tokio_tungstenite::{
    tungstenite::protocol::{self, WebSocketConfig},
    WebSocketStream,
};
/// Creates a Websocket Filter.
///
/// The yielded `Ws` is used to finish the websocket upgrade.
///
/// # Note
///
/// This filter combines multiple filters internally, so you don't need them:
///
/// - Method must be `GET`
/// - Header `connection` must be `upgrade`
/// - Header `upgrade` must be `websocket`
/// - Header `sec-websocket-version` must be `13`
/// - Header `sec-websocket-key` must be set.
///
/// If the filters are met, yields a `Ws`. Calling `Ws::on_upgrade` will
/// return a reply with:
///
/// - Status of `101 Switching Protocols`
/// - Header `connection: upgrade`
/// - Header `upgrade: websocket`
/// - Header `sec-websocket-accept` with the hash value of the received key.
pub fn ws() -> impl Filter<Extract = One<Ws>, Error = Rejection> + Copy {
    loop {}
}
/// Extracted by the [`ws`](ws) filter, and used to finish an upgrade.
pub struct Ws {
    config: Option<WebSocketConfig>,
    key: SecWebsocketKey,
    on_upgrade: Option<OnUpgrade>,
}
impl Ws {
    /// Finish the upgrade, passing a function to handle the `WebSocket`.
    ///
    /// The passed function must return a `Future`.
    pub fn on_upgrade<F, U>(self, func: F) -> impl Reply
    where
        F: FnOnce(WebSocket) -> U + Send + 'static,
        U: Future<Output = ()> + Send + 'static,
    {
        loop {}
    }
    /// Set the size of the internal message send queue.
    pub fn max_send_queue(mut self, max: usize) -> Self {
        loop {}
    }
    /// Set the maximum message size (defaults to 64 megabytes)
    pub fn max_message_size(mut self, max: usize) -> Self {
        loop {}
    }
    /// Set the maximum frame size (defaults to 16 megabytes)
    pub fn max_frame_size(mut self, max: usize) -> Self {
        loop {}
    }
}
impl fmt::Debug for Ws {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
struct WsReply<F> {
    ws: Ws,
    on_upgrade: F,
}
impl<F, U> Reply for WsReply<F>
where
    F: FnOnce(WebSocket) -> U + Send + 'static,
    U: Future<Output = ()> + Send + 'static,
{
    fn into_response(self) -> Response {
        loop {}
    }
}
fn on_upgrade() -> impl Filter<
    Extract = (Option<OnUpgrade>,),
    Error = Rejection,
> + Copy {
    loop {}
}
/// A websocket `Stream` and `Sink`, provided to `ws` filters.
///
/// Ping messages sent from the client will be handled internally by replying with a Pong message.
/// Close messages need to be handled explicitly: usually by closing the `Sink` end of the
/// `WebSocket`.
///
/// **Note!**
/// Due to rust futures nature, pings won't be handled until read part of `WebSocket` is polled
pub struct WebSocket {
    inner: WebSocketStream<hyper::upgrade::Upgraded>,
}
impl WebSocket {
    pub(crate) async fn from_raw_socket(
        upgraded: hyper::upgrade::Upgraded,
        role: protocol::Role,
        config: Option<protocol::WebSocketConfig>,
    ) -> Self {
        loop {}
    }
    /// Gracefully close this websocket.
    pub async fn close(mut self) -> Result<(), crate::Error> {
        loop {}
    }
}
impl Stream for WebSocket {
    type Item = Result<Message, crate::Error>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {}
    }
}
impl Sink<Message> for WebSocket {
    type Error = crate::Error;
    fn poll_ready(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        loop {}
    }
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
}
impl fmt::Debug for WebSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
/// A WebSocket message.
///
/// This will likely become a `non-exhaustive` enum in the future, once that
/// language feature has stabilized.
#[derive(Eq, PartialEq, Clone)]
pub struct Message {
    inner: protocol::Message,
}
impl Message {
    /// Construct a new Text `Message`.
    pub fn text<S: Into<String>>(s: S) -> Message {
        loop {}
    }
    /// Construct a new Binary `Message`.
    pub fn binary<V: Into<Vec<u8>>>(v: V) -> Message {
        loop {}
    }
    /// Construct a new Ping `Message`.
    pub fn ping<V: Into<Vec<u8>>>(v: V) -> Message {
        loop {}
    }
    /// Construct a new Pong `Message`.
    ///
    /// Note that one rarely needs to manually construct a Pong message because the underlying tungstenite socket
    /// automatically responds to the Ping messages it receives. Manual construction might still be useful in some cases
    /// like in tests or to send unidirectional heartbeats.
    pub fn pong<V: Into<Vec<u8>>>(v: V) -> Message {
        loop {}
    }
    /// Construct the default Close `Message`.
    pub fn close() -> Message {
        loop {}
    }
    /// Construct a Close `Message` with a code and reason.
    pub fn close_with(
        code: impl Into<u16>,
        reason: impl Into<Cow<'static, str>>,
    ) -> Message {
        loop {}
    }
    /// Returns true if this message is a Text message.
    pub fn is_text(&self) -> bool {
        loop {}
    }
    /// Returns true if this message is a Binary message.
    pub fn is_binary(&self) -> bool {
        loop {}
    }
    /// Returns true if this message a is a Close message.
    pub fn is_close(&self) -> bool {
        loop {}
    }
    /// Returns true if this message is a Ping message.
    pub fn is_ping(&self) -> bool {
        loop {}
    }
    /// Returns true if this message is a Pong message.
    pub fn is_pong(&self) -> bool {
        loop {}
    }
    /// Try to get the close frame (close code and reason)
    pub fn close_frame(&self) -> Option<(u16, &str)> {
        loop {}
    }
    /// Try to get a reference to the string text, if this is a Text message.
    pub fn to_str(&self) -> Result<&str, ()> {
        loop {}
    }
    /// Return the bytes of this message, if the message can contain data.
    pub fn as_bytes(&self) -> &[u8] {
        loop {}
    }
    /// Destructure this message into binary data.
    pub fn into_bytes(self) -> Vec<u8> {
        loop {}
    }
}
impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl From<Message> for Vec<u8> {
    fn from(m: Message) -> Self {
        loop {}
    }
}
/// Connection header did not include 'upgrade'
#[derive(Debug)]
pub struct MissingConnectionUpgrade;
impl fmt::Display for MissingConnectionUpgrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl ::std::error::Error for MissingConnectionUpgrade {}
