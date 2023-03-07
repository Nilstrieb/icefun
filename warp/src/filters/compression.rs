//! Compression Filters
//!
//! Filters that compress the body of a response.
#[cfg(feature = "compression-brotli")]
use async_compression::tokio::bufread::BrotliEncoder;
#[cfg(feature = "compression-gzip")]
use async_compression::tokio::bufread::{DeflateEncoder, GzipEncoder};
use http::header::HeaderValue;
use hyper::{
    header::{CONTENT_ENCODING, CONTENT_LENGTH},
    Body,
};
use tokio_util::io::{ReaderStream, StreamReader};
use crate::filter::{Filter, WrapSealed};
use crate::reject::IsReject;
use crate::reply::{Reply, Response};
use self::internal::{CompressionProps, WithCompression};
enum CompressionAlgo {
    #[cfg(feature = "compression-brotli")]
    BR,
    #[cfg(feature = "compression-gzip")]
    DEFLATE,
    #[cfg(feature = "compression-gzip")]
    GZIP,
}
impl From<CompressionAlgo> for HeaderValue {
    #[inline]
    fn from(algo: CompressionAlgo) -> Self {
        loop {}
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Compression<F> {
    func: F,
}













#[cfg(feature = "compression-gzip")]
pub fn gzip() -> Compression<impl Fn(CompressionProps) -> Response + Copy> {
    loop {}
}













#[cfg(feature = "compression-gzip")]
pub fn deflate() -> Compression<impl Fn(CompressionProps) -> Response + Copy> {
    loop {}
}













#[cfg(feature = "compression-brotli")]
pub fn brotli() -> Compression<impl Fn(CompressionProps) -> Response + Copy> {
    loop {}
}
impl<FN, F> WrapSealed<F> for Compression<FN>
where
    FN: Fn(CompressionProps) -> Response + Clone + Send,
    F: Filter + Clone + Send,
    F::Extract: Reply,
    F::Error: IsReject,
{
    type Wrapped = WithCompression<FN, F>;
    fn wrap(&self, filter: F) -> Self::Wrapped {
        loop {}
    }
}
mod internal {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use bytes::Bytes;
    use futures_util::{ready, Stream, TryFuture};
    use hyper::Body;
    use pin_project::pin_project;
    use crate::filter::{Filter, FilterBase, Internal};
    use crate::reject::IsReject;
    use crate::reply::{Reply, Response};
    use super::Compression;
    
    
    #[pin_project]
    #[derive(Debug)]
    pub struct CompressableBody<S, E>
    where
        E: std::error::Error,
        S: Stream<Item = Result<Bytes, E>>,
    {
        #[pin]
        body: S,
    }
    impl<S, E> Stream for CompressableBody<S, E>
    where
        E: std::error::Error,
        S: Stream<Item = Result<Bytes, E>>,
    {
        type Item = std::io::Result<Bytes>;
        fn poll_next(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Option<Self::Item>> {
            loop {}
        }
    }
    impl From<Body> for CompressableBody<Body, hyper::Error> {
        fn from(body: Body) -> Self {
            loop {}
        }
    }
    
    #[derive(Debug)]
    pub struct CompressionProps {
        pub(super) body: CompressableBody<Body, hyper::Error>,
        pub(super) head: http::response::Parts,
    }
    impl From<http::Response<Body>> for CompressionProps {
        fn from(resp: http::Response<Body>) -> Self {
            loop {}
        }
    }
    #[allow(missing_debug_implementations)]
    pub struct Compressed(pub(super) Response);
    impl Reply for Compressed {
        #[inline]
        fn into_response(self) -> Response {
            loop {}
        }
    }
    #[allow(missing_debug_implementations)]
    #[derive(Clone, Copy)]
    pub struct WithCompression<FN, F> {
        pub(super) compress: Compression<FN>,
        pub(super) filter: F,
    }
    impl<FN, F> FilterBase for WithCompression<FN, F>
    where
        FN: Fn(CompressionProps) -> Response + Clone + Send,
        F: Filter + Clone + Send,
        F::Extract: Reply,
        F::Error: IsReject,
    {
        type Extract = (Compressed,);
        type Error = F::Error;
        type Future = WithCompressionFuture<FN, F::Future>;
        fn filter(&self, _: Internal) -> Self::Future {
            loop {}
        }
    }
    #[allow(missing_debug_implementations)]
    #[pin_project]
    pub struct WithCompressionFuture<FN, F> {
        compress: Compression<FN>,
        #[pin]
        future: F,
    }
    impl<FN, F> Future for WithCompressionFuture<FN, F>
    where
        FN: Fn(CompressionProps) -> Response,
        F: TryFuture,
        F::Ok: Reply,
        F::Error: IsReject,
    {
        type Output = Result<(Compressed,), F::Error>;
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            loop {}
        }
    }
}
