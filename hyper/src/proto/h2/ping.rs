



















#[cfg(feature = "runtime")]
use std::fmt;
#[cfg(feature = "runtime")]
use std::future::Future;
#[cfg(feature = "runtime")]
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{self, Poll};
use std::time::Duration;
#[cfg(not(feature = "runtime"))]
use std::time::Instant;
use h2::{PingPong};
#[cfg(feature = "runtime")]
use tokio::time::{Instant, Sleep};

type WindowSize = u32;
pub(super) fn disabled() -> Recorder {
    loop {}
}
pub(super) fn channel(ping_pong: PingPong, config: Config) -> (Recorder, Ponger) {
    loop {}
}
#[derive(Clone)]
pub(super) struct Config {
    pub(super) bdp_initial_window: Option<WindowSize>,
    
    #[cfg(feature = "runtime")]
    pub(super) keep_alive_interval: Option<Duration>,
    
    
    #[cfg(feature = "runtime")]
    pub(super) keep_alive_timeout: Duration,
    
    #[cfg(feature = "runtime")]
    pub(super) keep_alive_while_idle: bool,
}
#[derive(Clone)]
pub(crate) struct Recorder {
    shared: Option<Arc<Mutex<Shared>>>,
}
pub(super) struct Ponger {
    bdp: Option<Bdp>,
    #[cfg(feature = "runtime")]
    keep_alive: Option<KeepAlive>,
    shared: Arc<Mutex<Shared>>,
}
struct Shared {
    ping_pong: PingPong,
    ping_sent_at: Option<Instant>,
    
    
    bytes: Option<usize>,
    
    
    next_bdp_at: Option<Instant>,
    
    
    #[cfg(feature = "runtime")]
    last_read_at: Option<Instant>,
    #[cfg(feature = "runtime")]
    is_keep_alive_timed_out: bool,
}
struct Bdp {
    
    bdp: u32,
    
    max_bandwidth: f64,
    
    rtt: f64,
    
    
    
    ping_delay: Duration,
    
    stable_count: u32,
}
#[cfg(feature = "runtime")]
struct KeepAlive {
    
    interval: Duration,
    
    
    timeout: Duration,
    
    while_idle: bool,
    state: KeepAliveState,
    timer: Pin<Box<Sleep>>,
}
#[cfg(feature = "runtime")]
enum KeepAliveState {
    Init,
    Scheduled,
    PingSent,
}
pub(super) enum Ponged {
    SizeUpdate(WindowSize),
    #[cfg(feature = "runtime")]
    KeepAliveTimedOut,
}
#[cfg(feature = "runtime")]
#[derive(Debug)]
pub(super) struct KeepAliveTimedOut;
impl Config {
    pub(super) fn is_enabled(&self) -> bool {
        loop {}
    }
}
impl Recorder {
    pub(crate) fn record_data(&self, len: usize) {
        loop {}
    }
    pub(crate) fn record_non_data(&self) {
        loop {}
    }
    
    
    #[cfg(feature = "client")]
    pub(super) fn for_stream(self, stream: &h2::RecvStream) -> Self {
        loop {}
    }
    pub(super) fn ensure_not_timed_out(&self) -> crate::Result<()> {
        loop {}
    }
}
impl Ponger {
    pub(super) fn poll(&mut self, cx: &mut task::Context<'_>) -> Poll<Ponged> {
        loop {}
    }
    #[cfg(feature = "runtime")]
    fn is_idle(&self) -> bool {
        loop {}
    }
}
impl Shared {
    fn send_ping(&mut self) {
        loop {}
    }
    fn is_ping_sent(&self) -> bool {
        loop {}
    }
    #[cfg(feature = "runtime")]
    fn update_last_read_at(&mut self) {
        loop {}
    }
    #[cfg(feature = "runtime")]
    fn last_read_at(&self) -> Instant {
        loop {}
    }
}

const BDP_LIMIT: usize = 1024 * 1024 * 16;
impl Bdp {
    fn calculate(&mut self, bytes: usize, rtt: Duration) -> Option<WindowSize> {
        loop {}
    }
    fn stabilize_delay(&mut self) {
        loop {}
    }
}
fn seconds(dur: Duration) -> f64 {
    loop {}
}
#[cfg(feature = "runtime")]
impl KeepAlive {
    fn schedule(&mut self, is_idle: bool, shared: &Shared) {
        loop {}
    }
    fn maybe_ping(&mut self, cx: &mut task::Context<'_>, shared: &mut Shared) {
        loop {}
    }
    fn maybe_timeout(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Result<(), KeepAliveTimedOut> {
        loop {}
    }
}
#[cfg(feature = "runtime")]
impl KeepAliveTimedOut {
    pub(super) fn crate_error(self) -> crate::Error {
        loop {}
    }
}
#[cfg(feature = "runtime")]
impl fmt::Display for KeepAliveTimedOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
#[cfg(feature = "runtime")]
impl std::error::Error for KeepAliveTimedOut {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        loop {}
    }
}
