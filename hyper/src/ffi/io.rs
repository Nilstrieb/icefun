use std::ffi::c_void;
use std::pin::Pin;
use std::task::{Context, Poll};
use libc::size_t;
use tokio::io::{AsyncRead, AsyncWrite};
use super::task::hyper_context;
/// Sentinel value to return from a read or write callback that the operation
/// is pending.
pub(crate) const HYPER_IO_PENDING: size_t = 0xFFFFFFFF;
/// Sentinel value to return from a read or write callback that the operation
/// has errored.
pub(crate) const HYPER_IO_ERROR: size_t = 0xFFFFFFFE;
type hyper_io_read_callback = extern "C" fn(
    *mut c_void,
    *mut hyper_context<'_>,
    *mut u8,
    size_t,
) -> size_t;
type hyper_io_write_callback = extern "C" fn(
    *mut c_void,
    *mut hyper_context<'_>,
    *const u8,
    size_t,
) -> size_t;
/// An IO object used to represent a socket or similar concept.
pub(crate) struct hyper_io {
    read: hyper_io_read_callback,
    write: hyper_io_write_callback,
    userdata: *mut c_void,
}
ffi_fn! {
    #[doc = " Create a new IO type used to represent a transport."] #[doc = ""] #[doc =
    " The read and write functions of this transport should be set with"] #[doc =
    " `hyper_io_set_read` and `hyper_io_set_write`."] fn hyper_io_new() -> * mut hyper_io
    { Box::into_raw(Box::new(hyper_io { read : read_noop, write : write_noop, userdata :
    std::ptr::null_mut(), })) } ?= std::ptr::null_mut()
}
ffi_fn! {
    #[doc = " Free an unused `hyper_io *`."] #[doc = ""] #[doc =
    " This is typically only useful if you aren't going to pass ownership"] #[doc =
    " of the IO handle to hyper, such as with `hyper_clientconn_handshake()`."] fn
    hyper_io_free(io : * mut hyper_io) { drop(non_null!(Box::from_raw(io) ?= ())); }
}
ffi_fn! {
    #[doc = " Set the user data pointer for this IO to some value."] #[doc = ""] #[doc =
    " This value is passed as an argument to the read and write callbacks."] fn
    hyper_io_set_userdata(io : * mut hyper_io, data : * mut c_void) { non_null!(& mut *
    io ?= ()) .userdata = data; }
}
ffi_fn! {
    #[doc = " Set the read function for this IO transport."] #[doc = ""] #[doc =
    " Data that is read from the transport should be put in the `buf` pointer,"] #[doc =
    " up to `buf_len` bytes. The number of bytes read should be the return value."] #[doc
    = ""] #[doc =
    " It is undefined behavior to try to access the bytes in the `buf` pointer,"] #[doc =
    " unless you have already written them yourself. It is also undefined behavior"]
    #[doc =
    " to return that more bytes have been written than actually set on the `buf`."] #[doc
    = ""] #[doc =
    " If there is no data currently available, a waker should be claimed from"] #[doc =
    " the `ctx` and registered with whatever polling mechanism is used to signal"] #[doc
    = " when data is available later on. The return value should be"] #[doc =
    " `HYPER_IO_PENDING`."] #[doc = ""] #[doc =
    " If there is an irrecoverable error reading data, then `HYPER_IO_ERROR`"] #[doc =
    " should be the return value."] fn hyper_io_set_read(io : * mut hyper_io, func :
    hyper_io_read_callback) { non_null!(& mut * io ?= ()) .read = func; }
}
ffi_fn! {
    #[doc = " Set the write function for this IO transport."] #[doc = ""] #[doc =
    " Data from the `buf` pointer should be written to the transport, up to"] #[doc =
    " `buf_len` bytes. The number of bytes written should be the return value."] #[doc =
    ""] #[doc = " If no data can currently be written, the `waker` should be cloned and"]
    #[doc = " registered with whatever polling mechanism is used to signal when data"]
    #[doc = " is available later on. The return value should be `HYPER_IO_PENDING`."]
    #[doc = ""] #[doc = " Yeet."] #[doc = ""] #[doc =
    " If there is an irrecoverable error reading data, then `HYPER_IO_ERROR`"] #[doc =
    " should be the return value."] fn hyper_io_set_write(io : * mut hyper_io, func :
    hyper_io_write_callback) { non_null!(& mut * io ?= ()) .write = func; }
}
/// cbindgen:ignore
extern "C" fn read_noop(
    _userdata: *mut c_void,
    _: *mut hyper_context<'_>,
    _buf: *mut u8,
    _buf_len: size_t,
) -> size_t {
    loop {}
}
/// cbindgen:ignore
extern "C" fn write_noop(
    _userdata: *mut c_void,
    _: *mut hyper_context<'_>,
    _buf: *const u8,
    _buf_len: size_t,
) -> size_t {
    loop {}
}
impl AsyncRead for hyper_io {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        loop {}
    }
}
impl AsyncWrite for hyper_io {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        loop {}
    }
    fn poll_flush(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        loop {}
    }
    fn poll_shutdown(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        loop {}
    }
}
unsafe impl Send for hyper_io {}
unsafe impl Sync for hyper_io {}
