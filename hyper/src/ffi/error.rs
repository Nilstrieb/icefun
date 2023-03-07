use libc::size_t;

pub(crate) struct hyper_error(crate::Error);

#[repr(C)]
pub(crate) enum hyper_code {
    
    HYPERE_OK,
    
    HYPERE_ERROR,
    
    HYPERE_INVALID_ARG,
    
    
    
    
    HYPERE_UNEXPECTED_EOF,
    
    HYPERE_ABORTED_BY_CALLBACK,
    
    #[cfg_attr(feature = "http2", allow(unused))]
    HYPERE_FEATURE_NOT_ENABLED,
    
    HYPERE_INVALID_PEER_MESSAGE,
}
impl hyper_error {
    fn code(&self) -> hyper_code {
        loop {}
    }
    fn print_to(&self, dst: &mut [u8]) -> usize {
        loop {}
    }
}
ffi_fn! {
    #[doc = " Frees a `hyper_error`."] fn hyper_error_free(err : * mut hyper_error) {
    drop(non_null!(Box::from_raw(err) ?= ())); }
}
ffi_fn! {
    #[doc = " Get an equivalent `hyper_code` from this error."] fn hyper_error_code(err :
    * const hyper_error) -> hyper_code { non_null!(&* err ?=
    hyper_code::HYPERE_INVALID_ARG) .code() }
}
ffi_fn! {
    #[doc = " Print the details of this error to a buffer."] #[doc = ""] #[doc =
    " The `dst_len` value must be the maximum length that the buffer can"] #[doc =
    " store."] #[doc = ""] #[doc =
    " The return value is number of bytes that were written to `dst`."] fn
    hyper_error_print(err : * const hyper_error, dst : * mut u8, dst_len : size_t) ->
    size_t { let dst = unsafe { std::slice::from_raw_parts_mut(dst, dst_len) };
    non_null!(&* err ?= 0) .print_to(dst) }
}
