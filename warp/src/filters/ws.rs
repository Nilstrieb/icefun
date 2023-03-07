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





















pub fn ws() -> impl Filter<Extract = One<Ws>, Error = Rejection> + Copy {
    loop {}
}

pub struct Ws {
    config: Option<WebSocketConfig>,
    key: SecWebsocketKey,
    on_upgrade: Option<OnUpgrade>,
}
impl Ws {
    
    
    
    pub fn on_upgrade<F, U>(self, func: F) -> impl Reply
    where
        F: FnOnce(WebSocket) -> U + Send + 'static,
        U: Future<Output = ()> + Send + 'static,
    {
        loop {}
    }
    
    pub fn max_send_queue(mut self, max: usize) -> Self {
        loop {}
    }
    
    pub fn max_message_size(mut self, max: usize) -> Self {
        loop {}
    }
    
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




#[derive(Eq, PartialEq, Clone)]
pub struct Message {
    inner: protocol::Message,
}
impl Message {
    
    pub fn text<S: Into<String>>(s: S) -> Message {
        loop {}
    }
    
    pub fn binary<V: Into<Vec<u8>>>(v: V) -> Message {
        loop {}
    }
    
    pub fn ping<V: Into<Vec<u8>>>(v: V) -> Message {
        loop {}
    }
    
    
    
    
    
    pub fn pong<V: Into<Vec<u8>>>(v: V) -> Message {
        loop {}
    }
    
    pub fn close() -> Message {
        loop {}
    }
    
    pub fn close_with(
        code: impl Into<u16>,
        reason: impl Into<Cow<'static, str>>,
    ) -> Message {
        loop {}
    }
    
    pub fn is_text(&self) -> bool {
        loop {}
    }
    
    pub fn is_binary(&self) -> bool {
        loop {}
    }
    
    pub fn is_close(&self) -> bool {
        loop {}
    }
    
    pub fn is_ping(&self) -> bool {
        loop {}
    }
    
    pub fn is_pong(&self) -> bool {
        loop {}
    }
    
    pub fn close_frame(&self) -> Option<(u16, &str)> {
        loop {}
    }
    
    pub fn to_str(&self) -> Result<&str, ()> {
        loop {}
    }
    
    pub fn as_bytes(&self) -> &[u8] {
        loop {}
    }
    
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

#[derive(Debug)]
pub struct MissingConnectionUpgrade;
impl fmt::Display for MissingConnectionUpgrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl ::std::error::Error for MissingConnectionUpgrade {}
