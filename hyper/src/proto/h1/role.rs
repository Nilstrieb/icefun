use std::fmt::{self, Write};
use std::mem::MaybeUninit;
use bytes::Bytes;
use bytes::BytesMut;
#[cfg(feature = "server")]
use http::header::ValueIter;
use http::header::{self, Entry, HeaderName, HeaderValue};
use http::{HeaderMap, Method, StatusCode, Version};
#[cfg(all(feature = "server", feature = "runtime"))]
use tokio::time::Instant;
use tracing::{debug, error, trace, trace_span, warn};
use crate::body::DecodedLength;
#[cfg(feature = "server")]
use crate::common::date;
use crate::error::Parse;
use crate::ext::HeaderCaseMap;
#[cfg(feature = "ffi")]
use crate::ext::OriginalHeaderOrder;
use crate::headers;
use crate::proto::h1::{
    Encode, Encoder, Http1Transaction, ParseContext, ParseResult, ParsedMessage,
};
use crate::proto::{BodyLength, MessageHead, RequestHead, RequestLine};
const MAX_HEADERS: usize = 100;
const AVERAGE_HEADER_SIZE: usize = 30;
#[cfg(feature = "server")]
const MAX_URI_LEN: usize = (u16::MAX - 1) as usize;
macro_rules! header_name {
    ($bytes:expr) => {
        { { match HeaderName::from_bytes($bytes) { Ok(name) => name, Err(e) =>
        maybe_panic!(e), } } }
    };
}
macro_rules! header_value {
    ($bytes:expr) => {
        { { unsafe { HeaderValue::from_maybe_shared_unchecked($bytes) } } }
    };
}
macro_rules! maybe_panic {
    ($($arg:tt)*) => {
        { let _err = ($($arg)*); if cfg!(debug_assertions) { panic!("{:?}", _err); } else
        { error!("Internal Hyper error, please report {:?}", _err); return
        Err(Parse::Internal) } }
    };
}
pub(super) fn parse_headers<T>(
    bytes: &mut BytesMut,
    ctx: ParseContext<'_>,
) -> ParseResult<T::Incoming>
where
    T: Http1Transaction,
{
    loop {}
}
pub(super) fn encode_headers<T>(
    enc: Encode<'_, T::Outgoing>,
    dst: &mut Vec<u8>,
) -> crate::Result<Encoder>
where
    T: Http1Transaction,
{
    loop {}
}
#[cfg(feature = "client")]
pub(crate) enum Client {}
#[cfg(feature = "server")]
pub(crate) enum Server {}
#[cfg(feature = "server")]
impl Http1Transaction for Server {
    type Incoming = RequestLine;
    type Outgoing = StatusCode;
    const LOG: &'static str = "{role=server}";
    fn parse(buf: &mut BytesMut, ctx: ParseContext<'_>) -> ParseResult<RequestLine> {
        loop {}
    }
    fn encode(
        mut msg: Encode<'_, Self::Outgoing>,
        dst: &mut Vec<u8>,
    ) -> crate::Result<Encoder> {
        loop {}
    }
    fn on_error(err: &crate::Error) -> Option<MessageHead<Self::Outgoing>> {
        loop {}
    }
    fn is_server() -> bool {
        loop {}
    }
    fn update_date() {
        loop {}
    }
}
#[cfg(feature = "server")]
impl Server {
    fn can_have_body(method: &Option<Method>, status: StatusCode) -> bool {
        loop {}
    }
    fn can_chunked(method: &Option<Method>, status: StatusCode) -> bool {
        loop {}
    }
    fn can_have_content_length(method: &Option<Method>, status: StatusCode) -> bool {
        loop {}
    }
    fn can_have_implicit_zero_content_length(
        method: &Option<Method>,
        status: StatusCode,
    ) -> bool {
        loop {}
    }
    fn encode_headers_with_lower_case(
        msg: Encode<'_, StatusCode>,
        dst: &mut Vec<u8>,
        is_last: bool,
        orig_len: usize,
        wrote_len: bool,
    ) -> crate::Result<Encoder> {
        loop {}
    }
    #[cold]
    #[inline(never)]
    fn encode_headers_with_original_case(
        msg: Encode<'_, StatusCode>,
        dst: &mut Vec<u8>,
        is_last: bool,
        orig_len: usize,
        wrote_len: bool,
        orig_headers: &HeaderCaseMap,
    ) -> crate::Result<Encoder> {
        loop {}
    }
    #[inline]
    fn encode_headers<W>(
        msg: Encode<'_, StatusCode>,
        dst: &mut Vec<u8>,
        mut is_last: bool,
        orig_len: usize,
        mut wrote_len: bool,
        mut header_name_writer: W,
    ) -> crate::Result<Encoder>
    where
        W: HeaderNameWriter,
    {
        loop {}
    }
}
#[cfg(feature = "server")]
trait HeaderNameWriter {
    fn write_full_header_line(
        &mut self,
        dst: &mut Vec<u8>,
        line: &str,
        name_value_pair: (HeaderName, &str),
    );
    fn write_header_name_with_colon(
        &mut self,
        dst: &mut Vec<u8>,
        name_with_colon: &str,
        name: HeaderName,
    );
    fn write_header_name(&mut self, dst: &mut Vec<u8>, name: &HeaderName);
}
#[cfg(feature = "client")]
impl Http1Transaction for Client {
    type Incoming = StatusCode;
    type Outgoing = RequestLine;
    const LOG: &'static str = "{role=client}";
    fn parse(buf: &mut BytesMut, ctx: ParseContext<'_>) -> ParseResult<StatusCode> {
        loop {}
    }
    fn encode(
        msg: Encode<'_, Self::Outgoing>,
        dst: &mut Vec<u8>,
    ) -> crate::Result<Encoder> {
        loop {}
    }
    fn on_error(_err: &crate::Error) -> Option<MessageHead<Self::Outgoing>> {
        loop {}
    }
    fn is_client() -> bool {
        loop {}
    }
}
#[cfg(feature = "client")]
impl Client {
    /// Returns Some(length, wants_upgrade) if successful.
    ///
    /// Returns None if this message head should be skipped (like a 100 status).
    fn decoder(
        inc: &MessageHead<StatusCode>,
        method: &mut Option<Method>,
    ) -> Result<Option<(DecodedLength, bool)>, Parse> {
        loop {}
    }
    fn set_length(head: &mut RequestHead, body: Option<BodyLength>) -> Encoder {
        loop {}
    }
}
fn set_content_length(headers: &mut HeaderMap, len: u64) -> Encoder {
    loop {}
}
#[derive(Clone, Copy)]
struct HeaderIndices {
    name: (usize, usize),
    value: (usize, usize),
}
fn record_header_indices(
    bytes: &[u8],
    headers: &[httparse::Header<'_>],
    indices: &mut [MaybeUninit<HeaderIndices>],
) -> Result<(), crate::error::Parse> {
    loop {}
}
fn title_case(dst: &mut Vec<u8>, name: &[u8]) {
    loop {}
}
fn write_headers_title_case(headers: &HeaderMap, dst: &mut Vec<u8>) {
    loop {}
}
fn write_headers(headers: &HeaderMap, dst: &mut Vec<u8>) {
    loop {}
}
#[cold]
fn write_headers_original_case(
    headers: &HeaderMap,
    orig_case: &HeaderCaseMap,
    dst: &mut Vec<u8>,
    title_case_headers: bool,
) {
    loop {}
}
struct FastWrite<'a>(&'a mut Vec<u8>);
impl<'a> fmt::Write for FastWrite<'a> {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        loop {}
    }
    #[inline]
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        loop {}
    }
}
#[inline]
fn extend(dst: &mut Vec<u8>, data: &[u8]) {
    loop {}
}
#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use super::*;
    #[test]
    fn test_parse_request() {
        loop {}
    }
    #[test]
    fn test_parse_response() {
        loop {}
    }
    #[test]
    fn test_parse_request_errors() {
        loop {}
    }
    const H09_RESPONSE: &'static str = "Baguettes are super delicious, don't you agree?";
    #[test]
    fn test_parse_response_h09_allowed() {
        loop {}
    }
    #[test]
    fn test_parse_response_h09_rejected() {
        loop {}
    }
    const RESPONSE_WITH_WHITESPACE_BETWEEN_HEADER_NAME_AND_COLON: &'static str = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Credentials : true\r\n\r\n";
    #[test]
    fn test_parse_allow_response_with_spaces_before_colons() {
        loop {}
    }
    #[test]
    fn test_parse_reject_response_with_spaces_before_colons() {
        loop {}
    }
    #[test]
    fn test_parse_preserve_header_case_in_request() {
        loop {}
    }
    #[test]
    fn test_decoder_request() {
        loop {}
    }
    #[test]
    fn test_decoder_response() {
        loop {}
    }
    #[test]
    fn test_client_request_encode_title_case() {
        loop {}
    }
    #[test]
    fn test_client_request_encode_orig_case() {
        loop {}
    }
    #[test]
    fn test_client_request_encode_orig_and_title_case() {
        loop {}
    }
    #[test]
    fn test_server_encode_connect_method() {
        loop {}
    }
    #[test]
    fn test_server_response_encode_title_case() {
        loop {}
    }
    #[test]
    fn test_server_response_encode_orig_case() {
        loop {}
    }
    #[test]
    fn test_server_response_encode_orig_and_title_case() {
        loop {}
    }
    #[test]
    fn parse_header_htabs() {
        loop {}
    }
    #[test]
    fn test_write_headers_orig_case_empty_value() {
        loop {}
    }
    #[test]
    fn test_write_headers_orig_case_multiple_entries() {
        loop {}
    }
    #[cfg(feature = "nightly")]
    use test::Bencher;
    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_parse_incoming(b: &mut Bencher) {
        loop {}
    }
    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_parse_short(b: &mut Bencher) {
        loop {}
    }
    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_server_encode_headers_preset(b: &mut Bencher) {
        loop {}
    }
    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_server_encode_no_headers(b: &mut Bencher) {
        loop {}
    }
}
