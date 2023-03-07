use scoped_tls::scoped_thread_local;
use std::cell::RefCell;

use std::net::SocketAddr;
use hyper::Body;
use crate::Request;
scoped_thread_local!(static ROUTE : RefCell < Route >);
pub(crate) fn set<F, U>(r: &RefCell<Route>, func: F) -> U
where
    F: FnOnce() -> U,
{
    loop {}
}
pub(crate) fn is_set() -> bool {
    loop {}
}
pub(crate) fn with<F, R>(func: F) -> R
where
    F: FnOnce(&mut Route) -> R,
{
    loop {}
}
#[derive(Debug)]
pub(crate) struct Route {
    body: BodyState,
    remote_addr: Option<SocketAddr>,
    req: Request,
    segments_index: usize,
}
#[derive(Debug)]
enum BodyState {
    Ready,
    Taken,
}
impl Route {
    pub(crate) fn new(req: Request, remote_addr: Option<SocketAddr>) -> RefCell<Route> {
        loop {}
    }
    pub(crate) fn method(&self) -> &http::Method {
        loop {}
    }
    pub(crate) fn headers(&self) -> &http::HeaderMap {
        loop {}
    }
    pub(crate) fn version(&self) -> http::Version {
        loop {}
    }
    pub(crate) fn extensions(&self) -> &http::Extensions {
        loop {}
    }
    #[cfg(feature = "websocket")]
    pub(crate) fn extensions_mut(&mut self) -> &mut http::Extensions {
        loop {}
    }
    pub(crate) fn uri(&self) -> &http::Uri {
        loop {}
    }
    pub(crate) fn path(&self) -> &str {
        loop {}
    }
    pub(crate) fn full_path(&self) -> &str {
        loop {}
    }
    pub(crate) fn set_unmatched_path(&mut self, index: usize) {
        loop {}
    }
    pub(crate) fn query(&self) -> Option<&str> {
        loop {}
    }
    pub(crate) fn matched_path_index(&self) -> usize {
        loop {}
    }
    pub(crate) fn reset_matched_path_index(&mut self, index: usize) {
        loop {}
    }
    pub(crate) fn remote_addr(&self) -> Option<SocketAddr> {
        loop {}
    }
    pub(crate) fn take_body(&mut self) -> Option<Body> {
        loop {}
    }
}
