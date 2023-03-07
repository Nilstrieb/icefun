#[cfg(feature = "http2")]
use std::future::Future;

use tokio::sync::{mpsc, oneshot};

use crate::common::{task, Poll};
pub(crate) type RetryPromise<T, U> = oneshot::Receiver<
    Result<U, (crate::Error, Option<T>)>,
>;
pub(crate) type Promise<T> = oneshot::Receiver<Result<T, crate::Error>>;
pub(crate) fn channel<T, U>() -> (Sender<T, U>, Receiver<T, U>) {
    loop {}
}
/// A bounded sender of requests and callbacks for when responses are ready.
///
/// While the inner sender is unbounded, the Giver is used to determine
/// if the Receiver is ready for another request.
pub(crate) struct Sender<T, U> {
    /// One message is always allowed, even if the Receiver hasn't asked
    /// for it yet. This boolean keeps track of whether we've sent one
    /// without notice.
    buffered_once: bool,
    /// The Giver helps watch that the the Receiver side has been polled
    /// when the queue is empty. This helps us know when a request and
    /// response have been fully processed, and a connection is ready
    /// for more.
    giver: want::Giver,
    /// Actually bounded by the Giver, plus `buffered_once`.
    inner: mpsc::UnboundedSender<Envelope<T, U>>,
}
/// An unbounded version.
///
/// Cannot poll the Giver, but can still use it to determine if the Receiver
/// has been dropped. However, this version can be cloned.
#[cfg(feature = "http2")]
pub(crate) struct UnboundedSender<T, U> {
    /// Only used for `is_closed`, since mpsc::UnboundedSender cannot be checked.
    giver: want::SharedGiver,
    inner: mpsc::UnboundedSender<Envelope<T, U>>,
}
impl<T, U> Sender<T, U> {
    pub(crate) fn poll_ready(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<crate::Result<()>> {
        loop {}
    }
    pub(crate) fn is_ready(&self) -> bool {
        loop {}
    }
    pub(crate) fn is_closed(&self) -> bool {
        loop {}
    }
    fn can_send(&mut self) -> bool {
        loop {}
    }
    pub(crate) fn try_send(&mut self, val: T) -> Result<RetryPromise<T, U>, T> {
        loop {}
    }
    pub(crate) fn send(&mut self, val: T) -> Result<Promise<U>, T> {
        loop {}
    }
    #[cfg(feature = "http2")]
    pub(crate) fn unbound(self) -> UnboundedSender<T, U> {
        loop {}
    }
}
#[cfg(feature = "http2")]
impl<T, U> UnboundedSender<T, U> {
    pub(crate) fn is_ready(&self) -> bool {
        loop {}
    }
    pub(crate) fn is_closed(&self) -> bool {
        loop {}
    }
    pub(crate) fn try_send(&mut self, val: T) -> Result<RetryPromise<T, U>, T> {
        loop {}
    }
}
#[cfg(feature = "http2")]
impl<T, U> Clone for UnboundedSender<T, U> {
    fn clone(&self) -> Self {
        loop {}
    }
}
pub(crate) struct Receiver<T, U> {
    inner: mpsc::UnboundedReceiver<Envelope<T, U>>,
    taker: want::Taker,
}
impl<T, U> Receiver<T, U> {
    pub(crate) fn poll_recv(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<(T, Callback<T, U>)>> {
        loop {}
    }
    #[cfg(feature = "http1")]
    pub(crate) fn close(&mut self) {
        loop {}
    }
    #[cfg(feature = "http1")]
    pub(crate) fn try_recv(&mut self) -> Option<(T, Callback<T, U>)> {
        loop {}
    }
}
impl<T, U> Drop for Receiver<T, U> {
    fn drop(&mut self) {
        loop {}
    }
}
struct Envelope<T, U>(Option<(T, Callback<T, U>)>);
impl<T, U> Drop for Envelope<T, U> {
    fn drop(&mut self) {
        loop {}
    }
}
pub(crate) enum Callback<T, U> {
    Retry(Option<oneshot::Sender<Result<U, (crate::Error, Option<T>)>>>),
    NoRetry(Option<oneshot::Sender<Result<U, crate::Error>>>),
}
impl<T, U> Drop for Callback<T, U> {
    fn drop(&mut self) {
        loop {}
    }
}
impl<T, U> Callback<T, U> {
    #[cfg(feature = "http2")]
    pub(crate) fn is_canceled(&self) -> bool {
        loop {}
    }
    pub(crate) fn poll_canceled(&mut self, cx: &mut task::Context<'_>) -> Poll<()> {
        loop {}
    }
    pub(crate) fn send(mut self, val: Result<U, (crate::Error, Option<T>)>) {
        loop {}
    }
    #[cfg(feature = "http2")]
    pub(crate) async fn send_when(
        self,
        mut when: impl Future<Output = Result<U, (crate::Error, Option<T>)>> + Unpin,
    ) {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    #[cfg(feature = "nightly")]
    extern crate test;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use super::{channel, Callback, Receiver};
    #[derive(Debug)]
    struct Custom(i32);
    impl<T, U> Future for Receiver<T, U> {
        type Output = Option<(T, Callback<T, U>)>;
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            loop {}
        }
    }
    /// Helper to check if the future is ready after polling once.
    struct PollOnce<'a, F>(&'a mut F);
    impl<F, T> Future for PollOnce<'_, F>
    where
        F: Future<Output = T> + Unpin,
    {
        type Output = Option<()>;
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            loop {}
        }
    }
    #[tokio::test]
    async fn drop_receiver_sends_cancel_errors() {
        loop {}
    }
    #[tokio::test]
    async fn sender_checks_for_want_on_send() {
        loop {}
    }
    #[cfg(feature = "http2")]
    #[test]
    fn unbounded_sender_doesnt_bound_on_want() {
        loop {}
    }
    #[cfg(feature = "nightly")]
    #[bench]
    fn giver_queue_throughput(b: &mut test::Bencher) {
        loop {}
    }
    #[cfg(feature = "nightly")]
    #[bench]
    fn giver_queue_not_ready(b: &mut test::Bencher) {
        loop {}
    }
    #[cfg(feature = "nightly")]
    #[bench]
    fn giver_queue_cancel(b: &mut test::Bencher) {
        loop {}
    }
}
