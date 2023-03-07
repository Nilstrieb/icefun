use std::error::Error as StdError;
#[cfg(feature = "tcp")]
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, AsyncWrite};
use super::accept::Accept;
#[cfg(all(feature = "tcp"))]
use super::tcp::AddrIncoming;
use crate::body::{Body, HttpBody};
use crate::common::exec::Exec;
use crate::common::exec::{ConnStreamExec, NewSvcExec};
use crate::common::{task, Future, Pin, Poll, Unpin};
use super::conn::{Http as Http_, UpgradeableConnection};
use super::shutdown::{Graceful, GracefulWatcher};
use crate::service::{HttpService, MakeServiceRef};
use self::new_svc::NewSvcTask;
pin_project! {
    #[doc =
    " A listening HTTP server that accepts connections in both HTTP1 and HTTP2 by default."]
    #[doc = ""] #[doc =
    " `Server` is a `Future` mapping a bound listener with a set of service"] #[doc =
    " handlers. It is built using the [`Builder`](Builder), and the future"] #[doc =
    " completes when the server has been shutdown. It should be run by an"] #[doc =
    " `Executor`."] pub struct Server < I, S, E = Exec > { #[pin] incoming : I,
    make_service : S, protocol : Http_ < E >, }
}
/// A builder for a [`Server`](Server).
#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
pub struct Builder<I, E = Exec> {
    incoming: I,
    protocol: Http_<E>,
}
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
impl<I> Server<I, ()> {
    /// Starts a [`Builder`](Builder) with the provided incoming stream.
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
    /// Binds to the provided address, and returns a [`Builder`](Builder).
    ///
    /// # Panics
    ///
    /// This method will panic if binding to the address fails. For a method
    /// to bind to an address and return a `Result`, see `Server::try_bind`.
    pub fn bind(addr: &SocketAddr) -> Builder<AddrIncoming> {
        loop {}
    }
}
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
impl<I, IO, IE, S, E, B> Server<I, S, E>
where
    I: Accept<Conn = IO, Error = IE>,
    IE: Into<Box<dyn StdError + Send + Sync>>,
    IO: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: MakeServiceRef<IO, Body, ResBody = B>,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    B: HttpBody + 'static,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
    E: ConnStreamExec<<S::Service as HttpService<Body>>::Future, B>,
{
    /// Prepares a server to handle graceful shutdown when the provided future
    /// completes.
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() {}
    /// # #[cfg(feature = "tcp")]
    /// # async fn run() {
    /// # use hyper::{Body, Response, Server, Error};
    /// # use hyper::service::{make_service_fn, service_fn};
    /// # let make_service = make_service_fn(|_| async {
    /// #     Ok::<_, Error>(service_fn(|_req| async {
    /// #         Ok::<_, Error>(Response::new(Body::from("Hello World")))
    /// #     }))
    /// # });
    /// // Make a server from the previous examples...
    /// let server = Server::bind(&([127, 0, 0, 1], 3000).into())
    ///     .serve(make_service);
    ///
    /// // Prepare some signal for when the server should start shutting down...
    /// let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    /// let graceful = server
    ///     .with_graceful_shutdown(async {
    ///         rx.await.ok();
    ///     });
    ///
    /// // Await the `server` receiving the signal...
    /// if let Err(e) = graceful.await {
    ///     eprintln!("server error: {}", e);
    /// }
    ///
    /// // And later, trigger the signal by calling `tx.send(())`.
    /// let _ = tx.send(());
    /// # }
    /// ```
    pub fn with_graceful_shutdown<F>(self, signal: F) -> Graceful<I, S, F, E>
    where
        F: Future<Output = ()>,
        E: NewSvcExec<IO, S::Future, S::Service, E, GracefulWatcher>,
    {
        loop {}
    }
    fn poll_next_(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<crate::Result<Connecting<IO, S::Future, E>>>> {
        loop {}
    }
}
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
impl<I, IO, IE, S, B, E> Future for Server<I, S, E>
where
    I: Accept<Conn = IO, Error = IE>,
    IE: Into<Box<dyn StdError + Send + Sync>>,
    IO: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: MakeServiceRef<IO, Body, ResBody = B>,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    B: HttpBody + 'static,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
    E: ConnStreamExec<<S::Service as HttpService<Body>>::Future, B>,
    E: NewSvcExec<IO, S::Future, S::Service, E, NoopWatcher>,
{
    type Output = crate::Result<()>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {
            if let Some(connecting) = ready!(self.as_mut().poll_next_(cx) ?) {
                let fut = NewSvcTask::new(connecting, NoopWatcher);
                self.as_mut().project().protocol.exec.execute_new_svc(fut);
            } else {
                loop {}
            }
        }
    }
}
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
impl<I, E> Builder<I, E> {
    #[doc(hidden)]
    #[cfg(feature = "http1")]
    pub fn http1_pipeline_flush(mut self, val: bool) -> Self {
        loop {}
    }
    /// Set a timeout for reading client request headers. If a client does not
    /// transmit the entire header within this time, the connection is closed.
    ///
    /// Default is None.
    #[cfg(all(feature = "http1", feature = "runtime"))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "http1", feature = "runtime"))))]
    pub(crate) fn http1_header_read_timeout(mut self, read_timeout: Duration) -> Self {
        loop {}
    }
    ///
    pub fn serve<S, B>(self, _: S) -> Server<I, S>
    where
        I: Accept,
        I::Error: Into<Box<dyn StdError + Send + Sync>>,
        S: MakeServiceRef<I::Conn, Body, ResBody = B>,
    {
        loop {}
    }
}
pub trait Watcher<I, S: HttpService<Body>, E>: Clone {
    type Future: Future<Output = crate::Result<()>>;
    fn watch(&self, conn: UpgradeableConnection<I, S, E>) -> Self::Future;
}
#[allow(missing_debug_implementations)]
#[derive(Copy, Clone)]
pub(crate) struct NoopWatcher;
impl<I, S, E> Watcher<I, S, E> for NoopWatcher
where
    I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: HttpService<Body>,
    E: ConnStreamExec<S::Future, S::ResBody>,
    S::ResBody: 'static,
    <S::ResBody as HttpBody>::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Future = UpgradeableConnection<I, S, E>;
    fn watch(&self, conn: UpgradeableConnection<I, S, E>) -> Self::Future {
        loop {}
    }
}
pub(crate) mod new_svc {
    use std::error::Error as StdError;
    use tokio::io::{AsyncRead, AsyncWrite};
    use super::{Connecting, Watcher};
    use crate::body::{Body, HttpBody};
    use crate::common::exec::ConnStreamExec;
    use crate::common::{task, Future, Pin, Poll, Unpin};
    use crate::service::HttpService;
    use pin_project_lite::pin_project;
    pin_project! {
        #[allow(missing_debug_implementations)] pub struct NewSvcTask < I, N, S :
        HttpService < Body >, E, W : Watcher < I, S, E >> { #[pin] state : State < I, N,
        S, E, W >, }
    }
    pin_project! {
        #[project = StateProj] pub (super) enum State < I, N, S : HttpService < Body >,
        E, W : Watcher < I, S, E >> { Connecting { #[pin] connecting : Connecting < I, N,
        E >, watcher : W, }, Connected { #[pin] future : W::Future, }, }
    }
    impl<I, N, S: HttpService<Body>, E, W: Watcher<I, S, E>> NewSvcTask<I, N, S, E, W> {
        pub(super) fn new(connecting: Connecting<I, N, E>, watcher: W) -> Self {
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
    #[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))] pub struct
    Connecting < I, F, E = Exec > { #[pin] future : F, io : Option < I >, protocol :
    Http_ < E >, }
}
