#![allow(non_camel_case_types)]
#![allow(missing_debug_implementations)]
#![allow(unreachable_pub)]
//! # hyper C API
//!
//! This part of the documentation describes the C API for hyper. That is, how
//! to *use* the hyper library in C code. This is **not** a regular Rust
//! module, and thus it is not accessible in Rust.
//!
//! ## Unstable
//!
//! The C API of hyper is currently **unstable**, which means it's not part of
//! the semver contract as the rest of the Rust API is. Because of that, it's
//! only accessible if `--cfg hyper_unstable_ffi` is passed to `rustc` when
//! compiling. The easiest way to do that is setting the `RUSTFLAGS`
//! environment variable.
//!
//! ## Building
//!
//! The C API is part of the Rust library, but isn't compiled by default. Using
//! `cargo`, it can be compiled with the following command:
//!
//! ```notrust
//! RUSTFLAGS="--cfg hyper_unstable_ffi" cargo build --features client,http1,http2,ffi
//! ```
#[cfg(not(all(feature = "client", feature = "http1")))]
compile_error!(
    "The `ffi` feature currently requires the `client` and `http1` features."
);
#[cfg(not(hyper_unstable_ffi))]
compile_error!(
    "\
    The `ffi` feature is unstable, and requires the \
    `RUSTFLAGS='--cfg hyper_unstable_ffi'` environment variable to be set.\
"
);
#[macro_use]
mod macros;
mod body;
mod client;
mod error;
mod http_types;
mod io;
mod task;
pub(crate) use self::body::*;
pub(crate) use self::client::*;
pub(crate) use self::error::*;
pub(crate) use self::http_types::*;
pub(crate) use self::io::*;
pub(crate) use self::task::*;

pub(crate) const HYPER_ITER_CONTINUE: libc::c_int = 0;

#[allow(unused)]
pub(crate) const HYPER_ITER_BREAK: libc::c_int = 1;

pub(crate) const HYPER_HTTP_VERSION_NONE: libc::c_int = 0;

pub(crate) const HYPER_HTTP_VERSION_1_0: libc::c_int = 10;

pub(crate) const HYPER_HTTP_VERSION_1_1: libc::c_int = 11;

pub(crate) const HYPER_HTTP_VERSION_2: libc::c_int = 20;
struct UserDataPointer(*mut std::ffi::c_void);
unsafe impl Send for UserDataPointer {}
unsafe impl Sync for UserDataPointer {}

static VERSION_CSTR: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");
ffi_fn! {
    #[doc = " Returns a static ASCII (null terminated) string of the hyper version."] fn
    hyper_version() -> * const libc::c_char { VERSION_CSTR.as_ptr() as _ } ?=
    std::ptr::null()
}
