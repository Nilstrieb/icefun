use self::new_svc::NewSvcTask;
use super::accept::Accept;
use super::conn::Http as Http_;
#[cfg(all(feature = "tcp"))]
use super::tcp::AddrIncoming;
use crate::body::Body;
use crate::common::exec::Exec;
use crate::common::exec::NewSvcExec;
use crate::common::{task, Future, Pin, Poll};
use crate::service::{HttpService, MakeServiceRef};
use std::error::Error as StdError;
#[cfg(feature = "tcp")]

pub struct Server<I, S, E = Exec> {
    incoming: I,
    make_service: S,
    protocol: E,
}
#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
pub struct Builder<I, E = Exec> {
    incoming: I,
    protocol: E,
}

#[cfg(feature = "tcp")]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "tcp", any(feature = "http1", feature = "http2"))))
)]
impl Server<AddrIncoming, ()> {
    pub fn bind() -> Builder<AddrIncoming> {
        loop {}
    }
}

fn mk<T>() -> T {
    loop {}
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
impl<I, IO, IE, S, E> Future for Server<I, S, E>
where
    I: Accept<Conn = IO, Error = IE>,
    IE: Into<Box<dyn StdError + Send + Sync>>,
    S: MakeServiceRef<IO, Body>,
    E: NewSvcExec<IO, S::Service, E, NoopWatcher>,
{
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {
            let _a: NewSvcTask<IO, <S as MakeServiceRef<IO, Body>>::Service, E, NoopWatcher> = mk();
        }
    }
}

impl<I, E> Builder<I, E> {
    pub fn serve<S, B>(self, _: S) -> Server<I, S>
    where
        I: Accept,
        S: MakeServiceRef<I::Conn, Body, ResBody = B>,
    {
        loop {}
    }
}
pub trait Watcher<I, S> {
    type Future;
}

pub(crate) struct NoopWatcher;
impl<I, S> Watcher<I, S> for NoopWatcher
where
    S: HttpService<Body>,
{
    type Future = ();
}

pub(crate) mod new_svc {
    use super::Watcher;
    use crate::body::Body;
    use crate::common::exec::ConnStreamExec;
    use crate::common::{task, Future, Pin, Poll};
    use crate::service::HttpService;

    pub struct NewSvcTask<I, S, E, W: Watcher<I, S>> {
        state: State<I, S, E, W>,
    }

    pub(super) struct State<I, S, E, W: Watcher<I, S>> {
        a: (I, S, E),
        future: W::Future,
    }

    impl<I, S, B, E, W> Future for NewSvcTask<I, S, E, W>
    where
        S: HttpService<Body, ResBody = B>,
        E: ConnStreamExec<S::Future, B>,
        W: Watcher<I, S>,
    {
        type Output = ();
        fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
            loop {}
        }
    }
}

pub struct Connecting<F, E = Exec> {
    future: F,
    protocol: Http_<E>,
}
