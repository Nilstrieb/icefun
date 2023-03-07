//! [`tracing`] filters.
//!
//! [`tracing`] is a framework for instrumenting Rust programs to
//! collect scoped, structured, and async-aware diagnostics. This module
//! provides a set of filters for instrumenting Warp applications with `tracing`
//! spans. [`Spans`] can be used to associate individual events  with a request,
//! and track contexts through the application.
//!
//! [`tracing`]: https://crates.io/crates/tracing
//! [`Spans`]: https://docs.rs/tracing/latest/tracing/#spans
use self::internal::WithTrace;
use crate::filter::{Filter, WrapSealed};
use crate::reject::IsReject;
use crate::reply::Reply;
use crate::route::Route;
use http::{self};
use std::net::SocketAddr;
use tracing::Span;

pub fn request() -> Trace<impl Fn(Info<'_>) -> Span + Clone> {
    trace(|info: Info<'_>| loop {})
}

pub fn trace<F>(func: F) -> Trace<F>
where
    F: Fn(Info<'_>) -> Span + Clone,
{
    loop {}
}

pub fn named(name: &'static str) -> Trace<impl Fn(Info<'_>) -> Span + Copy> {
    trace(move |_| tracing::debug_span!("context", "{}", name,))
}

#[derive(Clone, Copy, Debug)]
pub struct Trace<F> {
    func: F,
}

#[allow(missing_debug_implementations)]
pub struct Info<'a> {
    route: &'a Route,
}
impl<FN, F> WrapSealed<F> for Trace<FN>
where
    FN: Fn(Info<'_>) -> Span + Clone + Send,
    F: Filter + Clone + Send,
    F::Extract: Reply,
    F::Error: IsReject,
{
    type Wrapped = WithTrace<FN, F>;
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

    pub fn referer(&self) -> Option<&str> {
        loop {}
    }

    pub fn user_agent(&self) -> Option<&str> {
        loop {}
    }

    pub fn host(&self) -> Option<&str> {
        loop {}
    }

    pub fn request_headers(&self) -> &http::HeaderMap {
        loop {}
    }
}
mod internal {
    use super::{Info, Trace};
    use crate::filter::{Filter, FilterBase, Internal};
    use crate::reject::IsReject;
    use crate::reply::Reply;
    use crate::reply::Response;
    use futures_util::{future::Inspect, future::MapOk};

    #[allow(missing_debug_implementations)]
    pub struct Traced(pub(super) Response);
    impl Reply for Traced {
        #[inline]
        fn into_response(self) -> Response {
            loop {}
        }
    }
    #[allow(missing_debug_implementations)]
    #[derive(Clone, Copy)]
    pub struct WithTrace<FN, F> {
        pub(super) filter: F,
        pub(super) trace: Trace<FN>,
    }
    use tracing::instrument::Instrumented;
    use tracing::Span;
    fn finished_logger<E: IsReject>(reply: &Result<(Traced,), E>) {
        loop {}
    }
    fn convert_reply<R: Reply>(reply: R) -> (Traced,) {
        loop {}
    }
    impl<FN, F> FilterBase for WithTrace<FN, F>
    where
        FN: Fn(Info<'_>) -> Span + Clone + Send,
        F: Filter + Clone + Send,
        F::Extract: Reply,
        F::Error: IsReject,
    {
        type Extract = (Traced,);
        type Error = F::Error;
        type Future = Instrumented<
            Inspect<
                MapOk<F::Future, fn(F::Extract) -> Self::Extract>,
                fn(&Result<Self::Extract, F::Error>),
            >,
        >;
        fn filter(&self, _: Internal) -> Self::Future {
            loop {}
        }
    }
}
