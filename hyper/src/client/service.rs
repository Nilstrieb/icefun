//! Utilities used to interact with the Tower ecosystem.
//!
//! This module provides `Connect` which hook-ins into the Tower ecosystem.
use std::error::Error as StdError;
use std::future::Future;
use std::marker::PhantomData;

use super::conn::{Builder, SendRequest};
use crate::{
    body::HttpBody, common::{task, Pin, Poll},
    service::{MakeConnection, Service},
};





#[derive(Debug)]
pub(crate) struct Connect<C, B, T> {
    inner: C,
    builder: Builder,
    _pd: PhantomData<fn(T, B)>,
}
impl<C, B, T> Connect<C, B, T> {
    
    
    pub(crate) fn new(inner: C, builder: Builder) -> Self {
        loop {}
    }
}
impl<C, B, T> Service<T> for Connect<C, B, T>
where
    C: MakeConnection<T>,
    C::Connection: Unpin + Send + 'static,
    C::Future: Send + 'static,
    C::Error: Into<Box<dyn StdError + Send + Sync>> + Send,
    B: HttpBody + Unpin + Send + 'static,
    B::Data: Send + Unpin,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Response = SendRequest<B>;
    type Error = crate::Error;
    type Future = Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;
    fn poll_ready(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {}
    }
    fn call(&mut self, req: T) -> Self::Future {
        loop {}
    }
}
