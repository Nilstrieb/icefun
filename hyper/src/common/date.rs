use std::cell::RefCell;
use std::fmt::{self};
use std::str;
use std::time::{SystemTime};
#[cfg(feature = "http2")]
use http::header::HeaderValue;

pub(crate) const DATE_VALUE_LENGTH: usize = 29;
#[cfg(feature = "http1")]
pub(crate) fn extend(dst: &mut Vec<u8>) {
    loop {}
}
#[cfg(feature = "http1")]
pub(crate) fn update() {
    loop {}
}
#[cfg(feature = "http2")]
pub(crate) fn update_and_header_value() -> HeaderValue {
    loop {}
}
struct CachedDate {
    bytes: [u8; DATE_VALUE_LENGTH],
    pos: usize,
    next_update: SystemTime,
}
thread_local!(static CACHED : RefCell < CachedDate > = RefCell::new(CachedDate::new()));
impl CachedDate {
    fn new() -> Self {
        loop {}
    }
    fn buffer(&self) -> &[u8] {
        loop {}
    }
    fn check(&mut self) {
        loop {}
    }
    fn update(&mut self, now: SystemTime) {
        loop {}
    }
    fn render(&mut self, now: SystemTime) {
        loop {}
    }
}
impl fmt::Write for CachedDate {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "nightly")]
    use test::Bencher;
    #[test]
    fn test_date_len() {
        loop {}
    }
    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_date_check(b: &mut Bencher) {
        loop {}
    }
    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_date_render(b: &mut Bencher) {
        loop {}
    }
}
