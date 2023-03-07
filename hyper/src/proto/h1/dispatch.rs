use std::error::Error as StdError;
use bytes::{Buf, Bytes};
use http::Request;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::{trace};
use super::{Http1Transaction};
use crate::body::{Body, HttpBody};
use crate::common::{task, Future, Pin, Poll, Unpin};
use crate::proto::{Conn, Dispatched, MessageHead, RequestHead};

pub(crate) struct Dispatcher<D, Bs: HttpBody, I, T> {
    conn: Conn<I, Bs::Data, T>,
    dispatch: D,
    body_tx: Option<crate::body::Sender>,
    body_rx: Pin<Box<Option<Bs>>>,
    is_closing: bool,
}
pub(crate) trait Dispatch {
    type PollItem;
    type PollBody;
    type PollError;
    type RecvItem;
    fn poll_msg(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<Result<(Self::PollItem, Self::PollBody), Self::PollError>>>;
    fn recv_msg(
        &mut self,
        msg: crate::Result<(Self::RecvItem, Body)>,
    ) -> crate::Result<()>;
    fn poll_ready(&mut self, cx: &mut task::Context<'_>) -> Poll<Result<(), ()>>;
    fn should_poll(&self) -> bool;
}
cfg_server! {
    use crate ::service::HttpService; pub (crate) struct Server < S : HttpService < B >,
    B > { in_flight : Pin < Box < Option < S::Future >>>, pub (crate) service : S, }
}
cfg_client! {
    pin_project_lite::pin_project! { pub (crate) struct Client < B > { callback : Option
    < crate ::client::dispatch::Callback < Request < B >, http::Response < Body >>>,
    #[pin] rx : ClientRx < B >, rx_closed : bool, } } type ClientRx < B > = crate
    ::client::dispatch::Receiver < Request < B >, http::Response < Body >>;
}
impl<D, Bs, I, T> Dispatcher<D, Bs, I, T>
where
    D: Dispatch<
            PollItem = MessageHead<T::Outgoing>,
            PollBody = Bs,
            RecvItem = MessageHead<T::Incoming>,
        > + Unpin,
    D::PollError: Into<Box<dyn StdError + Send + Sync>>,
    I: AsyncRead + AsyncWrite + Unpin,
    T: Http1Transaction + Unpin,
    Bs: HttpBody + 'static,
    Bs::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    pub(crate) fn new(dispatch: D, conn: Conn<I, Bs::Data, T>) -> Self {
        loop {}
    }
    #[cfg(feature = "server")]
    pub(crate) fn disable_keep_alive(&mut self) {
        loop {}
    }
    pub(crate) fn into_inner(self) -> (I, Bytes, D) {
        loop {}
    }
    
    
    
    
    
    pub(crate) fn poll_without_shutdown(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<crate::Result<()>>
    where
        Self: Unpin,
    {
        loop {}
    }
    fn poll_catch(
        &mut self,
        cx: &mut task::Context<'_>,
        should_shutdown: bool,
    ) -> Poll<crate::Result<Dispatched>> {
        loop {}
    }
    fn poll_inner(
        &mut self,
        cx: &mut task::Context<'_>,
        should_shutdown: bool,
    ) -> Poll<crate::Result<Dispatched>> {
        loop {}
    }
    fn poll_loop(&mut self, cx: &mut task::Context<'_>) -> Poll<crate::Result<()>> {
        loop {}
    }
    fn poll_read(&mut self, cx: &mut task::Context<'_>) -> Poll<crate::Result<()>> {
        loop {}
    }
    fn poll_read_head(&mut self, cx: &mut task::Context<'_>) -> Poll<crate::Result<()>> {
        loop {}
    }
    fn poll_write(&mut self, cx: &mut task::Context<'_>) -> Poll<crate::Result<()>> {
        loop {}
    }
    fn poll_flush(&mut self, cx: &mut task::Context<'_>) -> Poll<crate::Result<()>> {
        loop {}
    }
    fn close(&mut self) {
        loop {}
    }
    fn is_done(&self) -> bool {
        loop {}
    }
}
impl<D, Bs, I, T> Future for Dispatcher<D, Bs, I, T>
where
    D: Dispatch<
            PollItem = MessageHead<T::Outgoing>,
            PollBody = Bs,
            RecvItem = MessageHead<T::Incoming>,
        > + Unpin,
    D::PollError: Into<Box<dyn StdError + Send + Sync>>,
    I: AsyncRead + AsyncWrite + Unpin,
    T: Http1Transaction + Unpin,
    Bs: HttpBody + 'static,
    Bs::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Output = crate::Result<Dispatched>;
    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}


struct OptGuard<'a, T>(Pin<&'a mut Option<T>>, bool);
impl<'a, T> OptGuard<'a, T> {
    fn new(pin: Pin<&'a mut Option<T>>) -> Self {
        loop {}
    }
    fn guard_mut(&mut self) -> (Option<Pin<&mut T>>, &mut bool) {
        loop {}
    }
}
impl<'a, T> Drop for OptGuard<'a, T> {
    fn drop(&mut self) {
        loop {}
    }
}
cfg_server! {
    impl < S, B > Server < S, B > where S : HttpService < B >, { pub (crate) fn
    new(service : S) -> Server < S, B > { Server { in_flight : Box::pin(None), service, }
    } pub (crate) fn into_service(self) -> S { self.service } } impl < S : HttpService <
    B >, B > Unpin for Server < S, B > {} impl < S, Bs > Dispatch for Server < S, Body >
    where S : HttpService < Body, ResBody = Bs >, S::Error : Into < Box < dyn StdError +
    Send + Sync >>, Bs : HttpBody, { type PollItem = MessageHead < http::StatusCode >;
    type PollBody = Bs; type PollError = S::Error; type RecvItem = RequestHead; fn
    poll_msg(mut self : Pin <& mut Self >, cx : & mut task::Context <'_ >,) -> Poll <
    Option < Result < (Self::PollItem, Self::PollBody), Self::PollError >>> { let mut
    this = self.as_mut(); let ret = if let Some(ref mut fut) = this.in_flight.as_mut()
    .as_pin_mut() { let resp = ready!(fut.as_mut().poll(cx) ?); let (parts, body) = resp
    .into_parts(); let head = MessageHead { version : parts.version, subject : parts
    .status, headers : parts.headers, extensions : parts.extensions, };
    Poll::Ready(Some(Ok((head, body)))) } else {
    unreachable!("poll_msg shouldn't be called if no inflight"); }; this.in_flight
    .set(None); ret } fn recv_msg(& mut self, msg : crate ::Result < (Self::RecvItem,
    Body) >) -> crate ::Result < () > { let (msg, body) = msg ?; let mut req =
    Request::new(body); * req.method_mut() = msg.subject.0; * req.uri_mut() = msg.subject
    .1; * req.headers_mut() = msg.headers; * req.version_mut() = msg.version; * req
    .extensions_mut() = msg.extensions; let fut = self.service.call(req); self.in_flight
    .set(Some(fut)); Ok(()) } fn poll_ready(& mut self, cx : & mut task::Context <'_ >)
    -> Poll < Result < (), () >> { if self.in_flight.is_some() { Poll::Pending } else {
    self.service.poll_ready(cx).map_err(| _e | { trace!("service closed"); }) } } fn
    should_poll(& self) -> bool { self.in_flight.is_some() } }
}
cfg_client! {
    impl < B > Client < B > { pub (crate) fn new(rx : ClientRx < B >) -> Client < B > {
    Client { callback : None, rx, rx_closed : false, } } } impl < B > Dispatch for Client
    < B > where B : HttpBody, { type PollItem = RequestHead; type PollBody = B; type
    PollError = crate ::common::Never; type RecvItem = crate ::proto::ResponseHead; fn
    poll_msg(mut self : Pin <& mut Self >, cx : & mut task::Context <'_ >,) -> Poll <
    Option < Result < (Self::PollItem, Self::PollBody), crate ::common::Never >>> { let
    mut this = self.as_mut(); debug_assert!(! this.rx_closed); match this.rx
    .poll_recv(cx) { Poll::Ready(Some((req, mut cb))) => { match cb.poll_canceled(cx) {
    Poll::Ready(()) => { trace!("request canceled"); Poll::Ready(None) } Poll::Pending =>
    { let (parts, body) = req.into_parts(); let head = RequestHead { version : parts
    .version, subject : crate ::proto::RequestLine(parts.method, parts.uri), headers :
    parts.headers, extensions : parts.extensions, }; this.callback = Some(cb);
    Poll::Ready(Some(Ok((head, body)))) } } } Poll::Ready(None) => {
    trace!("client tx closed"); this.rx_closed = true; Poll::Ready(None) } Poll::Pending
    => Poll::Pending, } } fn recv_msg(& mut self, msg : crate ::Result < (Self::RecvItem,
    Body) >) -> crate ::Result < () > { match msg { Ok((msg, body)) => { if let Some(cb)
    = self.callback.take() { let res = msg.into_response(body); cb.send(Ok(res)); Ok(())
    } else { Err(crate ::Error::new_unexpected_message()) } } Err(err) => { if let
    Some(cb) = self.callback.take() { cb.send(Err((err, None))); Ok(()) } else if ! self
    .rx_closed { self.rx.close(); if let Some((req, cb)) = self.rx.try_recv() {
    trace!("canceling queued request with connection error: {}", err); cb.send(Err((crate
    ::Error::new_canceled().with(err), Some(req)))); Ok(()) } else { Err(err) } } else {
    Err(err) } } } } fn poll_ready(& mut self, cx : & mut task::Context <'_ >) -> Poll <
    Result < (), () >> { match self.callback { Some(ref mut cb) => match cb
    .poll_canceled(cx) { Poll::Ready(()) => { trace!("callback receiver has dropped");
    Poll::Ready(Err(())) } Poll::Pending => Poll::Ready(Ok(())), }, None =>
    Poll::Ready(Err(())), } } fn should_poll(& self) -> bool { self.callback.is_none() }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::h1::ClientTransaction;
    use std::time::Duration;
    #[test]
    fn client_read_bytes_before_writing_request() {
        loop {}
    }
    #[tokio::test]
    async fn client_flushing_is_not_ready_for_next_request() {
        loop {}
    }
    #[tokio::test]
    async fn body_empty_chunks_ignored() {
        loop {}
    }
}
