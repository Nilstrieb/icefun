#[cfg(all(feature = "server", feature = "runtime"))]
use std::{pin::Pin, time::Duration};
use bytes::BytesMut;
use http::{HeaderMap, Method};
use httparse::ParserConfig;
#[cfg(all(feature = "server", feature = "runtime"))]
use tokio::time::Sleep;
use crate::body::DecodedLength;
use crate::proto::{BodyLength, MessageHead};
pub(crate) use self::conn::Conn;
pub(crate) use self::decode::Decoder;
pub(crate) use self::dispatch::Dispatcher;
pub(crate) use self::encode::{EncodedBuf, Encoder};

mod conn;
mod decode;
pub(crate) mod dispatch;
mod encode;
mod io;
mod role;
cfg_client! {
    pub (crate) type ClientTransaction = role::Client;
}
cfg_server! {
    pub (crate) type ServerTransaction = role::Server;
}
pub(crate) trait Http1Transaction {
    type Incoming;
    type Outgoing: Default;
    const LOG: &'static str;
    fn parse(bytes: &mut BytesMut, ctx: ParseContext<'_>) -> ParseResult<Self::Incoming>;
    fn encode(
        enc: Encode<'_, Self::Outgoing>,
        dst: &mut Vec<u8>,
    ) -> crate::Result<Encoder>;
    fn on_error(err: &crate::Error) -> Option<MessageHead<Self::Outgoing>>;
    fn is_client() -> bool {
        loop {}
    }
    fn is_server() -> bool {
        loop {}
    }
    fn should_error_on_parse_eof() -> bool {
        loop {}
    }
    fn should_read_first() -> bool {
        loop {}
    }
    fn update_date() {}
}

pub(crate) type ParseResult<T> = Result<Option<ParsedMessage<T>>, crate::error::Parse>;
#[derive(Debug)]
pub(crate) struct ParsedMessage<T> {
    head: MessageHead<T>,
    decode: DecodedLength,
    expect_continue: bool,
    keep_alive: bool,
    wants_upgrade: bool,
}
pub(crate) struct ParseContext<'a> {
    cached_headers: &'a mut Option<HeaderMap>,
    req_method: &'a mut Option<Method>,
    h1_parser_config: ParserConfig,
    #[cfg(all(feature = "server", feature = "runtime"))]
    h1_header_read_timeout: Option<Duration>,
    #[cfg(all(feature = "server", feature = "runtime"))]
    h1_header_read_timeout_fut: &'a mut Option<Pin<Box<Sleep>>>,
    #[cfg(all(feature = "server", feature = "runtime"))]
    h1_header_read_timeout_running: &'a mut bool,
    preserve_header_case: bool,
    #[cfg(feature = "ffi")]
    preserve_header_order: bool,
    h09_responses: bool,
    #[cfg(feature = "ffi")]
    on_informational: &'a mut Option<crate::ffi::OnInformational>,
    #[cfg(feature = "ffi")]
    raw_headers: bool,
}

pub(crate) struct Encode<'a, T> {
    head: &'a mut MessageHead<T>,
    body: Option<BodyLength>,
    #[cfg(feature = "server")]
    keep_alive: bool,
    req_method: &'a mut Option<Method>,
    title_case_headers: bool,
}

#[derive(Clone, Copy, Debug)]
struct Wants(u8);
impl Wants {
    const EMPTY: Wants = Wants(0b00);
    const EXPECT: Wants = Wants(0b01);
    const UPGRADE: Wants = Wants(0b10);
    #[must_use]
    fn add(self, other: Wants) -> Wants {
        loop {}
    }
    fn contains(&self, other: Wants) -> bool {
        loop {}
    }
}
