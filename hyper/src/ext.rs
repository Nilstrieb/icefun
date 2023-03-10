//! HTTP extensions.
use bytes::Bytes;
#[cfg(any(feature = "http1", feature = "ffi"))]
use http::header::HeaderName;
#[cfg(feature = "http1")]
use http::header::{IntoHeaderName, ValueIter};
use http::HeaderMap;
#[cfg(feature = "ffi")]
use std::collections::HashMap;
#[cfg(feature = "http2")]
use std::fmt;
#[cfg(any(feature = "http1", feature = "ffi"))]
mod h1_reason_phrase;

#[cfg(feature = "http2")]
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Protocol {
    inner: h2::ext::Protocol,
}
#[cfg(feature = "http2")]
impl Protocol {
    pub(crate) const fn from_static(value: &'static str) -> Self {
        loop {}
    }

    pub(crate) fn as_str(&self) -> &str {
        loop {}
    }
    #[cfg(feature = "server")]
    pub(crate) fn from_inner(inner: h2::ext::Protocol) -> Self {
        loop {}
    }
    pub(crate) fn into_inner(self) -> h2::ext::Protocol {
        loop {}
    }
}
#[cfg(feature = "http2")]
impl<'a> From<&'a str> for Protocol {
    fn from(value: &'a str) -> Self {
        loop {}
    }
}
#[cfg(feature = "http2")]
impl AsRef<[u8]> for Protocol {
    fn as_ref(&self) -> &[u8] {
        loop {}
    }
}
#[cfg(feature = "http2")]
impl fmt::Debug for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HeaderCaseMap(HeaderMap<Bytes>);
#[cfg(feature = "http1")]
impl HeaderCaseMap {
    pub(crate) fn get_all<'a>(
        &'a self,
        name: &HeaderName,
    ) -> impl Iterator<Item = impl AsRef<[u8]> + 'a> + 'a {
        self.get_all_internal(name).into_iter()
    }

    pub(crate) fn get_all_internal<'a>(&'a self, name: &HeaderName) -> ValueIter<'_, Bytes> {
        loop {}
    }
    pub(crate) fn default() -> Self {
        loop {}
    }
    #[cfg(any(test, feature = "ffi"))]
    pub(crate) fn insert(&mut self, name: HeaderName, orig: Bytes) {
        loop {}
    }
    pub(crate) fn append<N>(&mut self, name: N, orig: Bytes)
    where
        N: IntoHeaderName,
    {
        loop {}
    }
}
#[cfg(feature = "ffi")]
#[derive(Clone, Debug)]

pub(crate) struct OriginalHeaderOrder {
    num_entries: HashMap<HeaderName, usize>,

    entry_order: Vec<(HeaderName, usize)>,
}
#[cfg(all(feature = "http1", feature = "ffi"))]
impl OriginalHeaderOrder {
    pub(crate) fn default() -> Self {
        loop {}
    }
    pub(crate) fn insert(&mut self, name: HeaderName) {
        loop {}
    }
    pub(crate) fn append<N>(&mut self, name: N)
    where
        N: IntoHeaderName + Into<HeaderName> + Clone,
    {
        loop {}
    }

    pub(crate) fn get_in_order(&self) -> impl Iterator<Item = &(HeaderName, usize)> {
        loop {}
    }
}
