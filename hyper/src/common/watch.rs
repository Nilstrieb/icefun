//! An SPSC broadcast channel.
//!
//! - The value can only be a `usize`.
//! - The consumer is only notified if the value is different.
//! - The value `0` is reserved for closed.
use futures_util::task::AtomicWaker;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::task;
type Value = usize;
pub(crate) const CLOSED: usize = 0;
pub(crate) fn channel(initial: Value) -> (Sender, Receiver) {
    loop {}
}
pub(crate) struct Sender {
    shared: Arc<Shared>,
}
pub(crate) struct Receiver {
    shared: Arc<Shared>,
}
struct Shared {
    value: AtomicUsize,
    waker: AtomicWaker,
}
impl Sender {
    pub(crate) fn send(&mut self, value: Value) {
        loop {}
    }
}
impl Drop for Sender {
    fn drop(&mut self) {
        loop {}
    }
}
impl Receiver {
    pub(crate) fn load(&mut self, cx: &mut task::Context<'_>) -> Value {
        loop {}
    }
    pub(crate) fn peek(&self) -> Value {
        loop {}
    }
}
