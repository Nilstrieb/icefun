#[cfg(feature = "http1")]
use super::Never;
pub(crate) use std::task::{Context, Poll};



#[cfg(feature = "http1")]
pub(crate) fn yield_now(cx: &mut Context<'_>) -> Poll<Never> {
    loop {}
}
