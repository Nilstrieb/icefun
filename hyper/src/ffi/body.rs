use std::ffi::c_void;
use std::mem::ManuallyDrop;
use std::ptr;
use std::task::{Context, Poll};
use http::HeaderMap;
use libc::{c_int, size_t};
use super::task::{hyper_context, hyper_task, hyper_task_return_type, AsTaskType};
use super::{UserDataPointer, HYPER_ITER_CONTINUE};
use crate::body::{Body, Bytes, HttpBody as _};

pub(crate) struct hyper_body(pub(super) Body);

pub(crate) struct hyper_buf(pub(crate) Bytes);
pub(crate) struct UserBody {
    data_func: hyper_body_data_callback,
    userdata: *mut c_void,
}
type hyper_body_foreach_callback = extern "C" fn(*mut c_void, *const hyper_buf) -> c_int;
type hyper_body_data_callback = extern "C" fn(
    *mut c_void,
    *mut hyper_context<'_>,
    *mut *mut hyper_buf,
) -> c_int;
ffi_fn! {
    #[doc = " Create a new \"empty\" body."] #[doc = ""] #[doc =
    " If not configured, this body acts as an empty payload."] fn hyper_body_new() -> *
    mut hyper_body { Box::into_raw(Box::new(hyper_body(Body::empty()))) } ?=
    ptr::null_mut()
}
ffi_fn! {
    #[doc = " Free a `hyper_body *`."] fn hyper_body_free(body : * mut hyper_body) {
    drop(non_null!(Box::from_raw(body) ?= ())); }
}
ffi_fn! {
    #[doc = " Return a task that will poll the body for the next buffer of data."] #[doc
    = ""] #[doc = " The task value may have different types depending on the outcome:"]
    #[doc = ""] #[doc = " - `HYPER_TASK_BUF`: Success, and more data was received."]
    #[doc = " - `HYPER_TASK_ERROR`: An error retrieving the data."] #[doc =
    " - `HYPER_TASK_EMPTY`: The body has finished streaming data."] #[doc = ""] #[doc =
    " This does not consume the `hyper_body *`, so it may be used to again."] #[doc =
    " However, it MUST NOT be used or freed until the related task completes."] fn
    hyper_body_data(body : * mut hyper_body) -> * mut hyper_task { let mut body =
    ManuallyDrop::new(non_null!(Box::from_raw(body) ?= ptr::null_mut()));
    Box::into_raw(hyper_task::boxed(async move { body.0.data().await.map(| res | res
    .map(hyper_buf)) })) } ?= ptr::null_mut()
}
ffi_fn! {
    #[doc = " Return a task that will poll the body and execute the callback with each"]
    #[doc = " body chunk that is received."] #[doc = ""] #[doc =
    " The `hyper_buf` pointer is only a borrowed reference, it cannot live outside"]
    #[doc = " the execution of the callback. You must make a copy to retain it."] #[doc =
    ""] #[doc =
    " The callback should return `HYPER_ITER_CONTINUE` to continue iterating"] #[doc =
    " chunks as they are received, or `HYPER_ITER_BREAK` to cancel."] #[doc = ""] #[doc =
    " This will consume the `hyper_body *`, you shouldn't use it anymore or free it."] fn
    hyper_body_foreach(body : * mut hyper_body, func : hyper_body_foreach_callback,
    userdata : * mut c_void) -> * mut hyper_task { let mut body =
    non_null!(Box::from_raw(body) ?= ptr::null_mut()); let userdata =
    UserDataPointer(userdata); Box::into_raw(hyper_task::boxed(async move { while let
    Some(item) = body.0.data().await { let chunk = item ?; if HYPER_ITER_CONTINUE !=
    func(userdata.0, & hyper_buf(chunk)) { return Err(crate
    ::Error::new_user_aborted_by_callback()); } } Ok(()) })) } ?= ptr::null_mut()
}
ffi_fn! {
    #[doc = " Set userdata on this body, which will be passed to callback functions."] fn
    hyper_body_set_userdata(body : * mut hyper_body, userdata : * mut c_void) { let b =
    non_null!(& mut * body ?= ()); b.0.as_ffi_mut().userdata = userdata; }
}
ffi_fn! {
    #[doc = " Set the data callback for this body."] #[doc = ""] #[doc =
    " The callback is called each time hyper needs to send more data for the"] #[doc =
    " body. It is passed the value from `hyper_body_set_userdata`."] #[doc = ""] #[doc =
    " If there is data available, the `hyper_buf **` argument should be set"] #[doc =
    " to a `hyper_buf *` containing the data, and `HYPER_POLL_READY` should"] #[doc =
    " be returned."] #[doc = ""] #[doc =
    " Returning `HYPER_POLL_READY` while the `hyper_buf **` argument points"] #[doc =
    " to `NULL` will indicate the body has completed all data."] #[doc = ""] #[doc =
    " If there is more data to send, but it isn't yet available, a"] #[doc =
    " `hyper_waker` should be saved from the `hyper_context *` argument, and"] #[doc =
    " `HYPER_POLL_PENDING` should be returned. You must wake the saved waker"] #[doc =
    " to signal the task when data is available."] #[doc = ""] #[doc =
    " If some error has occurred, you can return `HYPER_POLL_ERROR` to abort"] #[doc =
    " the body."] fn hyper_body_set_data_func(body : * mut hyper_body, func :
    hyper_body_data_callback) { let b = non_null! { & mut * body ?= () }; b.0
    .as_ffi_mut().data_func = func; }
}
impl UserBody {
    pub(crate) fn new() -> UserBody {
        loop {}
    }
    pub(crate) fn poll_data(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<crate::Result<Bytes>>> {
        loop {}
    }
    pub(crate) fn poll_trailers(
        &mut self,
        _cx: &mut Context<'_>,
    ) -> Poll<crate::Result<Option<HeaderMap>>> {
        loop {}
    }
}

extern "C" fn data_noop(
    _userdata: *mut c_void,
    _: *mut hyper_context<'_>,
    _: *mut *mut hyper_buf,
) -> c_int {
    loop {}
}
unsafe impl Send for UserBody {}
unsafe impl Sync for UserBody {}
ffi_fn! {
    #[doc = " Create a new `hyper_buf *` by copying the provided bytes."] #[doc = ""]
    #[doc = " This makes an owned copy of the bytes, so the `buf` argument can be"] #[doc
    = " freed or changed afterwards."] #[doc = ""] #[doc =
    " This returns `NULL` if allocating a new buffer fails."] fn hyper_buf_copy(buf : *
    const u8, len : size_t) -> * mut hyper_buf { let slice = unsafe {
    std::slice::from_raw_parts(buf, len) };
    Box::into_raw(Box::new(hyper_buf(Bytes::copy_from_slice(slice)))) } ?=
    ptr::null_mut()
}
ffi_fn! {
    #[doc = " Get a pointer to the bytes in this buffer."] #[doc = ""] #[doc =
    " This should be used in conjunction with `hyper_buf_len` to get the length"] #[doc =
    " of the bytes data."] #[doc = ""] #[doc =
    " This pointer is borrowed data, and not valid once the `hyper_buf` is"] #[doc =
    " consumed/freed."] fn hyper_buf_bytes(buf : * const hyper_buf) -> * const u8 {
    unsafe { (* buf).0.as_ptr() } } ?= ptr::null()
}
ffi_fn! {
    #[doc = " Get the length of the bytes this buffer contains."] fn hyper_buf_len(buf :
    * const hyper_buf) -> size_t { unsafe { (* buf).0.len() } }
}
ffi_fn! {
    #[doc = " Free this buffer."] fn hyper_buf_free(buf : * mut hyper_buf) { drop(unsafe
    { Box::from_raw(buf) }); }
}
unsafe impl AsTaskType for hyper_buf {
    fn as_task_type(&self) -> hyper_task_return_type {
        loop {}
    }
}
