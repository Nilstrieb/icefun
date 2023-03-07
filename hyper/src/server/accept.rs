//! The `Accept` trait and supporting types.
//!
//! This module contains:
//!
//! - The [`Accept`](Accept) trait used to asynchronously accept incoming
//!   connections.
//! - Utilities like `poll_fn` to ease creating a custom `Accept`.
use crate::common::{
    task::{self, Poll},
    Pin,
};
#[cfg(feature = "stream")]
use futures_core::Stream;
#[cfg(feature = "stream")]
use pin_project_lite::pin_project;

pub trait Accept {
    type Conn;

    type Error;
}

#[cfg(feature = "stream")]
pub fn from_stream<S, IO, E>(stream: S) -> impl Accept<Conn = IO, Error = E>
where
    S: Stream<Item = Result<IO, E>>,
{
    pin_project! {
        struct FromStream < S > { #[pin] stream : S, }
    }
    impl<S, IO, E> Accept for FromStream<S>
    where
        S: Stream<Item = Result<IO, E>>,
    {
        type Conn = IO;
        type Error = E;
    }
    FromStream { stream }
}
