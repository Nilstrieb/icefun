//! Body filters
//!
//! Filters that extract a body for a route.
use std::error::Error as StdError;
use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};
use bytes::{Buf, Bytes};
use futures_util::{future, Stream, TryFutureExt};
use headers::ContentLength;
use http::header::CONTENT_TYPE;
use hyper::Body;
use mime;
use serde::de::DeserializeOwned;


use crate::filter::{filter_fn, filter_fn_one, Filter, FilterBase};
use crate::reject::{self, Rejection};
type BoxError = Box<dyn StdError + Send + Sync>;
pub(crate) fn body() -> impl Filter<Extract = (Body,), Error = Rejection> + Copy {
    filter_fn_one(|route| {
        future::ready(
            route
                .take_body()
                .ok_or_else(|| {
                    tracing::error!("request body already taken in previous filter");
                    reject::known(BodyConsumedMultipleTimes {
                        _p: (),
                    })
                }),
        )
    })
}














pub fn content_length_limit(
    limit: u64,
) -> impl Filter<Extract = (), Error = Rejection> + Copy {
    crate::filters::header::header2()
        .map_err(
            crate::filter::Internal,
            |_| {
                tracing::debug!("content-length missing");
                reject::length_required()
            },
        )
        .and_then(move |ContentLength(length)| {
            if length <= limit {
                future::ok(())
            } else {
                tracing::debug!("content-length: {} is over limit {}", length, limit);
                future::err(reject::payload_too_large())
            }
        })
        .untuple_one()
}









pub fn stream() -> impl Filter<
    Extract = (impl Stream<Item = Result<impl Buf, crate::Error>>,),
    Error = Rejection,
> + Copy {
    body().map(|body: Body| BodyStream { body })
}























pub fn bytes() -> impl Filter<Extract = (Bytes,), Error = Rejection> + Copy {
    body()
        .and_then(|body: hyper::Body| {
            hyper::body::to_bytes(body)
                .map_err(|err| {
                    tracing::debug!("to_bytes error: {}", err);
                    reject::known(BodyReadError(err))
                })
        })
}





























pub fn aggregate() -> impl Filter<Extract = (impl Buf,), Error = Rejection> + Copy {
    body()
        .and_then(|body: ::hyper::Body| {
            hyper::body::aggregate(body)
                .map_err(|err| {
                    tracing::debug!("aggregate error: {}", err);
                    reject::known(BodyReadError(err))
                })
        })
}




















pub fn json<T: DeserializeOwned + Send>() -> impl Filter<
    Extract = (T,),
    Error = Rejection,
> + Copy {
    is_content_type::<Json>()
        .and(bytes())
        .and_then(|buf| async move {
            Json::decode(buf)
                .map_err(|err| {
                    tracing::debug!("request json body error: {}", err);
                    reject::known(BodyDeserializeError { cause: err })
                })
        })
}
























pub fn form<T: DeserializeOwned + Send>() -> impl Filter<
    Extract = (T,),
    Error = Rejection,
> + Copy {
    is_content_type::<Form>()
        .and(aggregate())
        .and_then(|buf| async move {
            Form::decode(buf)
                .map_err(|err| {
                    tracing::debug!("request form body error: {}", err);
                    reject::known(BodyDeserializeError { cause: err })
                })
        })
}
trait Decode {
    const MIME: (mime::Name<'static>, mime::Name<'static>);
    const WITH_NO_CONTENT_TYPE: bool;
    fn decode<B: Buf, T: DeserializeOwned>(buf: B) -> Result<T, BoxError>;
}
struct Json;
impl Decode for Json {
    const MIME: (mime::Name<'static>, mime::Name<'static>) = (
        mime::APPLICATION,
        mime::JSON,
    );
    const WITH_NO_CONTENT_TYPE: bool = true;
    fn decode<B: Buf, T: DeserializeOwned>(mut buf: B) -> Result<T, BoxError> {
        loop {}
    }
}
struct Form;
impl Decode for Form {
    const MIME: (mime::Name<'static>, mime::Name<'static>) = (
        mime::APPLICATION,
        mime::WWW_FORM_URLENCODED,
    );
    const WITH_NO_CONTENT_TYPE: bool = true;
    fn decode<B: Buf, T: DeserializeOwned>(buf: B) -> Result<T, BoxError> {
        loop {}
    }
}
fn is_content_type<D: Decode>() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    filter_fn(move |route| {
        let (type_, subtype) = D::MIME;
        if let Some(value) = route.headers().get(CONTENT_TYPE) {
            tracing::trace!("is_content_type {}/{}? {:?}", type_, subtype, value);
            let ct = value.to_str().ok().and_then(|s| s.parse::<mime::Mime>().ok());
            if let Some(ct) = ct {
                if ct.type_() == type_ && ct.subtype() == subtype {
                    future::ok(())
                } else {
                    tracing::debug!(
                        "content-type {:?} doesn't match {}/{}", value, type_, subtype
                    );
                    future::err(reject::unsupported_media_type())
                }
            } else {
                tracing::debug!("content-type {:?} couldn't be parsed", value);
                future::err(reject::unsupported_media_type())
            }
        } else if D::WITH_NO_CONTENT_TYPE {
            tracing::trace!("no content-type header, assuming {}/{}", type_, subtype);
            future::ok(())
        } else {
            tracing::debug!("no content-type found");
            future::err(reject::unsupported_media_type())
        }
    })
}
struct BodyStream {
    body: Body,
}
impl Stream for BodyStream {
    type Item = Result<Bytes, crate::Error>;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {}
    }
}

#[derive(Debug)]
pub struct BodyDeserializeError {
    cause: BoxError,
}
impl fmt::Display for BodyDeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for BodyDeserializeError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        loop {}
    }
}
#[derive(Debug)]
pub(crate) struct BodyReadError(::hyper::Error);
impl fmt::Display for BodyReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for BodyReadError {}
unit_error! {
    pub (crate) BodyConsumedMultipleTimes : "Request body consumed multiple times"
}
