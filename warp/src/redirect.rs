//! Redirect requests to a new location.
//!
//! The types in this module are helpers that implement [`Reply`](Reply), and easy
//! to use in order to setup redirects.
use http::{header, StatusCode};
pub use self::sealed::AsLocation;
use crate::reply::{self, Reply};












pub fn redirect(uri: impl AsLocation) -> impl Reply {
    reply::with_header(
        StatusCode::MOVED_PERMANENTLY,
        header::LOCATION,
        uri.header_value(),
    )
}












pub fn found(uri: impl AsLocation) -> impl Reply {
    reply::with_header(StatusCode::FOUND, header::LOCATION, uri.header_value())
}














pub fn see_other(uri: impl AsLocation) -> impl Reply {
    reply::with_header(StatusCode::SEE_OTHER, header::LOCATION, uri.header_value())
}















pub fn temporary(uri: impl AsLocation) -> impl Reply {
    reply::with_header(
        StatusCode::TEMPORARY_REDIRECT,
        header::LOCATION,
        uri.header_value(),
    )
}















pub fn permanent(uri: impl AsLocation) -> impl Reply {
    reply::with_header(
        StatusCode::PERMANENT_REDIRECT,
        header::LOCATION,
        uri.header_value(),
    )
}
mod sealed {
    
    use http::{header::HeaderValue, Uri};
    
    
    
    
    pub trait AsLocation: Sealed {}
    pub trait Sealed {
        fn header_value(self) -> HeaderValue;
    }
    impl AsLocation for Uri {}
    impl Sealed for Uri {
        fn header_value(self) -> HeaderValue {
            loop {}
        }
    }
}
