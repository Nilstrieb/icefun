use std::convert::TryFrom;
use bytes::Bytes;
/// A reason phrase in an HTTP/1 response.
///
/// # Clients
///
/// For clients, a `ReasonPhrase` will be present in the extensions of the `http::Response` returned
/// for a request if the reason phrase is different from the canonical reason phrase for the
/// response's status code. For example, if a server returns `HTTP/1.1 200 Awesome`, the
/// `ReasonPhrase` will be present and contain `Awesome`, but if a server returns `HTTP/1.1 200 OK`,
/// the response will not contain a `ReasonPhrase`.
///
/// ```no_run
/// # #[cfg(all(feature = "tcp", feature = "client", feature = "http1"))]
/// # async fn fake_fetch() -> hyper::Result<()> {
/// use hyper::{Client, Uri};
/// use hyper::ext::ReasonPhrase;
///
/// let res = Client::new().get(Uri::from_static("http://example.com/non_canonical_reason")).await?;
///
/// // Print out the non-canonical reason phrase, if it has one...
/// if let Some(reason) = res.extensions().get::<ReasonPhrase>() {
///     println!("non-canonical reason: {}", std::str::from_utf8(reason.as_bytes()).unwrap());
/// }
/// # Ok(())
/// # }
/// ```
///
/// # Servers
///
/// When a `ReasonPhrase` is present in the extensions of the `http::Response` written by a server,
/// its contents will be written in place of the canonical reason phrase when responding via HTTP/1.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ReasonPhrase(Bytes);
impl ReasonPhrase {
    /// Gets the reason phrase as bytes.
    pub(crate) fn as_bytes(&self) -> &[u8] {
        loop {}
    }
    /// Converts a static byte slice to a reason phrase.
    pub(crate) fn from_static(reason: &'static [u8]) -> Self {
        loop {}
    }
    /// Converts a `Bytes` directly into a `ReasonPhrase` without validating.
    ///
    /// Use with care; invalid bytes in a reason phrase can cause serious security problems if
    /// emitted in a response.
    pub(crate) unsafe fn from_bytes_unchecked(reason: Bytes) -> Self {
        loop {}
    }
}
impl TryFrom<&[u8]> for ReasonPhrase {
    type Error = InvalidReasonPhrase;
    fn try_from(reason: &[u8]) -> Result<Self, Self::Error> {
        loop {}
    }
}
impl TryFrom<Vec<u8>> for ReasonPhrase {
    type Error = InvalidReasonPhrase;
    fn try_from(reason: Vec<u8>) -> Result<Self, Self::Error> {
        loop {}
    }
}
impl TryFrom<String> for ReasonPhrase {
    type Error = InvalidReasonPhrase;
    fn try_from(reason: String) -> Result<Self, Self::Error> {
        loop {}
    }
}
impl TryFrom<Bytes> for ReasonPhrase {
    type Error = InvalidReasonPhrase;
    fn try_from(reason: Bytes) -> Result<Self, Self::Error> {
        loop {}
    }
}
impl Into<Bytes> for ReasonPhrase {
    fn into(self) -> Bytes {
        loop {}
    }
}
impl AsRef<[u8]> for ReasonPhrase {
    fn as_ref(&self) -> &[u8] {
        loop {}
    }
}
/// Error indicating an invalid byte when constructing a `ReasonPhrase`.
///
/// See [the spec][spec] for details on allowed bytes.
///
/// [spec]: https://httpwg.org/http-core/draft-ietf-httpbis-messaging-latest.html#rfc.section.4.p.7
#[derive(Debug)]
pub(crate) struct InvalidReasonPhrase {
    bad_byte: u8,
}
impl std::fmt::Display for InvalidReasonPhrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
impl std::error::Error for InvalidReasonPhrase {}
const fn is_valid_byte(b: u8) -> bool {
    loop {}
}
const fn find_invalid_byte(bytes: &[u8]) -> Option<u8> {
    loop {}
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_valid() {
        loop {}
    }
    #[test]
    fn empty_valid() {
        loop {}
    }
    #[test]
    fn obs_text_valid() {
        loop {}
    }
    const NEWLINE_PHRASE: &'static [u8] = b"hyp\ner";
    #[test]
    #[should_panic]
    fn newline_invalid_panic() {
        loop {}
    }
    #[test]
    fn newline_invalid_err() {
        loop {}
    }
    const CR_PHRASE: &'static [u8] = b"hyp\rer";
    #[test]
    #[should_panic]
    fn cr_invalid_panic() {
        loop {}
    }
    #[test]
    fn cr_invalid_err() {
        loop {}
    }
}
