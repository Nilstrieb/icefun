use bytes::{Bytes};
use super::HttpBody;










































pub async fn to_bytes<T>(body: T) -> Result<Bytes, T::Error>
where
    T: HttpBody,
{
    loop {}
}
