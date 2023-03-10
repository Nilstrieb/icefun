use crate::body::HttpBody;
use crate::common::{task, Future, Poll};
use crate::{Request, Response};
use std::error::Error as StdError;
use std::fmt;
use std::marker::PhantomData;

pub fn service_fn<F, R, S>(f: F) -> ServiceFn<F, R>
where
    F: FnMut(Request<R>) -> S,
    S: Future,
{
    loop {}
}

pub struct ServiceFn<F, R> {
    f: F,
    _req: PhantomData<fn(R)>,
}
impl<F, ReqBody, Ret, ResBody, E> tower_service::Service<crate::Request<ReqBody>>
    for ServiceFn<F, ReqBody>
where
    F: FnMut(Request<ReqBody>) -> Ret,
    ReqBody: HttpBody,
    Ret: Future<Output = Result<Response<ResBody>, E>>,
    E: Into<Box<dyn StdError + Send + Sync>>,
    ResBody: HttpBody,
{
    type Response = crate::Response<ResBody>;
    type Error = E;
    type Future = Ret;
    fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        loop {}
    }
}
impl<F, R> fmt::Debug for ServiceFn<F, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<F, R> Clone for ServiceFn<F, R>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        loop {}
    }
}
impl<F, R> Copy for ServiceFn<F, R> where F: Copy {}
