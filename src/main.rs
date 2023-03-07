use std::convert::Infallible;
use std::net::SocketAddr;

use futures::future;
use hyper::service::make_service_fn;
use warp::Filter;

fn main() {
    let svc = warp::service(
        warp::path::end()
            .map(|| "Hello, world")
            // Try commenting out this line to make the program compile again.
            // vvvvvvvvvvvvvvvvvvvvvvvvvvv
            .with(warp::trace::request()),
    );
    let make_svc = make_service_fn(move |_| future::ok::<_, Infallible>(svc.clone()));
    tokio::spawn(hyper::Server::bind().serve(make_svc));
}
