//! Logger Filters
use std::fmt;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use http::{self, StatusCode};
use crate::filter::{Filter, WrapSealed};
use crate::reject::IsReject;
use crate::reply::Reply;
use crate::route::Route;
use self::internal::WithLog;

















pub fn log(name: &'static str) -> Log<impl Fn(Info<'_>) + Copy> {
    let func = move |info: Info<'_>| {
        log::info!(
            target : name, "{} \"{} {} {:?}\" {} \"{}\" \"{}\" {:?}", OptFmt(info.route
            .remote_addr()), info.method(), info.path(), info.route.version(), info
            .status().as_u16(), OptFmt(info.referer()), OptFmt(info.user_agent()), info
            .elapsed(),
        );
    };
    Log { func }
}




















pub fn custom<F>(func: F) -> Log<F>
where
    F: Fn(Info<'_>),
{
    loop {}
}

#[derive(Clone, Copy, Debug)]
pub struct Log<F> {
    func: F,
}

#[allow(missing_debug_implementations)]
pub struct Info<'a> {
    route: &'a Route,
    start: Instant,
    status: StatusCode,
}
impl<FN, F> WrapSealed<F> for Log<FN>
where
    FN: Fn(Info<'_>) + Clone + Send,
    F: Filter + Clone + Send,
    F::Extract: Reply,
    F::Error: IsReject,
{
    type Wrapped = WithLog<FN, F>;
    fn wrap(&self, filter: F) -> Self::Wrapped {
        loop {}
    }
}
impl<'a> Info<'a> {
    
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        loop {}
    }
    
    pub fn method(&self) -> &http::Method {
        loop {}
    }
    
    pub fn path(&self) -> &str {
        loop {}
    }
    
    pub fn version(&self) -> http::Version {
        loop {}
    }
    
    pub fn status(&self) -> http::StatusCode {
        loop {}
    }
    
    pub fn referer(&self) -> Option<&str> {
        loop {}
    }
    
    pub fn user_agent(&self) -> Option<&str> {
        loop {}
    }
    
    pub fn elapsed(&self) -> Duration {
        loop {}
    }
    
    pub fn host(&self) -> Option<&str> {
        loop {}
    }
    
    pub fn request_headers(&self) -> &http::HeaderMap {
        loop {}
    }
}
struct OptFmt<T>(Option<T>);
impl<T: fmt::Display> fmt::Display for OptFmt<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
mod internal {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::time::Instant;
    use futures_util::{TryFuture};
    use pin_project::pin_project;
    use super::{Info, Log};
    use crate::filter::{Filter, FilterBase, Internal};
    use crate::reject::IsReject;
    use crate::reply::{Reply, Response};
    
    #[allow(missing_debug_implementations)]
    pub struct Logged(pub(super) Response);
    impl Reply for Logged {
        #[inline]
        fn into_response(self) -> Response {
            loop {}
        }
    }
    #[allow(missing_debug_implementations)]
    #[derive(Clone, Copy)]
    pub struct WithLog<FN, F> {
        pub(super) filter: F,
        pub(super) log: Log<FN>,
    }
    impl<FN, F> FilterBase for WithLog<FN, F>
    where
        FN: Fn(Info<'_>) + Clone + Send,
        F: Filter + Clone + Send,
        F::Extract: Reply,
        F::Error: IsReject,
    {
        type Extract = (Logged,);
        type Error = F::Error;
        type Future = WithLogFuture<FN, F::Future>;
        fn filter(&self, _: Internal) -> Self::Future {
            loop {}
        }
    }
    #[allow(missing_debug_implementations)]
    #[pin_project]
    pub struct WithLogFuture<FN, F> {
        log: Log<FN>,
        #[pin]
        future: F,
        started: Instant,
    }
    impl<FN, F> Future for WithLogFuture<FN, F>
    where
        FN: Fn(Info<'_>),
        F: TryFuture,
        F::Ok: Reply,
        F::Error: IsReject,
    {
        type Output = Result<(Logged,), F::Error>;
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            loop {}
        }
    }
}
