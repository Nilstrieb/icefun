use self::new_svc::NewSvcTask;
use super::accept::Accept;
use super::conn::Http as Http_;
#[cfg(all(feature = "tcp"))]
use super::tcp::AddrIncoming;
use crate::body::{Body, HttpBody};
use crate::common::exec::Exec;
use crate::common::exec::{ConnStreamExec, NewSvcExec};
use crate::common::{task, Future, Pin, Poll, Unpin};
use crate::service::{HttpService, MakeServiceRef};
use pin_project_lite::pin_project;
use std::error::Error as StdError;
#[cfg(feature = "tcp")]
use tokio::io::{AsyncRead, AsyncWrite};
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
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
impl<I> Server<I, ()> {
    pub fn builder(incoming: I) -> Builder<I> {
        loop {}
    }
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

#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
impl<I, IO, IE, S, E> Future for Server<I, S, E>
where
    I: Accept<Conn = IO, Error = IE>,
    IE: Into<Box<dyn StdError + Send + Sync>>,
    S: MakeServiceRef<IO, Body>,
    E: NewSvcExec<IO, S::Future, S::Service, E, NoopWatcher>,
{
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {
            let fut = NewSvcTask::new(NoopWatcher);
            unsafe {
                self.as_mut()
                    .get_unchecked_mut()
                    .protocol
                    .execute_new_svc(fut);
            }
        }
    }
}

impl<I, E> Builder<I, E> {
    #[doc(hidden)]
    #[cfg(feature = "http1")]
    pub fn http1_pipeline_flush(self, val: bool) -> Self {
        loop {}
    }

    pub fn serve<S, B>(self, _: S) -> Server<I, S>
    where
        I: Accept,
        S: MakeServiceRef<I::Conn, Body, ResBody = B>,
    {
        loop {}
    }
}
pub trait Watcher<I, S, E>: Clone {
    type Future;
    fn watch(&self) -> Self::Future;
}
#[allow(missing_debug_implementations)]
#[derive(Copy, Clone)]
pub(crate) struct NoopWatcher;
impl<I, S, E> Watcher<I, S, E> for NoopWatcher
where
    S: HttpService<Body>,
{
    type Future = ();
    fn watch(&self) -> Self::Future {
        loop {}
    }
}
pub(crate) mod new_svc {
    use super::Watcher;
    use crate::body::{Body, HttpBody};
    use crate::common::exec::ConnStreamExec;
    use crate::common::{task, Future, Pin, Poll, Unpin};
    use crate::service::HttpService;
    use std::error::Error as StdError;
    use tokio::io::{AsyncRead, AsyncWrite};

    pub struct NewSvcTask<I, N, S, E, W: Watcher<I, S, E>> {
        state: State<I, S, E, W>,
        a: N,
    }

    pub(super) enum State<I, S, E, W: Watcher<I, S, E>> {
        Connecting { a: (I, S, W, E) },

        Connected { future: W::Future },
    }

    impl<I, N, S: HttpService<Body>, E, W: Watcher<I, S, E>> NewSvcTask<I, N, S, E, W> {
        pub(super) fn new(watcher: W) -> Self {
            loop {}
        }
    }
    impl<I, N, S, NE, B, E, W> Future for NewSvcTask<I, N, S, E, W>
    where
        I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
        N: Future<Output = Result<S, NE>>,
        NE: Into<Box<dyn StdError + Send + Sync>>,
        S: HttpService<Body, ResBody = B>,
        B: HttpBody + 'static,
        B::Error: Into<Box<dyn StdError + Send + Sync>>,
        E: ConnStreamExec<S::Future, B>,
        W: Watcher<I, S, E>,
    {
        type Output = ();
        fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
            loop {}
        }
    }
}
pin_project! {
    #[doc = " A future building a new `Service` to a `Connection`."] #[doc = ""] #[doc =
    " Wraps the future returned from `MakeService` into one that returns"] #[doc =
    " a `Connection`."] #[must_use = "futures do nothing unless polled"] #[derive(Debug)]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]

    pub struct Connecting < F, E = Exec > {

        #[pin] future : F,
        protocol :
        Http_ < E >,

    }
}
