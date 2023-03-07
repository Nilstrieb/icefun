use std::cmp;
use std::fmt;
#[cfg(all(feature = "server", feature = "runtime"))]
use std::future::Future;
use std::io::{self, IoSlice};
use std::marker::Unpin;
use std::mem::MaybeUninit;
#[cfg(all(feature = "server", feature = "runtime"))]
use std::time::Duration;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
#[cfg(all(feature = "server", feature = "runtime"))]
use tokio::time::Instant;
use tracing::{debug, trace};
use super::{Http1Transaction, ParseContext, ParsedMessage};
use crate::common::buf::BufList;
use crate::common::{task, Pin, Poll};
/// The initial buffer size allocated before trying to read from IO.
pub(crate) const INIT_BUFFER_SIZE: usize = 8192;
/// The minimum value that can be set to max buffer size.
pub(crate) const MINIMUM_MAX_BUFFER_SIZE: usize = INIT_BUFFER_SIZE;
/// The default maximum read buffer size. If the buffer gets this big and
/// a message is still not complete, a `TooLarge` error is triggered.
pub(crate) const DEFAULT_MAX_BUFFER_SIZE: usize = 8192 + 4096 * 100;
/// The maximum number of distinct `Buf`s to hold in a list before requiring
/// a flush. Only affects when the buffer strategy is to queue buffers.
///
/// Note that a flush can happen before reaching the maximum. This simply
/// forces a flush if the queue gets this big.
const MAX_BUF_LIST_BUFFERS: usize = 16;
pub(crate) struct Buffered<T, B> {
    flush_pipeline: bool,
    io: T,
    read_blocked: bool,
    read_buf: BytesMut,
    read_buf_strategy: ReadStrategy,
    write_buf: WriteBuf<B>,
}
impl<T, B> fmt::Debug for Buffered<T, B>
where
    B: Buf,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<T, B> Buffered<T, B>
where
    T: AsyncRead + AsyncWrite + Unpin,
    B: Buf,
{
    pub(crate) fn new(io: T) -> Buffered<T, B> {
        loop {}
    }
    #[cfg(feature = "server")]
    pub(crate) fn set_flush_pipeline(&mut self, enabled: bool) {
        loop {}
    }
    pub(crate) fn set_max_buf_size(&mut self, max: usize) {
        loop {}
    }
    #[cfg(feature = "client")]
    pub(crate) fn set_read_buf_exact_size(&mut self, sz: usize) {
        loop {}
    }
    pub(crate) fn set_write_strategy_flatten(&mut self) {
        loop {}
    }
    pub(crate) fn set_write_strategy_queue(&mut self) {
        loop {}
    }
    pub(crate) fn read_buf(&self) -> &[u8] {
        loop {}
    }
    #[cfg(test)]
    #[cfg(feature = "nightly")]
    pub(super) fn read_buf_mut(&mut self) -> &mut BytesMut {
        loop {}
    }
    /// Return the "allocated" available space, not the potential space
    /// that could be allocated in the future.
    fn read_buf_remaining_mut(&self) -> usize {
        loop {}
    }
    /// Return whether we can append to the headers buffer.
    ///
    /// Reasons we can't:
    /// - The write buf is in queue mode, and some of the past body is still
    ///   needing to be flushed.
    pub(crate) fn can_headers_buf(&self) -> bool {
        loop {}
    }
    pub(crate) fn headers_buf(&mut self) -> &mut Vec<u8> {
        loop {}
    }
    pub(super) fn write_buf(&mut self) -> &mut WriteBuf<B> {
        loop {}
    }
    pub(crate) fn buffer<BB: Buf + Into<B>>(&mut self, buf: BB) {
        loop {}
    }
    pub(crate) fn can_buffer(&self) -> bool {
        loop {}
    }
    pub(crate) fn consume_leading_lines(&mut self) {
        loop {}
    }
    pub(super) fn parse<S>(
        &mut self,
        cx: &mut task::Context<'_>,
        parse_ctx: ParseContext<'_>,
    ) -> Poll<crate::Result<ParsedMessage<S::Incoming>>>
    where
        S: Http1Transaction,
    {
        loop {}
    }
    pub(crate) fn poll_read_from_io(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<io::Result<usize>> {
        loop {}
    }
    pub(crate) fn into_inner(self) -> (T, Bytes) {
        loop {}
    }
    pub(crate) fn io_mut(&mut self) -> &mut T {
        loop {}
    }
    pub(crate) fn is_read_blocked(&self) -> bool {
        loop {}
    }
    pub(crate) fn poll_flush(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        loop {}
    }
    /// Specialized version of `flush` when strategy is Flatten.
    ///
    /// Since all buffered bytes are flattened into the single headers buffer,
    /// that skips some bookkeeping around using multiple buffers.
    fn poll_flush_flattened(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        loop {}
    }
    #[cfg(test)]
    fn flush<'a>(
        &'a mut self,
    ) -> impl std::future::Future<Output = io::Result<()>> + 'a {
        loop {}
    }
}
impl<T: Unpin, B> Unpin for Buffered<T, B> {}
pub(crate) trait MemRead {
    fn read_mem(
        &mut self,
        cx: &mut task::Context<'_>,
        len: usize,
    ) -> Poll<io::Result<Bytes>>;
}
impl<T, B> MemRead for Buffered<T, B>
where
    T: AsyncRead + AsyncWrite + Unpin,
    B: Buf,
{
    fn read_mem(
        &mut self,
        cx: &mut task::Context<'_>,
        len: usize,
    ) -> Poll<io::Result<Bytes>> {
        loop {}
    }
}
#[derive(Clone, Copy, Debug)]
enum ReadStrategy {
    Adaptive { decrease_now: bool, next: usize, max: usize },
    #[cfg(feature = "client")]
    Exact(usize),
}
impl ReadStrategy {
    fn with_max(max: usize) -> ReadStrategy {
        loop {}
    }
    fn next(&self) -> usize {
        loop {}
    }
    fn max(&self) -> usize {
        loop {}
    }
    fn record(&mut self, bytes_read: usize) {
        loop {}
    }
}
fn incr_power_of_two(n: usize) -> usize {
    loop {}
}
fn prev_power_of_two(n: usize) -> usize {
    loop {}
}
impl Default for ReadStrategy {
    fn default() -> ReadStrategy {
        loop {}
    }
}
#[derive(Clone)]
pub(crate) struct Cursor<T> {
    bytes: T,
    pos: usize,
}
impl<T: AsRef<[u8]>> Cursor<T> {
    #[inline]
    pub(crate) fn new(bytes: T) -> Cursor<T> {
        loop {}
    }
}
impl Cursor<Vec<u8>> {
    /// If we've advanced the position a bit in this cursor, and wish to
    /// extend the underlying vector, we may wish to unshift the "read" bytes
    /// off, and move everything else over.
    fn maybe_unshift(&mut self, additional: usize) {
        loop {}
    }
    fn reset(&mut self) {
        loop {}
    }
}
impl<T: AsRef<[u8]>> fmt::Debug for Cursor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<T: AsRef<[u8]>> Buf for Cursor<T> {
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
pub(super) struct WriteBuf<B> {
    /// Re-usable buffer that holds message headers
    headers: Cursor<Vec<u8>>,
    max_buf_size: usize,
    /// Deque of user buffers if strategy is Queue
    queue: BufList<B>,
    strategy: WriteStrategy,
}
impl<B: Buf> WriteBuf<B> {
    fn new(strategy: WriteStrategy) -> WriteBuf<B> {
        loop {}
    }
}
impl<B> WriteBuf<B>
where
    B: Buf,
{
    fn set_strategy(&mut self, strategy: WriteStrategy) {
        loop {}
    }
    pub(super) fn buffer<BB: Buf + Into<B>>(&mut self, mut buf: BB) {
        loop {}
    }
    fn can_buffer(&self) -> bool {
        loop {}
    }
    fn headers_mut(&mut self) -> &mut Cursor<Vec<u8>> {
        loop {}
    }
}
impl<B: Buf> fmt::Debug for WriteBuf<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<B: Buf> Buf for WriteBuf<B> {
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
#[derive(Debug)]
enum WriteStrategy {
    Flatten,
    Queue,
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio_test::io::Builder as Mock;
    #[tokio::test]
    #[ignore]
    async fn iobuf_write_empty_slice() {}
    #[tokio::test]
    async fn parse_reads_until_blocked() {
        loop {}
    }
    #[test]
    fn read_strategy_adaptive_increments() {
        loop {}
    }
    #[test]
    fn read_strategy_adaptive_decrements() {
        loop {}
    }
    #[test]
    fn read_strategy_adaptive_stays_the_same() {
        loop {}
    }
    #[test]
    fn read_strategy_adaptive_max_fuzz() {
        loop {}
    }
    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn write_buf_requires_non_empty_bufs() {
        loop {}
    }
    #[tokio::test]
    async fn write_buf_flatten() {
        loop {}
    }
    #[test]
    fn write_buf_flatten_partially_flushed() {
        loop {}
    }
    #[tokio::test]
    async fn write_buf_queue_disable_auto() {
        loop {}
    }
}
