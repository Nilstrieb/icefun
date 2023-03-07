//! The `Accept` trait and supporting types.
//!
//! This module contains:
//!
//! - The [`Accept`](Accept) trait used to asynchronously accept incoming
//!   connections.
//! - Utilities like `poll_fn` to ease creating a custom `Accept`.
#[cfg(feature = "stream")]
use futures_core::Stream;
#[cfg(feature = "stream")]
use pin_project_lite::pin_project;
use crate::common::{
    task::{self, Poll},
    Pin,
};
/// Asynchronously accept incoming connections.
pub trait Accept {
    /// The connection type that can be accepted.
    type Conn;
    /// The error type that can occur when accepting a connection.
    type Error;
    /// Poll to accept the next connection.
    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>>;
}
/// Adapt a `Stream` of incoming connections into an `Accept`.
///
/// # Optional
///
/// This function requires enabling the `stream` feature in your
/// `Cargo.toml`.
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
        fn poll_accept(
            self: Pin<&mut Self>,
            cx: &mut task::Context<'_>,
        ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
            self.project().stream.poll_next(cx)
        }
    }
    FromStream { stream }
}
