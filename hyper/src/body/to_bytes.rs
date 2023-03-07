use bytes::{Buf, BufMut, Bytes};
use super::HttpBody;
/// Concatenate the buffers from a body into a single `Bytes` asynchronously.
///
/// This may require copying the data into a single buffer. If you don't need
/// a contiguous buffer, prefer the [`aggregate`](crate::body::aggregate())
/// function.
///
/// # Note
///
/// Care needs to be taken if the remote is untrusted. The function doesn't implement any length
/// checks and an malicious peer might make it consume arbitrary amounts of memory. Checking the
/// `Content-Length` is a possibility, but it is not strictly mandated to be present.
///
/// # Example
///
/// ```
/// # #[cfg(all(feature = "client", feature = "tcp", any(feature = "http1", feature = "http2")))]
/// # async fn doc() -> hyper::Result<()> {
/// use hyper::{body::HttpBody};
///
/// # let request = hyper::Request::builder()
/// #        .method(hyper::Method::POST)
/// #        .uri("http://httpbin.org/post")
/// #        .header("content-type", "application/json")
/// #        .body(hyper::Body::from(r#"{"library":"hyper"}"#)).unwrap();
/// # let client = hyper::Client::new();
/// let response = client.request(request).await?;
///
/// const MAX_ALLOWED_RESPONSE_SIZE: u64 = 1024;
///
/// let response_content_length = match response.body().size_hint().upper() {
///     Some(v) => v,
///     None => MAX_ALLOWED_RESPONSE_SIZE + 1 // Just to protect ourselves from a malicious response
/// };
///
/// if response_content_length < MAX_ALLOWED_RESPONSE_SIZE {
///     let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
///     println!("body: {:?}", body_bytes);
/// }
///
/// # Ok(())
/// # }
/// ```
pub async fn to_bytes<T>(body: T) -> Result<Bytes, T::Error>
where
    T: HttpBody,
{
    loop {}
}
