use std::borrow::Cow;
#[cfg(feature = "stream")]
use std::error::Error as StdError;
use std::fmt;
use bytes::Bytes;
use futures_channel::mpsc;
use futures_channel::oneshot;
use futures_core::Stream;
use http::HeaderMap;
use http_body::{Body as HttpBody, SizeHint};
use super::DecodedLength;
#[cfg(feature = "stream")]
use crate::common::sync_wrapper::SyncWrapper;
#[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
use crate::common::Never;
use crate::common::{task, watch, Pin, Poll};
#[cfg(all(feature = "http2", any(feature = "client", feature = "server")))]
use crate::proto::h2::ping;
type BodySender = mpsc::Sender<Result<Bytes, crate::Error>>;
type TrailersSender = oneshot::Sender<HeaderMap>;







#[must_use = "streams do nothing unless polled"]
pub struct Body {
    kind: Kind,
    
    
    extra: Option<Box<Extra>>,
}
enum Kind {
    Once(Option<Bytes>),
    Chan {
        content_length: DecodedLength,
        want_tx: watch::Sender,
        data_rx: mpsc::Receiver<Result<Bytes, crate::Error>>,
        trailers_rx: oneshot::Receiver<HeaderMap>,
    },
    #[cfg(all(feature = "http2", any(feature = "client", feature = "server")))]
    H2 { ping: ping::Recorder, content_length: DecodedLength, recv: h2::RecvStream },
    #[cfg(feature = "ffi")]
    Ffi(crate::ffi::UserBody),
    #[cfg(feature = "stream")]
    Wrapped(
        SyncWrapper<
            Pin<
                Box<
                    dyn Stream<
                        Item = Result<Bytes, Box<dyn StdError + Send + Sync>>,
                    > + Send,
                >,
            >,
        >,
    ),
}
struct Extra {
    
    
    
    
    
    
    
    
    delayed_eof: Option<DelayEof>,
}
#[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
type DelayEofUntil = oneshot::Receiver<Never>;
enum DelayEof {
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    NotEof(DelayEofUntil),
    
    
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    Eof(DelayEofUntil),
}













#[must_use = "Sender does nothing unless sent on"]
pub struct Sender {
    want_rx: watch::Receiver,
    data_tx: BodySender,
    trailers_tx: Option<TrailersSender>,
}
impl Body {
    
    
    
    
    
    
    
    
    
    
    #[inline]
    pub fn empty() -> Body {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[cfg(feature = "stream")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
    pub fn wrap_stream<S, O, E>(stream: S) -> Body
    where
        S: Stream<Item = Result<O, E>> + Send + 'static,
        O: Into<Bytes> + 'static,
        E: Into<Box<dyn StdError + Send + Sync>> + 'static,
    {
        loop {}
    }
    #[cfg(feature = "ffi")]
    pub(crate) fn as_ffi_mut(&mut self) -> &mut crate::ffi::UserBody {
        loop {}
    }
}
impl Default for Body {
    
    #[inline]
    fn default() -> Body {
        loop {}
    }
}
impl HttpBody for Body {
    type Data = Bytes;
    type Error = crate::Error;
    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        loop {}
    }
    fn poll_trailers(
        #[cfg_attr(not(feature = "http2"), allow(unused_mut))]
        mut self: Pin<&mut Self>,
        #[cfg_attr(not(feature = "http2"), allow(unused))]
        cx: &mut task::Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        loop {}
    }
    fn is_end_stream(&self) -> bool {
        loop {}
    }
    fn size_hint(&self) -> SizeHint {
        loop {}
    }
}
impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}




#[cfg(feature = "stream")]
impl Stream for Body {
    type Item = crate::Result<Bytes>;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {}
    }
}




#[cfg(feature = "stream")]
impl From<Box<dyn Stream<Item = Result<Bytes, Box<dyn StdError + Send + Sync>>> + Send>>
for Body {
    #[inline]
    fn from(
        stream: Box<
            dyn Stream<Item = Result<Bytes, Box<dyn StdError + Send + Sync>>> + Send,
        >,
    ) -> Body {
        loop {}
    }
}
impl From<Bytes> for Body {
    #[inline]
    fn from(chunk: Bytes) -> Body {
        loop {}
    }
}
impl From<Vec<u8>> for Body {
    #[inline]
    fn from(vec: Vec<u8>) -> Body {
        loop {}
    }
}
impl From<&'static [u8]> for Body {
    #[inline]
    fn from(slice: &'static [u8]) -> Body {
        loop {}
    }
}
impl From<Cow<'static, [u8]>> for Body {
    #[inline]
    fn from(cow: Cow<'static, [u8]>) -> Body {
        loop {}
    }
}
impl From<String> for Body {
    #[inline]
    fn from(s: String) -> Body {
        loop {}
    }
}
impl From<&'static str> for Body {
    #[inline]
    fn from(slice: &'static str) -> Body {
        loop {}
    }
}
impl From<Cow<'static, str>> for Body {
    #[inline]
    fn from(cow: Cow<'static, str>) -> Body {
        loop {}
    }
}
impl Sender {}
impl fmt::Debug for Sender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
