//! Host ("authority") filter
//!
use crate::filter::{filter_fn_one, Filter, One};
use crate::reject::{self, Rejection};
use futures_util::future;
pub use http::uri::Authority;
use std::str::FromStr;















pub fn exact(expected: &str) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    let expected = Authority::from_str(expected).expect("invalid host/authority");
    optional()
        .and_then(move |option: Option<Authority>| match option {
            Some(authority) if authority == expected => future::ok(()),
            _ => future::err(reject::not_found()),
        })
        .untuple_one()
}


























pub fn optional() -> impl Filter<Extract = One<Option<Authority>>, Error = Rejection> + Copy {
    filter_fn_one(move |route| {
        // The authority can be sent by clients in various ways:
        //
        //  1) in the "target URI"
        //    a) serialized in the start line (HTTP/1.1 proxy requests)
        //    b) serialized in `:authority` pseudo-header (HTTP/2 generated - "SHOULD")
        //  2) in the `Host` header (HTTP/1.1 origin requests, HTTP/2 converted)
        //
        // Hyper transparently handles 1a/1b, but not 2, so we must look at both.

        let from_uri = route.uri().authority();

        let name = "host";
        let from_header = route.headers()
            .get(name)
            .map(|value|
                // Header present, parse it
                value.to_str().map_err(|_| reject::invalid_header(name))
                    .and_then(|value| Authority::from_str(value).map_err(|_| reject::invalid_header(name)))
            );

        future::ready(match (from_uri, from_header) {
            // no authority in the request (HTTP/1.0 or non-conforming)
            (None, None) => Ok(None),

            // authority specified in either or both matching
            (Some(a), None) => Ok(Some(a.clone())),
            (None, Some(Ok(a))) => Ok(Some(a)),
            (Some(a), Some(Ok(b))) if *a == b => Ok(Some(b)),

            // mismatch
            (Some(_), Some(Ok(_))) => Err(reject::invalid_header(name)),

            // parse error
            (_, Some(Err(r))) => Err(r),
        })
    })
}
