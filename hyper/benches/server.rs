#![feature(test)]
#![deny(warnings)]
extern crate test;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::time::Duration;
use futures_util::{stream, StreamExt};
use tokio::sync::oneshot;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server};
macro_rules! bench_server {
    ($b:ident, $header:expr, $body:expr) => {
        { let _ = pretty_env_logger::try_init(); let (_until_tx, until_rx) =
        oneshot::channel::< () > (); let addr = { let (addr_tx, addr_rx) =
        mpsc::channel(); std::thread::spawn(move || { let addr = "127.0.0.1:0".parse()
        .unwrap(); let make_svc = make_service_fn(| _ | async { Ok::< _, hyper::Error >
        (service_fn(| _ | async { Ok::< _, hyper::Error > (Response::builder()
        .header($header .0, $header .1).header("content-type", "text/plain").body($body
        ()).unwrap(),) })) }); let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all().build().expect("rt build"); let srv = rt.block_on(async move {
        Server::bind(& addr).serve(make_svc) }); addr_tx.send(srv.local_addr()).unwrap();
        let graceful = srv.with_graceful_shutdown(async { until_rx.await.ok(); }); rt
        .block_on(async move { if let Err(e) = graceful.await {
        panic!("server error: {}", e); } }); }); addr_rx.recv().unwrap() }; let
        total_bytes = { let mut tcp = TcpStream::connect(addr).unwrap(); tcp
        .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .unwrap(); let mut buf = Vec::new(); tcp.read_to_end(& mut buf).unwrap() }; let
        mut tcp = TcpStream::connect(addr).unwrap(); tcp
        .set_read_timeout(Some(Duration::from_secs(3))).unwrap(); let mut buf = [0u8;
        8192]; $b .bytes = 35 + total_bytes as u64; $b .iter(|| { tcp
        .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap(); let mut sum =
        0; while sum < total_bytes { sum += tcp.read(& mut buf).unwrap(); }
        assert_eq!(sum, total_bytes); }); }
    };
}
fn body(b: &'static [u8]) -> hyper::Body {
    loop {}
}
#[bench]
fn throughput_fixedsize_small_payload(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn throughput_fixedsize_large_payload(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn throughput_fixedsize_many_chunks(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn throughput_chunked_small_payload(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn throughput_chunked_large_payload(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn throughput_chunked_many_chunks(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn raw_tcp_throughput_small_payload(b: &mut test::Bencher) {
    loop {}
}
#[bench]
fn raw_tcp_throughput_large_payload(b: &mut test::Bencher) {
    loop {}
}
