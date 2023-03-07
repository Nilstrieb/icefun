use bytes::Bytes;
use libc::{c_int, size_t};
use std::ffi::c_void;
use super::body::{hyper_body, hyper_buf};
use super::error::hyper_code;
use super::task::{hyper_task_return_type, AsTaskType};
use super::{UserDataPointer, HYPER_ITER_CONTINUE};
use crate::ext::{HeaderCaseMap, OriginalHeaderOrder, ReasonPhrase};
use crate::header::{HeaderName, HeaderValue};
use crate::{Body, HeaderMap, Method, Request, Response, Uri};

pub(crate) struct hyper_request(pub(super) Request<Body>);

pub(crate) struct hyper_response(pub(super) Response<Body>);



pub(crate) struct hyper_headers {
    pub(super) headers: HeaderMap,
    orig_casing: HeaderCaseMap,
    orig_order: OriginalHeaderOrder,
}
pub(crate) struct RawHeaders(pub(crate) hyper_buf);
pub(crate) struct OnInformational {
    func: hyper_request_on_informational_callback,
    data: UserDataPointer,
}
type hyper_request_on_informational_callback = extern "C" fn(
    *mut c_void,
    *mut hyper_response,
);
ffi_fn! {
    #[doc = " Construct a new HTTP request."] fn hyper_request_new() -> * mut
    hyper_request { Box::into_raw(Box::new(hyper_request(Request::new(Body::empty())))) }
    ?= std::ptr::null_mut()
}
ffi_fn! {
    #[doc = " Free an HTTP request if not going to send it on a client."] fn
    hyper_request_free(req : * mut hyper_request) { drop(non_null!(Box::from_raw(req) ?=
    ())); }
}
ffi_fn! {
    #[doc = " Set the HTTP Method of the request."] fn hyper_request_set_method(req : *
    mut hyper_request, method : * const u8, method_len : size_t) -> hyper_code { let
    bytes = unsafe { std::slice::from_raw_parts(method, method_len as usize) }; let req =
    non_null!(& mut * req ?= hyper_code::HYPERE_INVALID_ARG); match
    Method::from_bytes(bytes) { Ok(m) => { * req.0.method_mut() = m;
    hyper_code::HYPERE_OK }, Err(_) => { hyper_code::HYPERE_INVALID_ARG } } }
}
ffi_fn! {
    #[doc = " Set the URI of the request."] #[doc = ""] #[doc =
    " The request's URI is best described as the `request-target` from the RFCs. So in HTTP/1,"]
    #[doc =
    " whatever is set will get sent as-is in the first line (GET $uri HTTP/1.1). It"]
    #[doc =
    " supports the 4 defined variants, origin-form, absolute-form, authority-form, and"]
    #[doc = " asterisk-form."] #[doc = ""] #[doc =
    " The underlying type was built to efficiently support HTTP/2 where the request-target is"]
    #[doc =
    " split over :scheme, :authority, and :path. As such, each part can be set explicitly, or the"]
    #[doc =
    " type can parse a single contiguous string and if a scheme is found, that slot is \"set\". If"]
    #[doc =
    " the string just starts with a path, only the path portion is set. All pseudo headers that"]
    #[doc = " have been parsed/set are sent when the connection type is HTTP/2."] #[doc =
    ""] #[doc = " To set each slot explicitly, use `hyper_request_set_uri_parts`."] fn
    hyper_request_set_uri(req : * mut hyper_request, uri : * const u8, uri_len : size_t)
    -> hyper_code { let bytes = unsafe { std::slice::from_raw_parts(uri, uri_len as
    usize) }; let req = non_null!(& mut * req ?= hyper_code::HYPERE_INVALID_ARG); match
    Uri::from_maybe_shared(bytes) { Ok(u) => { * req.0.uri_mut() = u;
    hyper_code::HYPERE_OK }, Err(_) => { hyper_code::HYPERE_INVALID_ARG } } }
}
ffi_fn! {
    #[doc = " Set the URI of the request with separate scheme, authority, and"] #[doc =
    " path/query strings."] #[doc = ""] #[doc =
    " Each of `scheme`, `authority`, and `path_and_query` should either be"] #[doc =
    " null, to skip providing a component, or point to a UTF-8 encoded"] #[doc =
    " string. If any string pointer argument is non-null, its corresponding"] #[doc =
    " `len` parameter must be set to the string's length."] fn
    hyper_request_set_uri_parts(req : * mut hyper_request, scheme : * const u8,
    scheme_len : size_t, authority : * const u8, authority_len : size_t, path_and_query :
    * const u8, path_and_query_len : size_t) -> hyper_code { let mut builder =
    Uri::builder(); if ! scheme.is_null() { let scheme_bytes = unsafe {
    std::slice::from_raw_parts(scheme, scheme_len as usize) }; builder = builder
    .scheme(scheme_bytes); } if ! authority.is_null() { let authority_bytes = unsafe {
    std::slice::from_raw_parts(authority, authority_len as usize) }; builder = builder
    .authority(authority_bytes); } if ! path_and_query.is_null() { let
    path_and_query_bytes = unsafe { std::slice::from_raw_parts(path_and_query,
    path_and_query_len as usize) }; builder = builder
    .path_and_query(path_and_query_bytes); } match builder.build() { Ok(u) => { * unsafe
    { & mut * req } .0.uri_mut() = u; hyper_code::HYPERE_OK }, Err(_) => {
    hyper_code::HYPERE_INVALID_ARG } } }
}
ffi_fn! {
    #[doc = " Set the preferred HTTP version of the request."] #[doc = ""] #[doc =
    " The version value should be one of the `HYPER_HTTP_VERSION_` constants."] #[doc =
    ""] #[doc = " Note that this won't change the major HTTP version of the connection,"]
    #[doc = " since that is determined at the handshake step."] fn
    hyper_request_set_version(req : * mut hyper_request, version : c_int) -> hyper_code {
    use http::Version; let req = non_null!(& mut * req ?=
    hyper_code::HYPERE_INVALID_ARG); * req.0.version_mut() = match version {
    super::HYPER_HTTP_VERSION_NONE => Version::HTTP_11, super::HYPER_HTTP_VERSION_1_0 =>
    Version::HTTP_10, super::HYPER_HTTP_VERSION_1_1 => Version::HTTP_11,
    super::HYPER_HTTP_VERSION_2 => Version::HTTP_2, _ => { return
    hyper_code::HYPERE_INVALID_ARG; } }; hyper_code::HYPERE_OK }
}
ffi_fn! {
    #[doc = " Gets a reference to the HTTP headers of this request"] #[doc = ""] #[doc =
    " This is not an owned reference, so it should not be accessed after the"] #[doc =
    " `hyper_request` has been consumed."] fn hyper_request_headers(req : * mut
    hyper_request) -> * mut hyper_headers { hyper_headers::get_or_default(unsafe { & mut
    * req } .0.extensions_mut()) } ?= std::ptr::null_mut()
}
ffi_fn! {
    #[doc = " Set the body of the request."] #[doc = ""] #[doc =
    " The default is an empty body."] #[doc = ""] #[doc =
    " This takes ownership of the `hyper_body *`, you must not use it or"] #[doc =
    " free it after setting it on the request."] fn hyper_request_set_body(req : * mut
    hyper_request, body : * mut hyper_body) -> hyper_code { let body =
    non_null!(Box::from_raw(body) ?= hyper_code::HYPERE_INVALID_ARG); let req =
    non_null!(& mut * req ?= hyper_code::HYPERE_INVALID_ARG); * req.0.body_mut() = body
    .0; hyper_code::HYPERE_OK }
}
ffi_fn! {
    #[doc = " Set an informational (1xx) response callback."] #[doc = ""] #[doc =
    " The callback is called each time hyper receives an informational (1xx)"] #[doc =
    " response for this request."] #[doc = ""] #[doc =
    " The third argument is an opaque user data pointer, which is passed to"] #[doc =
    " the callback each time."] #[doc = ""] #[doc =
    " The callback is passed the `void *` data pointer, and a"] #[doc =
    " `hyper_response *` which can be inspected as any other response. The"] #[doc =
    " body of the response will always be empty."] #[doc = ""] #[doc =
    " NOTE: The `hyper_response *` is just borrowed data, and will not"] #[doc =
    " be valid after the callback finishes. You must copy any data you wish"] #[doc =
    " to persist."] fn hyper_request_on_informational(req : * mut hyper_request, callback
    : hyper_request_on_informational_callback, data : * mut c_void) -> hyper_code { let
    ext = OnInformational { func : callback, data : UserDataPointer(data), }; let req =
    non_null!(& mut * req ?= hyper_code::HYPERE_INVALID_ARG); req.0.extensions_mut()
    .insert(ext); hyper_code::HYPERE_OK }
}
impl hyper_request {
    pub(super) fn finalize_request(&mut self) {
        loop {}
    }
}
ffi_fn! {
    #[doc = " Free an HTTP response after using it."] fn hyper_response_free(resp : * mut
    hyper_response) { drop(non_null!(Box::from_raw(resp) ?= ())); }
}
ffi_fn! {
    #[doc = " Get the HTTP-Status code of this response."] #[doc = ""] #[doc =
    " It will always be within the range of 100-599."] fn hyper_response_status(resp : *
    const hyper_response) -> u16 { non_null!(&* resp ?= 0) .0.status().as_u16() }
}
ffi_fn! {
    #[doc = " Get a pointer to the reason-phrase of this response."] #[doc = ""] #[doc =
    " This buffer is not null-terminated."] #[doc = ""] #[doc =
    " This buffer is owned by the response, and should not be used after"] #[doc =
    " the response has been freed."] #[doc = ""] #[doc =
    " Use `hyper_response_reason_phrase_len()` to get the length of this"] #[doc =
    " buffer."] fn hyper_response_reason_phrase(resp : * const hyper_response) -> * const
    u8 { non_null!(&* resp ?= std::ptr::null()) .reason_phrase().as_ptr() } ?=
    std::ptr::null()
}
ffi_fn! {
    #[doc = " Get the length of the reason-phrase of this response."] #[doc = ""] #[doc =
    " Use `hyper_response_reason_phrase()` to get the buffer pointer."] fn
    hyper_response_reason_phrase_len(resp : * const hyper_response) -> size_t {
    non_null!(&* resp ?= 0) .reason_phrase().len() }
}
ffi_fn! {
    #[doc = " Get a reference to the full raw headers of this response."] #[doc = ""]
    #[doc = " You must have enabled `hyper_clientconn_options_headers_raw()`, or this"]
    #[doc = " will return NULL."] #[doc = ""] #[doc =
    " The returned `hyper_buf *` is just a reference, owned by the response."] #[doc =
    " You need to make a copy if you wish to use it after freeing the"] #[doc =
    " response."] #[doc = ""] #[doc =
    " The buffer is not null-terminated, see the `hyper_buf` functions for"] #[doc =
    " getting the bytes and length."] fn hyper_response_headers_raw(resp : * const
    hyper_response) -> * const hyper_buf { let resp = non_null!(&* resp ?=
    std::ptr::null()); match resp.0.extensions().get::< RawHeaders > () { Some(raw) => &
    raw.0, None => std::ptr::null(), } } ?= std::ptr::null()
}
ffi_fn! {
    #[doc = " Get the HTTP version used by this response."] #[doc = ""] #[doc =
    " The returned value could be:"] #[doc = ""] #[doc = " - `HYPER_HTTP_VERSION_1_0`"]
    #[doc = " - `HYPER_HTTP_VERSION_1_1`"] #[doc = " - `HYPER_HTTP_VERSION_2`"] #[doc =
    " - `HYPER_HTTP_VERSION_NONE` if newer (or older)."] fn hyper_response_version(resp :
    * const hyper_response) -> c_int { use http::Version; match non_null!(&* resp ?= 0)
    .0.version() { Version::HTTP_10 => super::HYPER_HTTP_VERSION_1_0, Version::HTTP_11 =>
    super::HYPER_HTTP_VERSION_1_1, Version::HTTP_2 => super::HYPER_HTTP_VERSION_2, _ =>
    super::HYPER_HTTP_VERSION_NONE, } }
}
ffi_fn! {
    #[doc = " Gets a reference to the HTTP headers of this response."] #[doc = ""] #[doc
    = " This is not an owned reference, so it should not be accessed after the"] #[doc =
    " `hyper_response` has been freed."] fn hyper_response_headers(resp : * mut
    hyper_response) -> * mut hyper_headers { hyper_headers::get_or_default(unsafe { & mut
    * resp } .0.extensions_mut()) } ?= std::ptr::null_mut()
}
ffi_fn! {
    #[doc = " Take ownership of the body of this response."] #[doc = ""] #[doc =
    " It is safe to free the response even after taking ownership of its body."] fn
    hyper_response_body(resp : * mut hyper_response) -> * mut hyper_body { let body =
    std::mem::take(non_null!(& mut * resp ?= std::ptr::null_mut()) .0.body_mut());
    Box::into_raw(Box::new(hyper_body(body))) } ?= std::ptr::null_mut()
}
impl hyper_response {
    pub(super) fn wrap(mut resp: Response<Body>) -> hyper_response {
        loop {}
    }
    fn reason_phrase(&self) -> &[u8] {
        loop {}
    }
}
unsafe impl AsTaskType for hyper_response {
    fn as_task_type(&self) -> hyper_task_return_type {
        loop {}
    }
}
type hyper_headers_foreach_callback = extern "C" fn(
    *mut c_void,
    *const u8,
    size_t,
    *const u8,
    size_t,
) -> c_int;
impl hyper_headers {
    pub(super) fn get_or_default(ext: &mut http::Extensions) -> &mut hyper_headers {
        loop {}
    }
}
ffi_fn! {
    #[doc = " Iterates the headers passing each name and value pair to the callback."]
    #[doc = ""] #[doc = " The `userdata` pointer is also passed to the callback."] #[doc
    = ""] #[doc =
    " The callback should return `HYPER_ITER_CONTINUE` to keep iterating, or"] #[doc =
    " `HYPER_ITER_BREAK` to stop."] fn hyper_headers_foreach(headers : * const
    hyper_headers, func : hyper_headers_foreach_callback, userdata : * mut c_void) { let
    headers = non_null!(&* headers ?= ()); let mut ordered_iter = headers.orig_order
    .get_in_order().peekable(); if ordered_iter.peek().is_some() { for (name, idx) in
    ordered_iter { let (name_ptr, name_len) = if let Some(orig_name) = headers
    .orig_casing.get_all(name).nth(* idx) { (orig_name.as_ref().as_ptr(), orig_name
    .as_ref().len()) } else { (name.as_str().as_bytes().as_ptr(), name.as_str()
    .as_bytes().len(),) }; let val_ptr; let val_len; if let Some(value) = headers.headers
    .get_all(name).iter().nth(* idx) { val_ptr = value.as_bytes().as_ptr(); val_len =
    value.as_bytes().len(); } else { return; } if HYPER_ITER_CONTINUE != func(userdata,
    name_ptr, name_len, val_ptr, val_len) { return; } } } else { for name in headers
    .headers.keys() { let mut names = headers.orig_casing.get_all(name); for value in
    headers.headers.get_all(name) { let (name_ptr, name_len) = if let Some(orig_name) =
    names.next() { (orig_name.as_ref().as_ptr(), orig_name.as_ref().len()) } else { (name
    .as_str().as_bytes().as_ptr(), name.as_str().as_bytes().len(),) }; let val_ptr =
    value.as_bytes().as_ptr(); let val_len = value.as_bytes().len(); if
    HYPER_ITER_CONTINUE != func(userdata, name_ptr, name_len, val_ptr, val_len) { return;
    } } } } }
}
ffi_fn! {
    #[doc = " Sets the header with the provided name to the provided value."] #[doc = ""]
    #[doc = " This overwrites any previous value set for the header."] fn
    hyper_headers_set(headers : * mut hyper_headers, name : * const u8, name_len :
    size_t, value : * const u8, value_len : size_t) -> hyper_code { let headers =
    non_null!(& mut * headers ?= hyper_code::HYPERE_INVALID_ARG); match unsafe {
    raw_name_value(name, name_len, value, value_len) } { Ok((name, value, orig_name)) =>
    { headers.headers.insert(& name, value); headers.orig_casing.insert(name.clone(),
    orig_name.clone()); headers.orig_order.insert(name); hyper_code::HYPERE_OK }
    Err(code) => code, } }
}
ffi_fn! {
    #[doc = " Adds the provided value to the list of the provided name."] #[doc = ""]
    #[doc = " If there were already existing values for the name, this will append the"]
    #[doc = " new value to the internal list."] fn hyper_headers_add(headers : * mut
    hyper_headers, name : * const u8, name_len : size_t, value : * const u8, value_len :
    size_t) -> hyper_code { let headers = non_null!(& mut * headers ?=
    hyper_code::HYPERE_INVALID_ARG); match unsafe { raw_name_value(name, name_len, value,
    value_len) } { Ok((name, value, orig_name)) => { headers.headers.append(& name,
    value); headers.orig_casing.append(& name, orig_name.clone()); headers.orig_order
    .append(name); hyper_code::HYPERE_OK } Err(code) => code, } }
}
impl Default for hyper_headers {
    fn default() -> Self {
        loop {}
    }
}
unsafe fn raw_name_value(
    name: *const u8,
    name_len: size_t,
    value: *const u8,
    value_len: size_t,
) -> Result<(HeaderName, HeaderValue, Bytes), hyper_code> {
    loop {}
}
impl OnInformational {
    pub(crate) fn call(&mut self, resp: Response<Body>) {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_headers_foreach_cases_preserved() {
        loop {}
    }
    #[cfg(all(feature = "http1", feature = "ffi"))]
    #[test]
    fn test_headers_foreach_order_preserved() {
        loop {}
    }
}
