#![feature(test)]
#![deny(warnings)]
extern crate test;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::time::Duration;
use tokio::sync::oneshot;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server};
const PIPELINED_REQUESTS: usize = 16;
#[bench]
fn hello_world_16(b: &mut test::Bencher) {
    loop {}
}
