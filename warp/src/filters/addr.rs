//! Socket Address filters.

use std::convert::Infallible;
use std::net::SocketAddr;

use crate::filter::{filter_fn_one, Filter};

















pub fn remote() -> impl Filter<Extract = (Option<SocketAddr>,), Error = Infallible> + Copy {
    filter_fn_one(|route| futures_util::future::ok(route.remote_addr()))
}
