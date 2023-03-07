use std::error::Error as StdError;
use crate::body::HttpBody;
use crate::common::{task, Future, Poll};
use crate::{Request, Response};

pub trait HttpService<ReqBody>: sealed::Sealed<ReqBody> {
    
    type ResBody: HttpBody;
    
    
    
    
    
    type Error: Into<Box<dyn StdError + Send + Sync>>;
    
    type Future: Future<Output = Result<Response<Self::ResBody>, Self::Error>>;
    #[doc(hidden)]
    fn poll_ready(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>>;
    #[doc(hidden)]
    fn call(&mut self, req: Request<ReqBody>) -> Self::Future;
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
    fn poll_ready(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn call(&mut self, req: Request<B1>) -> Self::Future {
        loop {}
    }
}
impl<T, B1, B2> sealed::Sealed<B1> for T
where
    T: tower_service::Service<Request<B1>, Response = Response<B2>>,
    B2: HttpBody,
{}
mod sealed {
    pub trait Sealed<T> {}
}
