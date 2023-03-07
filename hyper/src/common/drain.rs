use std::mem;
use pin_project_lite::pin_project;
use tokio::sync::watch;
use super::{task, Future, Pin, Poll};
pub(crate) fn channel() -> (Signal, Watch) {
    loop {}
}
pub(crate) struct Signal {
    tx: watch::Sender<()>,
}
pub(crate) struct Draining(Pin<Box<dyn Future<Output = ()> + Send + Sync>>);
#[derive(Clone)]
pub(crate) struct Watch {
    rx: watch::Receiver<()>,
}
pin_project! {
    #[allow(missing_debug_implementations)] pub struct Watching < F, FN > { #[pin] future
    : F, state : State < FN >, watch : Pin < Box < dyn Future < Output = () > + Send +
    Sync >>, _rx : watch::Receiver < () >, }
}
enum State<F> {
    Watch(F),
    Draining,
}
impl Signal {
    pub(crate) fn drain(self) -> Draining {
        loop {}
    }
}
impl Future for Draining {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
impl Watch {
    pub(crate) fn watch<F, FN>(self, future: F, on_drain: FN) -> Watching<F, FN>
    where
        F: Future,
        FN: FnOnce(Pin<&mut F>),
    {
        loop {}
    }
}
impl<F, FN> Future for Watching<F, FN>
where
    F: Future,
    FN: FnOnce(Pin<&mut F>),
{
    type Output = F::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    struct TestMe {
        draining: bool,
        finished: bool,
        poll_cnt: usize,
    }
    impl Future for TestMe {
        type Output = ();
        fn poll(
            mut self: Pin<&mut Self>,
            _: &mut task::Context<'_>,
        ) -> Poll<Self::Output> {
            loop {}
        }
    }
    #[test]
    fn watch() {
        loop {}
    }
    #[test]
    fn watch_clones() {
        loop {}
    }
}
