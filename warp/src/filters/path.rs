//! Path Filters
//!
//! The filters here work on the "path" of requests.
//!
//! - [`path`](./fn.path.html) matches a specific segment, like `/foo`.
//! - [`param`](./fn.param.html) tries to parse a segment into a type, like `/:u16`.
//! - [`end`](./fn.end.html) matches when the path end is found.
//! - [`path!`](../../macro.path.html) eases combining multiple `path` and `param` filters.
//!
//! # Routing
//!
//! Routing in warp is simple yet powerful.
//!
//! First up, matching a single segment:
//!
//! ```
//! use warp::Filter;
//!
//! // GET /hi
//! let hi = warp::path("hi").map(|| {
//!     "Hello, World!"
//! });
//! ```
//!
//! How about multiple segments? It's easiest with the `path!` macro:
//!
//! ```
//! # use warp::Filter;
//! // GET /hello/from/warp
//! let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| {
//!     "Hello from warp!"
//! });
//! ```
//!
//! Neat! But how do I handle **parameters** in paths?
//!
//! ```
//! # use warp::Filter;
//! // GET /sum/:u32/:u32
//! let sum = warp::path!("sum" / u32 / u32).map(|a, b| {
//!     format!("{} + {} = {}", a, b, a + b)
//! });
//! ```
//!
//! In fact, any type that implements `FromStr` can be used, in any order:
//!
//! ```
//! # use warp::Filter;
//! // GET /:u16/times/:u16
//! let times = warp::path!(u16 / "times" / u16).map(|a, b| {
//!     format!("{} times {} = {}", a, b, a * b)
//! });
//! ```
//!
//! Oh shoot, those math routes should be **mounted** at a different path,
//! is that possible? Yep!
//!
//! ```
//! # use warp::Filter;
//! # let sum = warp::any().map(warp::reply);
//! # let times = sum.clone();
//! // GET /math/sum/:u32/:u32
//! // GET /math/:u16/times/:u16
//! let math = warp::path("math");
//! let math_sum = math.and(sum);
//! let math_times = math.and(times);
//! ```
//!
//! What! `and`? What's that do?
//!
//! It combines the filters in a sort of "this and then that" order. In fact,
//! it's exactly what the `path!` macro has been doing internally.
//!
//! ```
//! # use warp::Filter;
//! // GET /bye/:string
//! let bye = warp::path("bye")
//!     .and(warp::path::param())
//!     .map(|name: String| {
//!         format!("Good bye, {}!", name)
//!     });
//! ```
//!
//! Ah, so, can filters do things besides `and`?
//!
//! Why, yes they can! They can also `or`! As you might expect, `or` creates a
//! "this or else that" chain of filters. If the first doesn't succeed, then
//! it tries the other.
//!
//! So, those `math` routes could have been **mounted** all as one, with `or`.
//!
//!
//! ```
//! # use warp::Filter;
//! # let sum = warp::path("sum");
//! # let times = warp::path("times");
//! // GET /math/sum/:u32/:u32
//! // GET /math/:u16/times/:u16
//! let math = warp::path("math")
//!     .and(sum.or(times));
//! ```
//!
//! It turns out, using `or` is how you combine everything together into a
//! single API.
//!
//! ```
//! # use warp::Filter;
//! # let hi = warp::path("hi");
//! # let hello_from_warp = hi.clone();
//! # let bye = hi.clone();
//! # let math = hi.clone();
//! // GET /hi
//! // GET /hello/from/warp
//! // GET /bye/:string
//! // GET /math/sum/:u32/:u32
//! // GET /math/:u16/times/:u16
//! let routes = hi
//!     .or(hello_from_warp)
//!     .or(bye)
//!     .or(math);
//! ```
//!
//! Note that you will generally want path filters to come **before** other filters
//! like `body` or `headers`. If a different type of filter comes first, a request
//! with an invalid body for route `/right-path-wrong-body` may try matching against `/wrong-path`
//! and return the error from `/wrong-path` instead of the correct body-related error.
use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;
use futures_util::future;
use http::uri::PathAndQuery;
use self::internal::Opaque;
use crate::filter::{filter_fn, one, Filter, FilterBase, Internal, One, Tuple};
use crate::reject::{self, Rejection};
use crate::route::Route;



























pub fn path<P>(p: P) -> Exact<Opaque<P>>
where
    P: AsRef<str>,
{
    loop {}
}



#[allow(missing_debug_implementations)]
#[derive(Clone, Copy)]
pub struct Exact<P>(P);
impl<P> FilterBase for Exact<P>
where
    P: AsRef<str>,
{
    type Extract = ();
    type Error = Rejection;
    type Future = future::Ready<Result<Self::Extract, Self::Error>>;
    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        loop {}
    }
}














pub fn end() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    filter_fn(move |route| {
        if route.path().is_empty() {
            future::ok(())
        } else {
            future::err(reject::not_found())
        }
    })
}


















pub fn param<T: FromStr + Send + 'static>() -> impl Filter<
    Extract = One<T>,
    Error = Rejection,
> + Copy {
    filter_segment(|seg| {
        tracing::trace!("param?: {:?}", seg);
        if seg.is_empty() {
            return Err(reject::not_found());
        }
        T::from_str(seg).map(one).map_err(|_| reject::not_found())
    })
}

















pub fn tail() -> impl Filter<Extract = One<Tail>, Error = Infallible> + Copy {
    filter_fn(move |route| {
        let path = path_and_query(route);
        let idx = route.matched_path_index();
        let end = path.path().len() - idx;
        route.set_unmatched_path(end);
        future::ok(one(Tail { path, start_index: idx }))
    })
}

pub struct Tail {
    path: PathAndQuery,
    start_index: usize,
}
impl Tail {
    
    pub fn as_str(&self) -> &str {
        loop {}
    }
}
impl fmt::Debug for Tail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}


















pub fn peek() -> impl Filter<Extract = One<Peek>, Error = Infallible> + Copy {
    filter_fn(move |route| {
        let path = path_and_query(route);
        let idx = route.matched_path_index();
        future::ok(one(Peek { path, start_index: idx }))
    })
}

pub struct Peek {
    path: PathAndQuery,
    start_index: usize,
}
impl Peek {
    
    pub fn as_str(&self) -> &str {
        loop {}
    }
    
    pub fn segments(&self) -> impl Iterator<Item = &str> {
        self.as_str().split('/').filter(|seg| !seg.is_empty())
    }
}
impl fmt::Debug for Peek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}































pub fn full() -> impl Filter<Extract = One<FullPath>, Error = Infallible> + Copy {
    filter_fn(move |route| future::ok(one(FullPath(path_and_query(route)))))
}

pub struct FullPath(PathAndQuery);
impl FullPath {
    
    pub fn as_str(&self) -> &str {
        loop {}
    }
}
impl fmt::Debug for FullPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
fn filter_segment<F, U>(func: F) -> impl Filter<Extract = U, Error = Rejection> + Copy
where
    F: Fn(&str) -> Result<U, Rejection> + Copy,
    U: Tuple + Send + 'static,
{
    filter_fn(move |route| future::ready(with_segment(route, func)))
}
fn with_segment<F, U>(route: &mut Route, func: F) -> Result<U, Rejection>
where
    F: Fn(&str) -> Result<U, Rejection>,
{
    loop {}
}
fn path_and_query(route: &Route) -> PathAndQuery {
    loop {}
}























































#[macro_export]
macro_rules! path {
    ($($pieces:tt)*) => {
        { $crate ::__internal_path!(@ start $($pieces)*) }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __internal_path {
    (@ start) => {
        $crate ::path::end()
    };
    (@ start ..) => {
        { compile_error!("'..' cannot be the only segment") }
    };
    (@ start $first:tt $(/ $tail:tt)*) => {
        { $crate ::__internal_path!(@ munch $crate ::any(); [$first] [$(/ $tail)*]) }
    };
    (@ munch $sum:expr; [$cur:tt] [/ $next:tt $(/ $tail:tt)*]) => {
        { $crate ::__internal_path!(@ munch $crate ::Filter::and($sum, $crate
        ::__internal_path!(@ segment $cur)); [$next] [$(/ $tail)*]) }
    };
    (@ munch $sum:expr; [$cur:tt] []) => {
        { $crate ::__internal_path!(@ last $sum; $cur) }
    };
    (@ last $sum:expr; ..) => {
        $sum
    };
    (@ last $sum:expr; $end:tt) => {
        $crate ::Filter::and($crate ::Filter::and($sum, $crate ::__internal_path!(@
        segment $end)), $crate ::path::end())
    };
    (@ segment ..) => {
        compile_error!("'..' must be the last segment")
    };
    (@ segment $param:ty) => {
        $crate ::path::param::<$param > ()
    };
    (@ segment $s:literal) => {
        { #[derive(Clone, Copy)] struct __StaticPath; impl ::std::convert::AsRef < str >
        for __StaticPath { fn as_ref(& self) -> & str { static S : & str = $s; S } }
        $crate ::path(__StaticPath) }
    };
}



















fn _path_macro_compile_fail() {}
mod internal {
    #[allow(missing_debug_implementations)]
    #[derive(Clone, Copy)]
    pub struct Opaque<T>(pub(super) T);
    impl<T: AsRef<str>> AsRef<str> for Opaque<T> {
        #[inline]
        fn as_ref(&self) -> &str {
            loop {}
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_path_exact_size() {
        loop {}
    }
}
