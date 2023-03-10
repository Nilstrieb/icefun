//! Cookie Filters

use futures_util::future;
use headers::Cookie;

use super::header;
use crate::filter::{Filter, One};
use crate::reject::Rejection;
use std::convert::Infallible;
use std::str::FromStr;




pub fn cookie<T>(name: &'static str) -> impl Filter<Extract = One<T>, Error = Rejection> + Copy
where
    T: FromStr + Send + 'static,
{
    header::header2().and_then(move |cookie: Cookie| {
        let cookie = cookie
            .get(name)
            .ok_or_else(|| crate::reject::missing_cookie(name))
            .and_then(|s| T::from_str(s).map_err(|_| crate::reject::missing_cookie(name)));
        future::ready(cookie)
    })
}





pub fn optional<T>(
    name: &'static str,
) -> impl Filter<Extract = One<Option<T>>, Error = Infallible> + Copy
where
    T: FromStr + Send + 'static,
{
    header::optional2().map(move |opt: Option<Cookie>| {
        let cookie = opt.and_then(|cookie| cookie.get(name).map(|x| T::from_str(x)));
        match cookie {
            Some(Ok(t)) => Some(t),
            Some(Err(_)) => None,
            None => None,
        }
    })
}
