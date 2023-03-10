//! Pieces pertaining to the HTTP message protocol.
cfg_feature! {
    #![feature = "http1"] pub (crate) mod h1; pub (crate) use self::h1::Conn;
    #[cfg(feature = "client")] pub (crate) use self::h1::dispatch; #[cfg(feature =
    "server")] pub (crate) use self::h1::ServerTransaction;
}
#[cfg(feature = "http2")]
pub(crate) mod h2;

#[derive(Debug, Default)]
pub(crate) struct MessageHead<S> {
    
    pub(crate) version: http::Version,
    
    pub(crate) subject: S,
    
    pub(crate) headers: http::HeaderMap,
    
    extensions: http::Extensions,
}

#[cfg(feature = "http1")]
pub(crate) type RequestHead = MessageHead<RequestLine>;
#[derive(Debug, Default, PartialEq)]
#[cfg(feature = "http1")]
pub(crate) struct RequestLine(pub(crate) http::Method, pub(crate) http::Uri);

#[cfg(all(feature = "http1", feature = "client"))]
pub(crate) type ResponseHead = MessageHead<http::StatusCode>;
#[derive(Debug)]
#[cfg(feature = "http1")]
pub(crate) enum BodyLength {
    
    Known(u64),
    
    Unknown,
}

pub(crate) enum Dispatched {
    
    Shutdown,
    
    #[cfg(feature = "http1")]
    Upgrade(crate::upgrade::Pending),
}
impl MessageHead<http::StatusCode> {
    fn into_response<B>(self, body: B) -> http::Response<B> {
        loop {}
    }
}
