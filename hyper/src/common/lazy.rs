use pin_project_lite::pin_project;
use super::{task, Future, Pin, Poll};
pub(crate) trait Started: Future {
    fn started(&self) -> bool;
}
pub(crate) fn lazy<F, R>(func: F) -> Lazy<F, R>
where
    F: FnOnce() -> R,
    R: Future + Unpin,
{
    loop {}
}
pin_project! {
    #[allow(missing_debug_implementations)] pub (crate) struct Lazy < F, R > { #[pin]
    inner : Inner < F, R >, }
}
pin_project! {
    #[project = InnerProj] #[project_replace = InnerProjReplace] enum Inner < F, R > {
    Init { func : F }, Fut { #[pin] fut : R }, Empty, }
}
impl<F, R> Started for Lazy<F, R>
where
    F: FnOnce() -> R,
    R: Future,
{
    fn started(&self) -> bool {
        loop {}
    }
}
impl<F, R> Future for Lazy<F, R>
where
    F: FnOnce() -> R,
    R: Future,
{
    type Output = R::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
