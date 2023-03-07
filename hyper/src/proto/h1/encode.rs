use std::fmt;
use std::io::IoSlice;
use bytes::buf::{Chain, Take};
use bytes::Buf;

use super::io::WriteBuf;
type StaticBuf = &'static [u8];

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Encoder {
    kind: Kind,
    is_last: bool,
}
#[derive(Debug)]
pub(crate) struct EncodedBuf<B> {
    kind: BufKind<B>,
}
#[derive(Debug)]
pub(crate) struct NotEof(u64);
#[derive(Debug, PartialEq, Clone)]
enum Kind {
    
    Chunked,
    
    
    
    Length(u64),
    
    
    
    
    #[cfg(feature = "server")]
    CloseDelimited,
}
#[derive(Debug)]
enum BufKind<B> {
    Exact(B),
    Limited(Take<B>),
    Chunked(Chain<Chain<ChunkSize, B>, StaticBuf>),
    ChunkedEnd(StaticBuf),
}
impl Encoder {
    fn new(kind: Kind) -> Encoder {
        loop {}
    }
    pub(crate) fn chunked() -> Encoder {
        loop {}
    }
    pub(crate) fn length(len: u64) -> Encoder {
        loop {}
    }
    #[cfg(feature = "server")]
    pub(crate) fn close_delimited() -> Encoder {
        loop {}
    }
    pub(crate) fn is_eof(&self) -> bool {
        loop {}
    }
    #[cfg(feature = "server")]
    pub(crate) fn set_last(mut self, is_last: bool) -> Self {
        loop {}
    }
    pub(crate) fn is_last(&self) -> bool {
        loop {}
    }
    pub(crate) fn is_close_delimited(&self) -> bool {
        loop {}
    }
    pub(crate) fn end<B>(&self) -> Result<Option<EncodedBuf<B>>, NotEof> {
        loop {}
    }
    pub(crate) fn encode<B>(&mut self, msg: B) -> EncodedBuf<B>
    where
        B: Buf,
    {
        loop {}
    }
    pub(super) fn encode_and_end<B>(
        &self,
        msg: B,
        dst: &mut WriteBuf<EncodedBuf<B>>,
    ) -> bool
    where
        B: Buf,
    {
        loop {}
    }
    
    
    
    
    
    pub(super) fn danger_full_buf<B>(self, msg: B, dst: &mut WriteBuf<EncodedBuf<B>>)
    where
        B: Buf,
    {
        loop {}
    }
}
impl<B> Buf for EncodedBuf<B>
where
    B: Buf,
{
    #[inline]
    fn remaining(&self) -> usize {
        loop {}
    }
    #[inline]
    fn chunk(&self) -> &[u8] {
        loop {}
    }
    #[inline]
    fn advance(&mut self, cnt: usize) {
        loop {}
    }
    #[inline]
    fn chunks_vectored<'t>(&'t self, dst: &mut [IoSlice<'t>]) -> usize {
        loop {}
    }
}
#[cfg(target_pointer_width = "32")]
const USIZE_BYTES: usize = 4;
#[cfg(target_pointer_width = "64")]
const USIZE_BYTES: usize = 8;
const CHUNK_SIZE_MAX_BYTES: usize = USIZE_BYTES * 2;
#[derive(Clone, Copy)]
struct ChunkSize {
    bytes: [u8; CHUNK_SIZE_MAX_BYTES + 2],
    pos: u8,
    len: u8,
}
impl ChunkSize {
    fn new(len: usize) -> ChunkSize {
        loop {}
    }
}
impl Buf for ChunkSize {
    #[inline]
    fn remaining(&self) -> usize {
        loop {}
    }
    #[inline]
    fn chunk(&self) -> &[u8] {
        loop {}
    }
    #[inline]
    fn advance(&mut self, cnt: usize) {
        loop {}
    }
}
impl fmt::Debug for ChunkSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl fmt::Write for ChunkSize {
    fn write_str(&mut self, num: &str) -> fmt::Result {
        loop {}
    }
}
impl<B: Buf> From<B> for EncodedBuf<B> {
    fn from(buf: B) -> Self {
        loop {}
    }
}
impl<B: Buf> From<Take<B>> for EncodedBuf<B> {
    fn from(buf: Take<B>) -> Self {
        loop {}
    }
}
impl<B: Buf> From<Chain<Chain<ChunkSize, B>, StaticBuf>> for EncodedBuf<B> {
    fn from(buf: Chain<Chain<ChunkSize, B>, StaticBuf>) -> Self {
        loop {}
    }
}
impl fmt::Display for NotEof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl std::error::Error for NotEof {}
#[cfg(test)]
mod tests {
    use bytes::BufMut;
    use super::super::io::Cursor;
    use super::Encoder;
    #[test]
    fn chunked() {
        loop {}
    }
    #[test]
    fn length() {
        loop {}
    }
    #[test]
    fn eof() {
        loop {}
    }
}
