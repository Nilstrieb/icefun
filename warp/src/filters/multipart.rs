//! Multipart body filters
//!
//! Filters that extract a multipart body for a route.
use std::fmt;
use std::future::Future;
use std::io::{Cursor, Read};
use std::pin::Pin;
use std::task::{Context, Poll};
use bytes::{Buf, Bytes};
use futures_util::{future, Stream};
use headers::ContentType;
use mime::Mime;
use multipart::server::Multipart;
use crate::filter::{Filter, FilterBase, Internal};
use crate::reject::{self, Rejection};
const DEFAULT_FORM_DATA_MAX_LENGTH: u64 = 1024 * 1024 * 2;



#[derive(Debug, Clone)]
pub struct FormOptions {
    max_length: u64,
}



pub struct FormData {
    inner: Multipart<Cursor<::bytes::Bytes>>,
}



pub struct Part {
    name: String,
    filename: Option<String>,
    content_type: Option<String>,
    data: Option<Vec<u8>>,
}




pub fn form() -> FormOptions {
    loop {}
}
impl FormOptions {
    
    
    
    pub fn max_length(mut self, max: u64) -> Self {
        loop {}
    }
}
type FormFut = Pin<Box<dyn Future<Output = Result<(FormData,), Rejection>> + Send>>;
impl FilterBase for FormOptions {
    type Extract = (FormData,);
    type Error = Rejection;
    type Future = FormFut;
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}
impl fmt::Debug for FormData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl Stream for FormData {
    type Item = Result<Part, crate::Error>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {}
    }
}
impl Part {
    
    pub fn name(&self) -> &str {
        loop {}
    }
    
    pub fn filename(&self) -> Option<&str> {
        loop {}
    }
    
    pub fn content_type(&self) -> Option<&str> {
        loop {}
    }
    
    pub async fn data(&mut self) -> Option<Result<impl Buf, crate::Error>> {
        loop {}
    }
    
    pub fn stream(self) -> impl Stream<Item = Result<impl Buf, crate::Error>> {
        loop {}
    }
    fn take_data(&mut self) -> Option<Result<Bytes, crate::Error>> {
        loop {}
    }
}
impl fmt::Debug for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
struct PartStream(Part);
impl Stream for PartStream {
    type Item = Result<Bytes, crate::Error>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {}
    }
}
