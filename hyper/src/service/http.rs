use crate::body::HttpBody;
use crate::common::Future;
use crate::{Request, Response};
use std::error::Error as StdError;

pub trait HttpService<ReqBody>: sealed::Sealed<ReqBody> {
    type ResBody;

    type Error;

    type Future: Future<Output = Result<Response<Self::ResBody>, Self::Error>>;
}
impl<T, B1, B2> HttpService<B1> for T
where
    T: tower_service::Service<Request<B1>, Response = Response<B2>>,
    B2: HttpBody,
    T::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type ResBody = B2;
    type Error = T::Error;
    type Future = T::Future;
}
impl<T, B1, B2> sealed::Sealed<B1> for T
where
    T: tower_service::Service<Request<B1>, Response = Response<B2>>,
    B2: HttpBody,
{
}
mod sealed {
    pub trait Sealed<T> {}
}
