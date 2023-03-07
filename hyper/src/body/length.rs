use std::fmt;
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct DecodedLength(u64);
#[cfg(any(feature = "http1", feature = "http2"))]
impl From<Option<u64>> for DecodedLength {
    fn from(len: Option<u64>) -> Self {
        loop {}
    }
}
#[cfg(any(feature = "http1", feature = "http2", test))]
const MAX_LEN: u64 = std::u64::MAX - 2;
impl DecodedLength {
    pub(crate) const CLOSE_DELIMITED: DecodedLength = DecodedLength(::std::u64::MAX);
    pub(crate) const CHUNKED: DecodedLength = DecodedLength(::std::u64::MAX - 1);
    pub(crate) const ZERO: DecodedLength = DecodedLength(0);
    #[cfg(test)]
    pub(crate) fn new(len: u64) -> Self {
        loop {}
    }
    /// Takes the length as a content-length without other checks.
    ///
    /// Should only be called if previously confirmed this isn't
    /// CLOSE_DELIMITED or CHUNKED.
    #[inline]
    #[cfg(feature = "http1")]
    pub(crate) fn danger_len(self) -> u64 {
        loop {}
    }
    /// Converts to an Option<u64> representing a Known or Unknown length.
    pub(crate) fn into_opt(self) -> Option<u64> {
        loop {}
    }
    /// Checks the `u64` is within the maximum allowed for content-length.
    #[cfg(any(feature = "http1", feature = "http2"))]
    pub(crate) fn checked_new(len: u64) -> Result<Self, crate::error::Parse> {
        loop {}
    }
    pub(crate) fn sub_if(&mut self, amt: u64) {
        loop {}
    }
    /// Returns whether this represents an exact length.
    ///
    /// This includes 0, which of course is an exact known length.
    ///
    /// It would return false if "chunked" or otherwise size-unknown.
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sub_if_known() {
        loop {}
    }
    #[test]
    fn sub_if_chunked() {
        loop {}
    }
}
