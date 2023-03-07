use super::{HttpService, Service};
use crate::body::HttpBody;
use crate::common::{task, Future, Poll};
use std::error::Error as StdError;
use std::fmt;
use tokio::io::{AsyncRead, AsyncWrite};
pub(crate) trait MakeConnection<Target>: self::sealed::Sealed<(Target,)> {
    type Connection: AsyncRead + AsyncWrite;
    type Error;
    type Future: Future<Output = Result<Self::Connection, Self::Error>>;
    fn poll_ready(&mut self, cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn make_connection(&mut self, target: Target) -> Self::Future;
}
impl<S, Target> self::sealed::Sealed<(Target,)> for S where S: Service<Target> {}
impl<S, Target> MakeConnection<Target> for S
where
    S: Service<Target>,
    S::Response: AsyncRead + AsyncWrite,
{
    type Connection = S::Response;
    type Error = S::Error;
    type Future = S::Future;
    fn poll_ready(&mut self, cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn make_connection(&mut self, target: Target) -> Self::Future {
        loop {}
    }
}
pub trait MakeServiceRef<Target, ReqBody> {
    type ResBody;
    type Error;
    type Service: HttpService<ReqBody>;
}

impl<T, Target, E, ME, S, F, IB, OB> MakeServiceRef<Target, IB> for T
where
    T: for<'a> Service<&'a Target, Error = ME, Response = S, Future = F>,
    E: Into<Box<dyn StdError + Send + Sync>>,
    ME: Into<Box<dyn StdError + Send + Sync>>,
    S: HttpService<IB, ResBody = OB, Error = E>,
    F: Future<Output = Result<S, ME>>,
    IB: HttpBody,
    OB: HttpBody,
{
    type Error = E;
    type Service = S;
    type ResBody = OB;
}
impl<T, Target, S, B1, B2> self::sealed::Sealed<(Target, B1)> for T
where
    T: for<'a> Service<&'a Target, Response = S>,
    S: HttpService<B1, ResBody = B2>,
    B1: HttpBody,
    B2: HttpBody,
{
}

pub fn make_service_fn<F, Target, Ret>(f: F) -> MakeServiceFn<F>
where
    F: FnMut(&Target) -> Ret,
    Ret: Future,
{
    loop {}
}

#[derive(Clone, Copy)]
pub struct MakeServiceFn<F> {
    f: F,
}
impl<'t, F, Ret, Target, Svc, MkErr> Service<&'t Target> for MakeServiceFn<F>
where
    F: FnMut(&Target) -> Ret,
    Ret: Future<Output = Result<Svc, MkErr>>,
    MkErr: Into<Box<dyn StdError + Send + Sync>>,
{
    type Error = MkErr;
    type Response = Svc;
    type Future = Ret;
    fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn call(&mut self, target: &'t Target) -> Self::Future {
        loop {}
    }
}
impl<F> fmt::Debug for MakeServiceFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
mod sealed {
    pub trait Sealed<X> {}
    #[allow(unreachable_pub)]
    pub trait CantImpl {}
    #[allow(missing_debug_implementations)]
    pub enum CantName {}
    impl CantImpl for CantName {}
}
