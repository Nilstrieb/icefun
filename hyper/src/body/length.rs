use std::fmt;
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct DecodedLength(u64);
#[cfg(any(feature = "http1", feature = "http2"))]
impl From<Option<u64>> for DecodedLength {
    fn from(len: Option<u64>) -> Self {
        loop {}
    }
}
impl DecodedLength {
    pub(crate) const CLOSE_DELIMITED: DecodedLength = DecodedLength(::std::u64::MAX);
    pub(crate) const CHUNKED: DecodedLength = DecodedLength(::std::u64::MAX - 1);
    pub(crate) const ZERO: DecodedLength = DecodedLength(0);
    #[cfg(test)]
    pub(crate) fn new(len: u64) -> Self {
        loop {}
    }
    
    
    
    
    #[inline]
    #[cfg(feature = "http1")]
    pub(crate) fn danger_len(self) -> u64 {
        loop {}
    }
    
    pub(crate) fn into_opt(self) -> Option<u64> {
        loop {}
    }
    
    #[cfg(any(feature = "http1", feature = "http2"))]
    pub(crate) fn checked_new(len: u64) -> Result<Self, crate::error::Parse> {
        loop {}
    }
    pub(crate) fn sub_if(&mut self, amt: u64) {
        loop {}
    }
    
    
    
    
    
    #[cfg(feature = "http2")]
    pub(crate) fn is_exact(&self) -> bool {
        loop {}
    }
}
impl fmt::Debug for DecodedLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl fmt::Display for DecodedLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
