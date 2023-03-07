//! Test utilities to test your filters.
//!
//! [`Filter`](../trait.Filter.html)s can be easily tested without starting up an HTTP
//! server, by making use of the [`RequestBuilder`](./struct.RequestBuilder.html) in this
//! module.
//!
//! # Testing Filters
//!
//! It's easy to test filters, especially if smaller filters are used to build
//! up your full set. Consider these example filters:
//!
//! ```
//! use warp::Filter;
//!
//! fn sum() -> impl Filter<Extract = (u32,), Error = warp::Rejection> + Copy {
//!     warp::path::param()
//!         .and(warp::path::param())
//!         .map(|x: u32, y: u32| {
//!             x + y
//!         })
//! }
//!
//! fn math() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
//!     warp::post()
//!         .and(sum())
//!         .map(|z: u32| {
//!             format!("Sum = {}", z)
//!         })
//! }
//! ```
//!
//! We can test some requests against the `sum` filter like this:
//!
//! ```
//! # use warp::Filter;
//! #[tokio::test]
//! async fn test_sum() {
//! #    let sum = || warp::any().map(|| 3);
//!     let filter = sum();
//!
//!     // Execute `sum` and get the `Extract` back.
//!     let value = warp::test::request()
//!         .path("/1/2")
//!         .filter(&filter)
//!         .await
//!         .unwrap();
//!     assert_eq!(value, 3);
//!
//!     // Or simply test if a request matches (doesn't reject).
//!     assert!(
//!         warp::test::request()
//!             .path("/1/-5")
//!             .matches(&filter)
//!             .await
//!     );
//! }
//! ```
//!
//! If the filter returns something that implements `Reply`, and thus can be
//! turned into a response sent back to the client, we can test what exact
//! response is returned. The `math` filter uses the `sum` filter, but returns
//! a `String` that can be turned into a response.
//!
//! ```
//! # use warp::Filter;
//! #[test]
//! fn test_math() {
//! #    let math = || warp::any().map(warp::reply);
//!     let filter = math();
//!
//!     let res = warp::test::request()
//!         .path("/1/2")
//!         .reply(&filter);
//!     assert_eq!(res.status(), 405, "GET is not allowed");
//!
//!     let res = warp::test::request()
//!         .method("POST")
//!         .path("/1/2")
//!         .reply(&filter);
//!     assert_eq!(res.status(), 200);
//!     assert_eq!(res.body(), "Sum is 3");
//! }
//! ```
use std::convert::TryFrom;
use std::error::Error as StdError;
use std::fmt;

use std::net::SocketAddr;
#[cfg(feature = "websocket")]
use std::pin::Pin;
#[cfg(feature = "websocket")]
use std::task::Context;
#[cfg(feature = "websocket")]
use std::task::{self, Poll};
use bytes::Bytes;
#[cfg(feature = "websocket")]
use futures_channel::mpsc;
#[cfg(feature = "websocket")]
use futures_util::StreamExt;

use http::{
    header::{HeaderName, HeaderValue},
    Response,
};
use serde::Serialize;
#[cfg(feature = "websocket")]
use tokio::sync::oneshot;
use crate::filter::Filter;
#[cfg(feature = "websocket")]
use crate::filters::ws::Message;
use crate::reject::IsReject;
use crate::reply::Reply;

use crate::Request;
#[cfg(feature = "websocket")]
use crate::{Sink, Stream};
use self::inner::OneOrTuple;

pub fn request() -> RequestBuilder {
    loop {}
}

#[cfg(feature = "websocket")]
pub fn ws() -> WsBuilder {
    loop {}
}



#[must_use = "RequestBuilder does nothing on its own"]
#[derive(Debug)]
pub struct RequestBuilder {
    remote_addr: Option<SocketAddr>,
    req: Request,
}



#[cfg(feature = "websocket")]
#[must_use = "WsBuilder does nothing on its own"]
#[derive(Debug)]
pub struct WsBuilder {
    req: RequestBuilder,
}

#[cfg(feature = "websocket")]
pub struct WsClient {
    tx: mpsc::UnboundedSender<crate::ws::Message>,
    rx: mpsc::UnboundedReceiver<Result<crate::ws::Message, crate::error::Error>>,
}

#[derive(Debug)]
pub struct WsError {
    cause: Box<dyn StdError + Send + Sync>,
}
impl RequestBuilder {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub fn method(mut self, method: &str) -> Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub fn path(mut self, p: &str) -> Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        HeaderValue: TryFrom<V>,
    {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    pub fn remote_addr(mut self, addr: SocketAddr) -> Self {
        loop {}
    }
    
    pub fn extension<T>(mut self, ext: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    pub fn body(mut self, body: impl AsRef<[u8]>) -> Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    pub fn json(mut self, val: &impl Serialize) -> Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub async fn filter<F>(
        self,
        f: &F,
    ) -> Result<<F::Extract as OneOrTuple>::Output, F::Error>
    where
        F: Filter,
        F::Future: Send + 'static,
        F::Extract: OneOrTuple + Send + 'static,
        F::Error: Send + 'static,
    {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub async fn matches<F>(self, f: &F) -> bool
    where
        F: Filter,
        F::Future: Send + 'static,
        F::Extract: Send + 'static,
        F::Error: Send + 'static,
    {
        loop {}
    }
    
    
    
    pub async fn reply<F>(self, f: &F) -> Response<Bytes>
    where
        F: Filter + 'static,
        F::Extract: Reply + Send,
        F::Error: IsReject + Send,
    {
        loop {}
    }
}
#[cfg(feature = "websocket")]
impl WsBuilder {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub fn path(self, p: &str) -> Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub fn header<K, V>(self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        HeaderValue: TryFrom<V>,
    {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub async fn handshake<F>(self, f: F) -> Result<WsClient, WsError>
    where
        F: Filter + Clone + Send + Sync + 'static,
        F::Extract: Reply + Send,
        F::Error: IsReject + Send,
    {
        loop {}
    }
}
#[cfg(feature = "websocket")]
impl WsClient {
    
    pub async fn send_text(&mut self, text: impl Into<String>) {
        loop {}
    }
    
    pub async fn send(&mut self, msg: crate::ws::Message) {
        loop {}
    }
    
    pub async fn recv(&mut self) -> Result<crate::filters::ws::Message, WsError> {
        loop {}
    }
    
    pub async fn recv_closed(&mut self) -> Result<(), WsError> {
        loop {}
    }
    fn pinned_tx(
        self: Pin<&mut Self>,
    ) -> Pin<&mut mpsc::UnboundedSender<crate::ws::Message>> {
        loop {}
    }
}
#[cfg(feature = "websocket")]
impl fmt::Debug for WsClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
#[cfg(feature = "websocket")]
impl Sink<crate::ws::Message> for WsClient {
    type Error = WsError;
    fn poll_ready(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn start_send(self: Pin<&mut Self>, message: Message) -> Result<(), Self::Error> {
        loop {}
    }
    fn poll_flush(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn poll_close(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
}
#[cfg(feature = "websocket")]
impl Stream for WsClient {
    type Item = Result<crate::ws::Message, WsError>;
    fn poll_next(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {}
    }
}
#[cfg(feature = "websocket")]
impl WsError {
    fn new<E: Into<Box<dyn StdError + Send + Sync>>>(cause: E) -> Self {
        loop {}
    }
}
impl fmt::Display for WsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for WsError {
    fn description(&self) -> &str {
        loop {}
    }
}
#[cfg(feature = "websocket")]
#[derive(Clone)]
struct AddrConnect(SocketAddr);
#[cfg(feature = "websocket")]
impl tower_service::Service<::http::Uri> for AddrConnect {
    type Response = ::tokio::net::TcpStream;
    type Error = ::std::io::Error;
    type Future = Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;
    fn poll_ready(
        &mut self,
        _cx: &mut task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn call(&mut self, _: ::http::Uri) -> Self::Future {
        loop {}
    }
}
mod inner {
    pub trait OneOrTuple {
        type Output;
        fn one_or_tuple(self) -> Self::Output;
    }
    impl OneOrTuple for () {
        type Output = ();
        fn one_or_tuple(self) -> Self::Output {}
    }
    macro_rules! one_or_tuple {
        ($type1:ident) => {
            impl <$type1 > OneOrTuple for ($type1,) { type Output = $type1; fn
            one_or_tuple(self) -> Self::Output { self.0 } }
        };
        ($type1:ident, $($type:ident),*) => {
            one_or_tuple!($($type),*); impl <$type1, $($type),*> OneOrTuple for ($type1,
            $($type),*) { type Output = Self; fn one_or_tuple(self) -> Self::Output {
            self } }
        };
    }
    one_or_tuple! {
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
    }
}
