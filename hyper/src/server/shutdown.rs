use std::error::Error as StdError;
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, AsyncWrite};
use super::accept::Accept;
use super::conn::UpgradeableConnection;
use super::server::{Server, Watcher};
use crate::body::{Body, HttpBody};
use crate::common::drain::{Draining, Signal, Watch, Watching};
use crate::common::exec::{ConnStreamExec, NewSvcExec};
use crate::common::{task, Future, Pin, Poll, Unpin};
use crate::service::{HttpService, MakeServiceRef};
pin_project! {
    #[allow(missing_debug_implementations)] pub struct Graceful < I, S, F, E > { #[pin]
    state : State < I, S, F, E >, }
}
pin_project! {
    #[project = StateProj] pub (super) enum State < I, S, F, E > { Running { drain :
    Option < (Signal, Watch) >, #[pin] server : Server < I, S, E >, #[pin] signal : F, },
    Draining { draining : Draining }, }
}
impl<I, S, F, E> Graceful<I, S, F, E> {}
impl<I, IO, IE, S, B, F, E> Future for Graceful<I, S, F, E>
where
    I: Accept<Conn = IO, Error = IE>,
    IE: Into<Box<dyn StdError + Send + Sync>>,
    IO: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: MakeServiceRef<IO, Body, ResBody = B>,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    B: HttpBody + 'static,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
    F: Future<Output = ()>,
    E: ConnStreamExec<<S::Service as HttpService<Body>>::Future, B>,
    E: NewSvcExec<IO, S::Future, S::Service, E, GracefulWatcher>,
{
    type Output = crate::Result<()>;
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct GracefulWatcher(Watch);
impl<I, S, E> Watcher<I, S, E> for GracefulWatcher
where
    I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: HttpService<Body>,
    E: ConnStreamExec<S::Future, S::ResBody>,
    S::ResBody: 'static,
    <S::ResBody as HttpBody>::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Future = Watching<
        UpgradeableConnection<I, S, E>,
        fn(Pin<&mut UpgradeableConnection<I, S, E>>),
    >;
    fn watch(&self) -> Self::Future {
        loop {}
    }
}
