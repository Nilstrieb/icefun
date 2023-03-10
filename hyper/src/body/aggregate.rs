use bytes::Buf;

use super::HttpBody;
use crate::common::buf::BufList;











pub async fn aggregate<T>(body: T) -> Result<impl Buf, T::Error>
where
    T: HttpBody,
{
    let mut bufs = BufList::new();

    futures_util::pin_mut!(body);
    while let Some(buf) = body.data().await {
        let buf = buf?;
        if buf.has_remaining() {
            bufs.push(buf);
        }
    }

    Ok(bufs)
}
