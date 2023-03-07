

use http::header::{HeaderValue, ValueIter};
use http::HeaderMap;
#[cfg(all(feature = "http2", feature = "client"))]
use http::Method;
#[cfg(feature = "http1")]
pub(super) fn connection_keep_alive(value: &HeaderValue) -> bool {
    loop {}
}
#[cfg(feature = "http1")]
pub(super) fn connection_close(value: &HeaderValue) -> bool {
    loop {}
}
#[cfg(feature = "http1")]
fn connection_has(value: &HeaderValue, needle: &str) -> bool {
    loop {}
}
#[cfg(all(feature = "http1", feature = "server"))]
pub(super) fn content_length_parse(value: &HeaderValue) -> Option<u64> {
    loop {}
}
pub(super) fn content_length_parse_all(headers: &HeaderMap) -> Option<u64> {
    loop {}
}
pub(super) fn content_length_parse_all_values(
    values: ValueIter<'_, HeaderValue>,
) -> Option<u64> {
    loop {}
}
fn from_digits(bytes: &[u8]) -> Option<u64> {
    loop {}
}
#[cfg(all(feature = "http2", feature = "client"))]
pub(super) fn method_has_defined_payload_semantics(method: &Method) -> bool {
    loop {}
}
#[cfg(feature = "http2")]
pub(super) fn set_content_length_if_missing(headers: &mut HeaderMap, len: u64) {
    loop {}
}
#[cfg(feature = "http1")]
pub(super) fn transfer_encoding_is_chunked(headers: &HeaderMap) -> bool {
    loop {}
}
#[cfg(feature = "http1")]
pub(super) fn is_chunked(mut encodings: ValueIter<'_, HeaderValue>) -> bool {
    loop {}
}
#[cfg(feature = "http1")]
pub(super) fn is_chunked_(value: &HeaderValue) -> bool {
    loop {}
}
#[cfg(feature = "http1")]
pub(super) fn add_chunked(mut entry: http::header::OccupiedEntry<'_, HeaderValue>) {
    loop {}
}
