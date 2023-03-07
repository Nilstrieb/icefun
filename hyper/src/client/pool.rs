use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error as StdError;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, Weak};
#[cfg(not(feature = "runtime"))]
use std::time::{Duration, Instant};
use futures_channel::oneshot;
#[cfg(feature = "runtime")]
use tokio::time::{Duration, Instant, Interval};

use super::client::Ver;
use crate::common::{exec::Exec, task, Future, Pin, Poll, Unpin};
#[allow(missing_debug_implementations)]
pub(super) struct Pool<T> {
    inner: Option<Arc<Mutex<PoolInner<T>>>>,
}
pub(super) trait Poolable: Unpin + Send + Sized + 'static {
    fn is_open(&self) -> bool;
    
    
    
    fn reserve(self) -> Reservation<Self>;
    fn can_share(&self) -> bool;
}





#[allow(missing_debug_implementations)]
pub(super) enum Reservation<T> {
    
    
    
    #[cfg(feature = "http2")]
    Shared(T, T),
    
    
    Unique(T),
}

pub(super) type Key = (http::uri::Scheme, http::uri::Authority);
struct PoolInner<T> {
    connecting: HashSet<Key>,
    idle: HashMap<Key, Vec<Idle<T>>>,
    max_idle_per_host: usize,
    waiters: HashMap<Key, VecDeque<oneshot::Sender<T>>>,
    #[cfg(feature = "runtime")]
    idle_interval_ref: Option<oneshot::Sender<crate::common::Never>>,
    #[cfg(feature = "runtime")]
    exec: Exec,
    timeout: Option<Duration>,
}
struct WeakOpt<T>(Option<Weak<T>>);
#[derive(Clone, Copy, Debug)]
pub(super) struct Config {
    pub(super) idle_timeout: Option<Duration>,
    pub(super) max_idle_per_host: usize,
}
impl Config {
    pub(super) fn is_enabled(&self) -> bool {
        loop {}
    }
}
impl<T> Pool<T> {
    pub(super) fn new(config: Config, __exec: &Exec) -> Pool<T> {
        loop {}
    }
    fn is_enabled(&self) -> bool {
        loop {}
    }
    #[cfg(test)]
    pub(super) fn no_timer(&self) {
        loop {}
    }
}
impl<T: Poolable> Pool<T> {
    
    
    pub(super) fn checkout(&self, key: Key) -> Checkout<T> {
        loop {}
    }
    
    
    pub(super) fn connecting(&self, key: &Key, ver: Ver) -> Option<Connecting<T>> {
        loop {}
    }
    #[cfg(test)]
    fn locked(&self) -> std::sync::MutexGuard<'_, PoolInner<T>> {
        loop {}
    }
    pub(super) fn pooled(
        &self,
        #[cfg_attr(not(feature = "http2"), allow(unused_mut))]
        mut connecting: Connecting<T>,
        value: T,
    ) -> Pooled<T> {
        loop {}
    }
    fn reuse(&self, key: &Key, value: T) -> Pooled<T> {
        loop {}
    }
}

struct IdlePopper<'a, T> {
    key: &'a Key,
    list: &'a mut Vec<Idle<T>>,
}
impl<'a, T: Poolable + 'a> IdlePopper<'a, T> {
    fn pop(self, expiration: &Expiration) -> Option<Idle<T>> {
        loop {}
    }
}
impl<T: Poolable> PoolInner<T> {
    fn put(&mut self, key: Key, value: T, __pool_ref: &Arc<Mutex<PoolInner<T>>>) {
        loop {}
    }
    
    
    fn connected(&mut self, key: &Key) {
        loop {}
    }
    #[cfg(feature = "runtime")]
    fn spawn_idle_interval(&mut self, pool_ref: &Arc<Mutex<PoolInner<T>>>) {
        loop {}
    }
}
impl<T> PoolInner<T> {
    
    
    
    
    fn clean_waiters(&mut self, key: &Key) {
        loop {}
    }
}
#[cfg(feature = "runtime")]
impl<T: Poolable> PoolInner<T> {
    
    fn clear_expired(&mut self) {
        loop {}
    }
}
impl<T> Clone for Pool<T> {
    fn clone(&self) -> Pool<T> {
        loop {}
    }
}

pub(super) struct Pooled<T: Poolable> {
    value: Option<T>,
    is_reused: bool,
    key: Key,
    pool: WeakOpt<Mutex<PoolInner<T>>>,
}
impl<T: Poolable> Pooled<T> {
    pub(super) fn is_reused(&self) -> bool {
        loop {}
    }
    pub(super) fn is_pool_enabled(&self) -> bool {
        loop {}
    }
    fn as_ref(&self) -> &T {
        loop {}
    }
    fn as_mut(&mut self) -> &mut T {
        loop {}
    }
}
impl<T: Poolable> Deref for Pooled<T> {
    type Target = T;
    fn deref(&self) -> &T {
        loop {}
    }
}
impl<T: Poolable> DerefMut for Pooled<T> {
    fn deref_mut(&mut self) -> &mut T {
        loop {}
    }
}
impl<T: Poolable> Drop for Pooled<T> {
    fn drop(&mut self) {
        loop {}
    }
}
impl<T: Poolable> fmt::Debug for Pooled<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
struct Idle<T> {
    idle_at: Instant,
    value: T,
}
#[allow(missing_debug_implementations)]
pub(super) struct Checkout<T> {
    key: Key,
    pool: Pool<T>,
    waiter: Option<oneshot::Receiver<T>>,
}
#[derive(Debug)]
pub(super) struct CheckoutIsClosedError;
impl StdError for CheckoutIsClosedError {}
impl fmt::Display for CheckoutIsClosedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<T: Poolable> Checkout<T> {
    fn poll_waiter(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<crate::Result<Pooled<T>>>> {
        loop {}
    }
    fn checkout(&mut self, cx: &mut task::Context<'_>) -> Option<Pooled<T>> {
        loop {}
    }
}
impl<T: Poolable> Future for Checkout<T> {
    type Output = crate::Result<Pooled<T>>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
impl<T> Drop for Checkout<T> {
    fn drop(&mut self) {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
pub(super) struct Connecting<T: Poolable> {
    key: Key,
    pool: WeakOpt<Mutex<PoolInner<T>>>,
}
impl<T: Poolable> Connecting<T> {
    pub(super) fn alpn_h2(self, pool: &Pool<T>) -> Option<Self> {
        loop {}
    }
}
impl<T: Poolable> Drop for Connecting<T> {
    fn drop(&mut self) {
        loop {}
    }
}
struct Expiration(Option<Duration>);
impl Expiration {
    fn new(dur: Option<Duration>) -> Expiration {
        loop {}
    }
    fn expires(&self, instant: Instant) -> bool {
        loop {}
    }
}
#[cfg(feature = "runtime")]
pin_project_lite::pin_project! {
    struct IdleTask < T > { #[pin] interval : Interval, pool : WeakOpt < Mutex <
    PoolInner < T >>>, #[pin] pool_drop_notifier : oneshot::Receiver < crate
    ::common::Never >, }
}
#[cfg(feature = "runtime")]
impl<T: Poolable + 'static> Future for IdleTask<T> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
impl<T> WeakOpt<T> {
    fn none() -> Self {
        loop {}
    }
    fn downgrade(arc: &Arc<T>) -> Self {
        loop {}
    }
    fn upgrade(&self) -> Option<Arc<T>> {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use std::task::Poll;
    use std::time::Duration;
    use super::{Connecting, Key, Pool, Poolable, Reservation, WeakOpt};
    use crate::common::{exec::Exec, task, Future, Pin};
    
    #[derive(Debug, PartialEq, Eq)]
    struct Uniq<T>(T);
    impl<T: Send + 'static + Unpin> Poolable for Uniq<T> {
        fn is_open(&self) -> bool {
            loop {}
        }
        fn reserve(self) -> Reservation<Self> {
            loop {}
        }
        fn can_share(&self) -> bool {
            loop {}
        }
    }
    fn c<T: Poolable>(key: Key) -> Connecting<T> {
        loop {}
    }
    fn host_key(s: &str) -> Key {
        loop {}
    }
    fn pool_no_timer<T>() -> Pool<T> {
        loop {}
    }
    fn pool_max_idle_no_timer<T>(max_idle: usize) -> Pool<T> {
        loop {}
    }
    #[tokio::test]
    async fn test_pool_checkout_smoke() {
        loop {}
    }
    
    struct PollOnce<'a, F>(&'a mut F);
    impl<F, T, U> Future for PollOnce<'_, F>
    where
        F: Future<Output = Result<T, U>> + Unpin,
    {
        type Output = Option<()>;
        fn poll(
            mut self: Pin<&mut Self>,
            cx: &mut task::Context<'_>,
        ) -> Poll<Self::Output> {
            loop {}
        }
    }
    #[tokio::test]
    async fn test_pool_checkout_returns_none_if_expired() {
        loop {}
    }
    #[cfg(feature = "runtime")]
    #[tokio::test]
    async fn test_pool_checkout_removes_expired() {
        loop {}
    }
    #[test]
    fn test_pool_max_idle_per_host() {
        loop {}
    }
    #[cfg(feature = "runtime")]
    #[tokio::test]
    async fn test_pool_timer_removes_expired() {
        loop {}
    }
    #[tokio::test]
    async fn test_pool_checkout_task_unparked() {
        loop {}
    }
    #[tokio::test]
    async fn test_pool_checkout_drop_cleans_up_waiters() {
        loop {}
    }
    #[derive(Debug)]
    struct CanClose {
        #[allow(unused)]
        val: i32,
        closed: bool,
    }
    impl Poolable for CanClose {
        fn is_open(&self) -> bool {
            loop {}
        }
        fn reserve(self) -> Reservation<Self> {
            loop {}
        }
        fn can_share(&self) -> bool {
            loop {}
        }
    }
    #[test]
    fn pooled_drop_if_closed_doesnt_reinsert() {
        loop {}
    }
}
