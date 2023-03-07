//! DNS Resolution used by the `HttpConnector`.
//!
//! This module contains:
//!
//! - A [`GaiResolver`](GaiResolver) that is the default resolver for the
//!   `HttpConnector`.
//! - The `Name` type used as an argument to custom resolvers.
//!
//! # Resolvers are `Service`s
//!
//! A resolver is just a
//! `Service<Name, Response = impl Iterator<Item = SocketAddr>>`.
//!
//! A simple resolver that ignores the name and always returns a specific
//! address:
//!
//! ```rust,ignore
//! use std::{convert::Infallible, iter, net::SocketAddr};
//!
//! let resolver = tower::service_fn(|_name| async {
//!     Ok::<_, Infallible>(iter::once(SocketAddr::from(([127, 0, 0, 1], 8080))))
//! });
//! ```
use std::error::Error;
use std::future::Future;
use std::net::{
    Ipv4Addr, Ipv6Addr, SocketAddr,
};
use std::pin::Pin;
use std::str::FromStr;
use std::task::{self, Poll};
use std::{fmt, io, vec};
use tokio::task::JoinHandle;
use tower_service::Service;

pub(super) use self::sealed::Resolve;
/// A domain name to resolve into IP addresses.
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Name {
    host: Box<str>,
}
/// A resolver using blocking `getaddrinfo` calls in a threadpool.
#[derive(Clone)]
pub struct GaiResolver {
    _priv: (),
}
/// An iterator of IP addresses returned from `getaddrinfo`.
pub struct GaiAddrs {
    inner: SocketAddrs,
}
/// A future to resolve a name returned by `GaiResolver`.
pub struct GaiFuture {
    inner: JoinHandle<Result<SocketAddrs, io::Error>>,
}
impl Name {
    pub(super) fn new(host: Box<str>) -> Name {
        loop {}
    }
    /// View the hostname as a string slice.
    pub(crate) fn as_str(&self) -> &str {
        loop {}
    }
}
impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl FromStr for Name {
    type Err = InvalidNameError;
    fn from_str(host: &str) -> Result<Self, Self::Err> {
        loop {}
    }
}
/// Error indicating a given string was not a valid domain name.
#[derive(Debug)]
pub struct InvalidNameError(());
impl fmt::Display for InvalidNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Error for InvalidNameError {}
impl GaiResolver {
    /// Construct a new `GaiResolver`.
    pub(crate) fn new() -> Self {
        loop {}
    }
}
impl Service<Name> for GaiResolver {
    type Response = GaiAddrs;
    type Error = io::Error;
    type Future = GaiFuture;
    fn poll_ready(
        &mut self,
        _cx: &mut task::Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        loop {}
    }
    fn call(&mut self, name: Name) -> Self::Future {
        loop {}
    }
}
impl fmt::Debug for GaiResolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Future for GaiFuture {
    type Output = Result<GaiAddrs, io::Error>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
impl fmt::Debug for GaiFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Drop for GaiFuture {
    fn drop(&mut self) {
        loop {}
    }
}
impl Iterator for GaiAddrs {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<Self::Item> {
        loop {}
    }
}
impl fmt::Debug for GaiAddrs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
pub(super) struct SocketAddrs {
    iter: vec::IntoIter<SocketAddr>,
}
impl SocketAddrs {
    pub(super) fn new(addrs: Vec<SocketAddr>) -> Self {
        loop {}
    }
    pub(super) fn try_parse(host: &str, port: u16) -> Option<SocketAddrs> {
        loop {}
    }
    #[inline]
    fn filter(self, predicate: impl FnMut(&SocketAddr) -> bool) -> SocketAddrs {
        loop {}
    }
    pub(super) fn split_by_preference(
        self,
        local_addr_ipv4: Option<Ipv4Addr>,
        local_addr_ipv6: Option<Ipv6Addr>,
    ) -> (SocketAddrs, SocketAddrs) {
        loop {}
    }
    pub(super) fn is_empty(&self) -> bool {
        loop {}
    }
    pub(super) fn len(&self) -> usize {
        loop {}
    }
}
impl Iterator for SocketAddrs {
    type Item = SocketAddr;
    #[inline]
    fn next(&mut self) -> Option<SocketAddr> {
        loop {}
    }
}
mod sealed {
    use super::{SocketAddr, Name};
    use crate::common::{task, Future, Poll};
    use tower_service::Service;
    pub trait Resolve {
        type Addrs: Iterator<Item = SocketAddr>;
        type Error: Into<Box<dyn std::error::Error + Send + Sync>>;
        type Future: Future<Output = Result<Self::Addrs, Self::Error>>;
        fn poll_ready(
            &mut self,
            cx: &mut task::Context<'_>,
        ) -> Poll<Result<(), Self::Error>>;
        fn resolve(&mut self, name: Name) -> Self::Future;
    }
    impl<S> Resolve for S
    where
        S: Service<Name>,
        S::Response: Iterator<Item = SocketAddr>,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        type Addrs = S::Response;
        type Error = S::Error;
        type Future = S::Future;
        fn poll_ready(
            &mut self,
            cx: &mut task::Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            loop {}
        }
        fn resolve(&mut self, name: Name) -> Self::Future {
            loop {}
        }
    }
}
pub(super) async fn resolve<R>(
    resolver: &mut R,
    name: Name,
) -> Result<R::Addrs, R::Error>
where
    R: Resolve,
{
    loop {}
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};
    #[test]
    fn test_ip_addrs_split_by_preference() {
        loop {}
    }
    #[test]
    fn test_name_from_str() {
        loop {}
    }
}
