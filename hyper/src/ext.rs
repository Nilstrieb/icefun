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
#[cfg(any(feature = "http1", feature = "ffi"))]
pub(crate) use h1_reason_phrase::ReasonPhrase;
#[cfg(feature = "http2")]
/// Represents the `:protocol` pseudo-header used by
/// the [Extended CONNECT Protocol].
///
/// [Extended CONNECT Protocol]: https://datatracker.ietf.org/doc/html/rfc8441#section-4
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Protocol {
    inner: h2::ext::Protocol,
}
#[cfg(feature = "http2")]
impl Protocol {
    /// Converts a static string to a protocol name.
    pub(crate) const fn from_static(value: &'static str) -> Self {
        loop {}
    }
    /// Returns a str representation of the header.
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
/// A map from header names to their original casing as received in an HTTP message.
///
/// If an HTTP/1 response `res` is parsed on a connection whose option
/// [`http1_preserve_header_case`] was set to true and the response included
/// the following headers:
///
/// ```ignore
/// x-Bread: Baguette
/// X-BREAD: Pain
/// x-bread: Ficelle
/// ```
///
/// Then `res.extensions().get::<HeaderCaseMap>()` will return a map with:
///
/// ```ignore
/// HeaderCaseMap({
///     "x-bread": ["x-Bread", "X-BREAD", "x-bread"],
/// })
/// ```
///
/// [`http1_preserve_header_case`]: /client/struct.Client.html#method.http1_preserve_header_case
#[derive(Clone, Debug)]
pub(crate) struct HeaderCaseMap(HeaderMap<Bytes>);
#[cfg(feature = "http1")]
impl HeaderCaseMap {
    /// Returns a view of all spellings associated with that header name,
    /// in the order they were found.
    pub(crate) fn get_all<'a>(
        &'a self,
        name: &HeaderName,
    ) -> impl Iterator<Item = impl AsRef<[u8]> + 'a> + 'a {
        self.get_all_internal(name).into_iter()
    }
    /// Returns a view of all spellings associated with that header name,
    /// in the order they were found.
    pub(crate) fn get_all_internal<'a>(
        &'a self,
        name: &HeaderName,
    ) -> ValueIter<'_, Bytes> {
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
/// Hashmap<Headername, numheaders with that name>
pub(crate) struct OriginalHeaderOrder {
    /// Stores how many entries a Headername maps to. This is used
    /// for accounting.
    num_entries: HashMap<HeaderName, usize>,
    /// Stores the ordering of the headers. ex: `vec[i] = (headerName, idx)`,
    /// The vector is ordered such that the ith element
    /// represents the ith header that came in off the line.
    /// The `HeaderName` and `idx` are then used elsewhere to index into
    /// the multi map that stores the header values.
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
    /// This returns an iterator that provides header names and indexes
    /// in the original order received.
    ///
    /// # Examples
    /// ```no_run
    /// use hyper::ext::OriginalHeaderOrder;
    /// use hyper::header::{HeaderName, HeaderValue, HeaderMap};
    ///
    /// let mut h_order = OriginalHeaderOrder::default();
    /// let mut h_map = Headermap::new();
    ///
    /// let name1 = b"Set-CookiE";
    /// let value1 = b"a=b";
    /// h_map.append(name1);
    /// h_order.append(name1);
    ///
    /// let name2 = b"Content-Encoding";
    /// let value2 = b"gzip";
    /// h_map.append(name2, value2);
    /// h_order.append(name2);
    ///
    /// let name3 = b"SET-COOKIE";
    /// let value3 = b"c=d";
    /// h_map.append(name3, value3);
    /// h_order.append(name3)
    ///
    /// let mut iter = h_order.get_in_order()
    ///
    /// let (name, idx) = iter.next();
    /// assert_eq!(b"a=b", h_map.get_all(name).nth(idx).unwrap());
    ///
    /// let (name, idx) = iter.next();
    /// assert_eq!(b"gzip", h_map.get_all(name).nth(idx).unwrap());
    ///
    /// let (name, idx) = iter.next();
    /// assert_eq!(b"c=d", h_map.get_all(name).nth(idx).unwrap());
    /// ```
    pub(crate) fn get_in_order(&self) -> impl Iterator<Item = &(HeaderName, usize)> {
        loop {}
    }
}
