use std::collections::VecDeque;
use std::io::IoSlice;
use bytes::{Buf, BufMut, Bytes, BytesMut};
pub(crate) struct BufList<T> {
    bufs: VecDeque<T>,
}
impl<T: Buf> BufList<T> {
    pub(crate) fn new() -> BufList<T> {
        loop {}
    }
    #[inline]
    pub(crate) fn push(&mut self, buf: T) {
        loop {}
    }
    #[inline]
    #[cfg(feature = "http1")]
    pub(crate) fn bufs_cnt(&self) -> usize {
        loop {}
    }
}
impl<T: Buf> Buf for BufList<T> {
    #[inline]
    fn remaining(&self) -> usize {
        loop {}
    }
    #[inline]
    fn chunk(&self) -> &[u8] {
        loop {}
    }
    #[inline]
    fn advance(&mut self, mut cnt: usize) {
        loop {}
    }
    #[inline]
    fn chunks_vectored<'t>(&'t self, dst: &mut [IoSlice<'t>]) -> usize {
        loop {}
    }
    #[inline]
    fn copy_to_bytes(&mut self, len: usize) -> Bytes {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use std::ptr;
    use super::*;
    fn hello_world_buf() -> BufList<Bytes> {
        loop {}
    }
    #[test]
    fn to_bytes_shorter() {
        loop {}
    }
    #[test]
    fn to_bytes_eq() {
        loop {}
    }
    #[test]
    fn to_bytes_longer() {
        loop {}
    }
    #[test]
    fn one_long_buf_to_bytes() {
        loop {}
    }
    #[test]
    #[should_panic(expected = "`len` greater than remaining")]
    fn buf_to_bytes_too_many() {
        loop {}
    }
}
