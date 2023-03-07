#![feature(test)]
#![deny(warnings)]
extern crate test;
use std::net::SocketAddr;
use futures_util::future::join_all;
use hyper::client::HttpConnector;
use hyper::{body::HttpBody as _, Body, Method, Request, Response, Server};
#[bench]
fn http1_consecutive_x1_empty(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http1_consecutive_x1_req_10b(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http1_consecutive_x1_both_100kb(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http1_consecutive_x1_both_10mb(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http1_parallel_x10_empty(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http1_parallel_x10_req_10mb(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http1_parallel_x10_req_10kb_100_chunks(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http1_parallel_x10_res_1mb(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http1_parallel_x10_res_10mb(b: &mut test::Bencher) {
    loop {}
}
const HTTP2_MAX_WINDOW: u32 = std::u32::MAX >> 1;
#[bench]
fn http2_consecutive_x1_empty(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http2_consecutive_x1_req_10b(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http2_consecutive_x1_req_100kb(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http2_parallel_x10_empty(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http2_parallel_x10_req_10mb(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http2_parallel_x10_req_10kb_100_chunks(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http2_parallel_x10_req_10kb_100_chunks_adaptive_window(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http2_parallel_x10_req_10kb_100_chunks_max_window(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http2_parallel_x10_res_1mb(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn http2_parallel_x10_res_10mb(b: &mut test::Bencher) {
    loop {}
}
struct Opts {
    http2: bool,
    http2_stream_window: Option<u32>,
    http2_conn_window: Option<u32>,
    http2_adaptive_window: bool,
    parallel_cnt: u32,
    request_method: Method,
    request_body: Option<&'static [u8]>,
    request_chunks: usize,
    response_body: &'static [u8],
}
fn opts() -> Opts {
    loop {}
}
impl Opts {
    fn http2(mut self) -> Self {
        loop {}
    }
    fn http2_stream_window(mut self, sz: impl Into<Option<u32>>) -> Self {
        loop {}
    }
    fn http2_conn_window(mut self, sz: impl Into<Option<u32>>) -> Self {
        loop {}
    }
    fn http2_adaptive_window(mut self) -> Self {
        loop {}
    }
    fn method(mut self, m: Method) -> Self {
        loop {}
    }
    fn request_body(mut self, body: &'static [u8]) -> Self {
        loop {}
    }
    fn request_chunks(mut self, chunk: &'static [u8], cnt: usize) -> Self {
        loop {}
    }
    fn response_body(mut self, body: &'static [u8]) -> Self {
        loop {}
    }
    fn parallel(mut self, cnt: u32) -> Self {
        loop {}
    }
    fn bench(self, b: &mut test::Bencher) {
        loop {}
    }
}
fn spawn_server(rt: &tokio::runtime::Runtime, opts: &Opts) -> SocketAddr {
    loop {}
}
