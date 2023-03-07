use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt;
type BoxError = Box<dyn std::error::Error + Send + Sync>;
/// Errors that can happen inside warp.
pub struct Error {
    inner: BoxError,
}
impl Error {
    pub(crate) fn new<E: Into<BoxError>>(err: E) -> Error {
        loop {}
    }
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        loop {}
    }
}
impl From<Infallible> for Error {
    fn from(infallible: Infallible) -> Error {
        loop {}
    }
}
#[test]
fn error_size_of() {
    loop {}
}
#[test]
fn error_source() {
    loop {}
}
macro_rules! unit_error {
    ($(#[$docs:meta])* $pub:vis $typ:ident : $display:literal) => {
        $(#[$docs])* $pub struct $typ { _p : (), } impl ::std::fmt::Debug for $typ { fn
        fmt(& self, f : & mut ::std::fmt::Formatter <'_ >) -> ::std::fmt::Result { f
        .debug_struct(stringify!($typ)).finish() } } impl ::std::fmt::Display for $typ {
        fn fmt(& self, f : & mut ::std::fmt::Formatter <'_ >) -> ::std::fmt::Result { f
        .write_str($display) } } impl ::std::error::Error for $typ {}
    };
}
