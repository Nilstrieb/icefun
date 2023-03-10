//! Request Extensions

use std::convert::Infallible;

use futures_util::future;

use crate::filter::{filter_fn_one, Filter};
use crate::reject::{self, Rejection};




pub fn get<T: Clone + Send + Sync + 'static>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    filter_fn_one(|route| {
        let route = route
            .extensions()
            .get::<T>()
            .cloned()
            .ok_or_else(|| reject::known(MissingExtension { _p: () }));
        future::ready(route)
    })
}




pub fn optional<T: Clone + Send + Sync + 'static>(
) -> impl Filter<Extract = (Option<T>,), Error = Infallible> + Copy {
    filter_fn_one(|route| future::ok(route.extensions().get::<T>().cloned()))
}

unit_error! {
    
    pub MissingExtension: "Missing request extension"
}
