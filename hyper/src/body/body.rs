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
/// A stream of `Bytes`, used when receiving bodies.
///
/// A good default [`HttpBody`](crate::body::HttpBody) to use in many
/// applications.
///
/// Note: To read the full body, use [`body::to_bytes`](crate::body::to_bytes)
/// or [`body::aggregate`](crate::body::aggregate).
#[must_use = "streams do nothing unless polled"]
pub struct Body {
    kind: Kind,
    /// Keep the extra bits in an `Option<Box<Extra>>`, so that
    /// Body stays small in the common case (no extras needed).
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
    /// Allow the client to pass a future to delay the `Body` from returning
    /// EOF. This allows the `Client` to try to put the idle connection
    /// back into the pool before the body is "finished".
    ///
    /// The reason for this is so that creating a new request after finishing
    /// streaming the body of a response could sometimes result in creating
    /// a brand new connection, since the pool didn't know about the idle
    /// connection yet.
    delayed_eof: Option<DelayEof>,
}
#[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
type DelayEofUntil = oneshot::Receiver<Never>;
enum DelayEof {
    /// Initial state, stream hasn't seen EOF yet.
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    NotEof(DelayEofUntil),
    /// Transitions to this state once we've seen `poll` try to
    /// return EOF (`None`). This future is then polled, and
    /// when it completes, the Body finally returns EOF (`None`).
    #[cfg(any(feature = "http1", feature = "http2"))]
    #[cfg(feature = "client")]
    Eof(DelayEofUntil),
}
/// A sender half created through [`Body::channel()`].
///
/// Useful when wanting to stream chunks from another thread.
///
/// ## Body Closing
///
/// Note that the request body will always be closed normally when the sender is dropped (meaning
/// that the empty terminating chunk will be sent to the remote). If you desire to close the
/// connection with an incomplete response (e.g. in the case of an error during asynchronous
/// processing), call the [`Sender::abort()`] method to abort the body in an abnormal fashion.
///
/// [`Body::channel()`]: struct.Body.html#method.channel
/// [`Sender::abort()`]: struct.Sender.html#method.abort
#[must_use = "Sender does nothing unless sent on"]
pub struct Sender {
    want_rx: watch::Receiver,
    data_tx: BodySender,
    trailers_tx: Option<TrailersSender>,
}
const WANT_PENDING: usize = 1;
const WANT_READY: usize = 2;
impl Body {
    /// Create an empty `Body` stream.
    ///
    /// # Example
    ///
    /// ```
    /// use hyper::{Body, Request};
    ///
    /// // create a `GET /` request
    /// let get = Request::new(Body::empty());
    /// ```
    #[inline]
    pub fn empty() -> Body {
        loop {}
    }
    /// Wrap a futures `Stream` in a box inside `Body`.
    ///
    /// # Example
    ///
    /// ```
    /// # use hyper::Body;
    /// let chunks: Vec<Result<_, std::io::Error>> = vec![
    ///     Ok("hello"),
    ///     Ok(" "),
    ///     Ok("world"),
    /// ];
    ///
    /// let stream = futures_util::stream::iter(chunks);
    ///
    /// let body = Body::wrap_stream(stream);
    /// ```
    ///
    /// # Optional
    ///
    /// This function requires enabling the `stream` feature in your
    /// `Cargo.toml`.
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
    /// Returns `Body::empty()`.
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
/// # Optional
///
/// This function requires enabling the `stream` feature in your
/// `Cargo.toml`.
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
/// # Optional
///
/// This function requires enabling the `stream` feature in your
/// `Cargo.toml`.
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
#[cfg(test)]
mod tests {
    use std::mem;
    use std::task::Poll;
    use super::{Body, DecodedLength, HttpBody, Sender, SizeHint};
    #[test]
    fn test_size_of() {
        loop {}
    }
    #[test]
    fn size_hint() {
        loop {}
    }
    #[tokio::test]
    async fn channel_abort() {
        loop {}
    }
    #[tokio::test]
    async fn channel_abort_when_buffer_is_full() {
        loop {}
    }
    #[test]
    fn channel_buffers_one() {
        loop {}
    }
    #[tokio::test]
    async fn channel_empty() {
        loop {}
    }
    #[test]
    fn channel_ready() {
        loop {}
    }
    #[test]
    fn channel_wanter() {
        loop {}
    }
    #[test]
    fn channel_notices_closure() {
        loop {}
    }
}
