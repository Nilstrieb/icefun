use pin_project_lite::pin_project;
use tower_service::Service;
use crate::common::{task, Future, Pin, Poll};
pub(crate) fn oneshot<S, Req>(svc: S, req: Req) -> Oneshot<S, Req>
where
    S: Service<Req>,
{
    loop {}
}
pin_project! {
    #[allow(missing_debug_implementations)] pub struct Oneshot < S : Service < Req >, Req
    > { #[pin] state : State < S, Req >, }
}
pin_project! {
    #[project = StateProj] #[project_replace = StateProjOwn] enum State < S : Service <
    Req >, Req > { NotReady { svc : S, req : Req, }, Called { #[pin] fut : S::Future, },
    Tmp, }
}
impl<S, Req> Future for Oneshot<S, Req>
where
    S: Service<Req>,
{
    type Output = Result<S::Response, S::Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
