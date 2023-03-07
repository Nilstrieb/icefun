//! File System Filters

use std::convert::Infallible;
use std::fs::Metadata;
use std::future::Future;
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;
use bytes::{Bytes, BytesMut};
use futures_util::future::Either;
use futures_util::{future, ready, stream, FutureExt, Stream, StreamExt, TryFutureExt};
use headers::{
    AcceptRanges, ContentLength, ContentRange, ContentType, HeaderMapExt,
    IfModifiedSince, IfRange, IfUnmodifiedSince, LastModified, Range,
};
use http::StatusCode;
use hyper::Body;
use mime_guess;

use tokio::fs::File as TkFile;
use tokio::io::AsyncSeekExt;
use tokio_util::io::poll_read_buf;
use crate::filter::{Filter, FilterClone, One};
use crate::reject::{self, Rejection};
use crate::reply::{Reply, Response};
/// Creates a `Filter` that serves a File at the `path`.
///
/// Does not filter out based on any information of the request. Always serves
/// the file at the exact `path` provided. Thus, this can be used to serve a
/// single file with `GET`s, but could also be used in combination with other
/// filters, such as after validating in `POST` request, wanting to return a
/// specific file as the body.
///
/// For serving a directory, see [dir](dir).
///
/// # Example
///
/// ```
/// // Always serves this file from the file system.
/// let route = warp::fs::file("/www/static/app.js");
/// ```
pub fn file(
    path: impl Into<PathBuf>,
) -> impl FilterClone<Extract = One<File>, Error = Rejection> {
    let path = Arc::new(path.into());
    crate::any()
        .map(move || {
            tracing::trace!("file: {:?}", path);
            ArcPath(path.clone())
        })
        .and(conditionals())
        .and_then(file_reply)
}
/// Creates a `Filter` that serves a directory at the base `path` joined
/// by the request path.
///
/// This can be used to serve "static files" from a directory. By far the most
/// common pattern of serving static files is for `GET` requests, so this
/// filter automatically includes a `GET` check.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// // Matches requests that start with `/static`,
/// // and then uses the rest of that path to lookup
/// // and serve a file from `/www/static`.
/// let route = warp::path("static")
///     .and(warp::fs::dir("/www/static"));
///
/// // For example:
/// // - `GET /static/app.js` would serve the file `/www/static/app.js`
/// // - `GET /static/css/app.css` would serve the file `/www/static/css/app.css`
/// ```
pub fn dir(
    path: impl Into<PathBuf>,
) -> impl FilterClone<Extract = One<File>, Error = Rejection> {
    let base = Arc::new(path.into());
    crate::get()
        .or(crate::head())
        .unify()
        .and(path_from_tail(base))
        .and(conditionals())
        .and_then(file_reply)
}
fn path_from_tail(
    base: Arc<PathBuf>,
) -> impl FilterClone<Extract = One<ArcPath>, Error = Rejection> {
    crate::path::tail()
        .and_then(move |tail: crate::path::Tail| {
            future::ready(sanitize_path(base.as_ref(), tail.as_str()))
                .and_then(|mut buf| async {
                    let is_dir = tokio::fs::metadata(buf.clone())
                        .await
                        .map(|m| m.is_dir())
                        .unwrap_or(false);
                    if is_dir {
                        tracing::debug!("dir: appending index.html to directory path");
                        buf.push("index.html");
                    }
                    tracing::trace!("dir: {:?}", buf);
                    Ok(ArcPath(Arc::new(buf)))
                })
        })
}
fn sanitize_path(base: impl AsRef<Path>, tail: &str) -> Result<PathBuf, Rejection> {
    loop {}
}
#[derive(Debug)]
struct Conditionals {
    if_modified_since: Option<IfModifiedSince>,
    if_unmodified_since: Option<IfUnmodifiedSince>,
    if_range: Option<IfRange>,
    range: Option<Range>,
}
enum Cond {
    NoBody(Response),
    WithBody(Option<Range>),
}
impl Conditionals {
    fn check(self, last_modified: Option<LastModified>) -> Cond {
        loop {}
    }
}
fn conditionals() -> impl Filter<
    Extract = One<Conditionals>,
    Error = Infallible,
> + Copy {
    crate::header::optional2()
        .and(crate::header::optional2())
        .and(crate::header::optional2())
        .and(crate::header::optional2())
        .map(|if_modified_since, if_unmodified_since, if_range, range| Conditionals {
            if_modified_since,
            if_unmodified_since,
            if_range,
            range,
        })
}
/// A file response.
#[derive(Debug)]
pub struct File {
    resp: Response,
    path: ArcPath,
}
impl File {
    /// Extract the `&Path` of the file this `Response` delivers.
    ///
    /// # Example
    ///
    /// The example below changes the Content-Type response header for every file called `video.mp4`.
    ///
    /// ```
    /// use warp::{Filter, reply::Reply};
    ///
    /// let route = warp::path("static")
    ///     .and(warp::fs::dir("/www/static"))
    ///     .map(|reply: warp::filters::fs::File| {
    ///         if reply.path().ends_with("video.mp4") {
    ///             warp::reply::with_header(reply, "Content-Type", "video/mp4").into_response()
    ///         } else {
    ///             reply.into_response()
    ///         }
    ///     });
    /// ```
    pub fn path(&self) -> &Path {
        loop {}
    }
}
#[derive(Clone, Debug)]
struct ArcPath(Arc<PathBuf>);
impl AsRef<Path> for ArcPath {
    fn as_ref(&self) -> &Path {
        loop {}
    }
}
impl Reply for File {
    fn into_response(self) -> Response {
        loop {}
    }
}
fn file_reply(
    path: ArcPath,
    conditionals: Conditionals,
) -> impl Future<Output = Result<File, Rejection>> + Send {
    TkFile::open(path.clone())
        .then(move |res| match res {
            Ok(f) => Either::Left(file_conditional(f, path, conditionals)),
            Err(err) => {
                let rej = match err.kind() {
                    io::ErrorKind::NotFound => {
                        tracing::debug!("file not found: {:?}", path.as_ref().display());
                        reject::not_found()
                    }
                    io::ErrorKind::PermissionDenied => {
                        tracing::warn!(
                            "file permission denied: {:?}", path.as_ref().display()
                        );
                        reject::known(FilePermissionError { _p: () })
                    }
                    _ => {
                        tracing::error!(
                            "file open error (path={:?}): {} ", path.as_ref().display(),
                            err
                        );
                        reject::known(FileOpenError { _p: () })
                    }
                };
                Either::Right(future::err(rej))
            }
        })
}
async fn file_metadata(f: TkFile) -> Result<(TkFile, Metadata), Rejection> {
    loop {}
}
fn file_conditional(
    f: TkFile,
    path: ArcPath,
    conditionals: Conditionals,
) -> impl Future<Output = Result<File, Rejection>> + Send {
    file_metadata(f)
        .map_ok(move |(file, meta)| {
            let mut len = meta.len();
            let modified = meta.modified().ok().map(LastModified::from);
            let resp = match conditionals.check(modified) {
                Cond::NoBody(resp) => resp,
                Cond::WithBody(range) => {
                    bytes_range(range, len)
                        .map(|(start, end)| {
                            let sub_len = end - start;
                            let buf_size = optimal_buf_size(&meta);
                            let stream = file_stream(file, buf_size, (start, end));
                            let body = Body::wrap_stream(stream);
                            let mut resp = Response::new(body);
                            if sub_len != len {
                                *resp.status_mut() = StatusCode::PARTIAL_CONTENT;
                                resp.headers_mut()
                                    .typed_insert(
                                        ContentRange::bytes(start..end, len)
                                            .expect("valid ContentRange"),
                                    );
                                len = sub_len;
                            }
                            let mime = mime_guess::from_path(path.as_ref())
                                .first_or_octet_stream();
                            resp.headers_mut().typed_insert(ContentLength(len));
                            resp.headers_mut().typed_insert(ContentType::from(mime));
                            resp.headers_mut().typed_insert(AcceptRanges::bytes());
                            if let Some(last_modified) = modified {
                                resp.headers_mut().typed_insert(last_modified);
                            }
                            resp
                        })
                        .unwrap_or_else(|BadRange| {
                            let mut resp = Response::new(Body::empty());
                            *resp.status_mut() = StatusCode::RANGE_NOT_SATISFIABLE;
                            resp.headers_mut()
                                .typed_insert(ContentRange::unsatisfied_bytes(len));
                            resp
                        })
                }
            };
            File { resp, path }
        })
}
struct BadRange;
fn bytes_range(range: Option<Range>, max_len: u64) -> Result<(u64, u64), BadRange> {
    loop {}
}
fn file_stream(
    mut file: TkFile,
    buf_size: usize,
    (start, end): (u64, u64),
) -> impl Stream<Item = Result<Bytes, io::Error>> + Send {
    use std::io::SeekFrom;
    let seek = async move {
        if start != 0 {
            file.seek(SeekFrom::Start(start)).await?;
        }
        Ok(file)
    };
    seek.into_stream()
        .map(move |result| {
            let mut buf = BytesMut::new();
            let mut len = end - start;
            let mut f = match result {
                Ok(f) => f,
                Err(f) => return Either::Left(stream::once(future::err(f))),
            };
            Either::Right(
                stream::poll_fn(move |cx| {
                    if len == 0 {
                        return Poll::Ready(None);
                    }
                    reserve_at_least(&mut buf, buf_size);
                    let n = match ready!(
                        poll_read_buf(Pin::new(& mut f), cx, & mut buf)
                    ) {
                        Ok(n) => n as u64,
                        Err(err) => {
                            tracing::debug!("file read error: {}", err);
                            return Poll::Ready(Some(Err(err)));
                        }
                    };
                    if n == 0 {
                        tracing::debug!("file read found EOF before expected length");
                        return Poll::Ready(None);
                    }
                    let mut chunk = buf.split().freeze();
                    if n > len {
                        chunk = chunk.split_to(len as usize);
                        len = 0;
                    } else {
                        len -= n;
                    }
                    Poll::Ready(Some(Ok(chunk)))
                }),
            )
        })
        .flatten()
}
fn reserve_at_least(buf: &mut BytesMut, cap: usize) {
    loop {}
}
const DEFAULT_READ_BUF_SIZE: usize = 8_192;
fn optimal_buf_size(metadata: &Metadata) -> usize {
    loop {}
}
#[cfg(unix)]
fn get_block_size(metadata: &Metadata) -> usize {
    loop {}
}
#[cfg(not(unix))]
fn get_block_size(_metadata: &Metadata) -> usize {
    loop {}
}
unit_error! {
    pub (crate) FileOpenError : "file open error"
}
unit_error! {
    pub (crate) FilePermissionError : "file perimission error"
}
#[cfg(test)]
mod tests {
    use super::sanitize_path;
    use bytes::BytesMut;
    #[test]
    fn test_sanitize_path() {
        loop {}
    }
    #[test]
    fn test_reserve_at_least() {
        loop {}
    }
}
