#![feature(test)]
#![deny(warnings)]
extern crate test;
use http::Uri;
use hyper::client::connect::HttpConnector;
use hyper::service::Service;
use std::net::SocketAddr;
use tokio::net::TcpListener;
#[bench]
fn http_connector(b: &mut test::Bencher) {
    loop {}
}
