use std::ffi::c_void;
use std::future::Future;
use std::pin::Pin;
use std::ptr;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex, Weak,
};
use std::task::{Context, Poll};
use futures_util::stream::{FuturesUnordered, Stream};
use libc::c_int;
use super::error::hyper_code;
use super::UserDataPointer;
type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
type BoxAny = Box<dyn AsTaskType + Send + Sync>;

pub(crate) const HYPER_POLL_READY: c_int = 0;




pub(crate) const HYPER_POLL_PENDING: c_int = 1;

pub(crate) const HYPER_POLL_ERROR: c_int = 3;

pub(crate) struct hyper_executor {
    
    
    
    
    
    
    
    driver: Mutex<FuturesUnordered<TaskFuture>>,
    
    
    
    
    spawn_queue: Mutex<Vec<TaskFuture>>,
    
    
    is_woken: Arc<ExecWaker>,
}
#[derive(Clone)]
pub(crate) struct WeakExec(Weak<hyper_executor>);
struct ExecWaker(AtomicBool);

pub(crate) struct hyper_task {
    future: BoxFuture<BoxAny>,
    output: Option<BoxAny>,
    userdata: UserDataPointer,
}
struct TaskFuture {
    task: Option<Box<hyper_task>>,
}

pub(crate) struct hyper_context<'a>(Context<'a>);

pub(crate) struct hyper_waker {
    waker: std::task::Waker,
}

#[repr(C)]
pub(crate) enum hyper_task_return_type {
    
    HYPER_TASK_EMPTY,
    
    HYPER_TASK_ERROR,
    
    HYPER_TASK_CLIENTCONN,
    
    HYPER_TASK_RESPONSE,
    
    HYPER_TASK_BUF,
}
pub(crate) unsafe trait AsTaskType {
    fn as_task_type(&self) -> hyper_task_return_type;
}
pub(crate) trait IntoDynTaskType {
    fn into_dyn_task_type(self) -> BoxAny;
}
impl hyper_executor {
    fn new() -> Arc<hyper_executor> {
        loop {}
    }
    pub(crate) fn downgrade(exec: &Arc<hyper_executor>) -> WeakExec {
        loop {}
    }
    fn spawn(&self, task: Box<hyper_task>) {
        loop {}
    }
    fn poll_next(&self) -> Option<Box<hyper_task>> {
        loop {}
    }
    fn drain_queue(&self) -> bool {
        loop {}
    }
}
impl futures_util::task::ArcWake for ExecWaker {
    fn wake_by_ref(me: &Arc<ExecWaker>) {
        loop {}
    }
}
impl WeakExec {
    pub(crate) fn new() -> Self {
        loop {}
    }
}
impl crate::rt::Executor<BoxFuture<()>> for WeakExec {
    fn execute(&self, fut: BoxFuture<()>) {
        loop {}
    }
}
ffi_fn! {
    #[doc = " Creates a new task executor."] fn hyper_executor_new() -> * const
    hyper_executor { Arc::into_raw(hyper_executor::new()) } ?= ptr::null()
}
ffi_fn! {
    #[doc = " Frees an executor and any incomplete tasks still part of it."] fn
    hyper_executor_free(exec : * const hyper_executor) {
    drop(non_null!(Arc::from_raw(exec) ?= ())); }
}
ffi_fn! {
    #[doc = " Push a task onto the executor."] #[doc = ""] #[doc =
    " The executor takes ownership of the task, it should not be accessed"] #[doc =
    " again unless returned back to the user with `hyper_executor_poll`."] fn
    hyper_executor_push(exec : * const hyper_executor, task : * mut hyper_task) ->
    hyper_code { let exec = non_null!(&* exec ?= hyper_code::HYPERE_INVALID_ARG); let
    task = non_null!(Box::from_raw(task) ?= hyper_code::HYPERE_INVALID_ARG); exec
    .spawn(task); hyper_code::HYPERE_OK }
}
ffi_fn! {
    #[doc =
    " Polls the executor, trying to make progress on any tasks that have notified"] #[doc
    = " that they are ready again."] #[doc = ""] #[doc =
    " If ready, returns a task from the executor that has completed."] #[doc = ""] #[doc
    = " If there are no ready tasks, this returns `NULL`."] fn hyper_executor_poll(exec :
    * const hyper_executor) -> * mut hyper_task { let exec = non_null!(&* exec ?=
    ptr::null_mut()); match exec.poll_next() { Some(task) => Box::into_raw(task), None =>
    ptr::null_mut(), } } ?= ptr::null_mut()
}
impl hyper_task {
    pub(crate) fn boxed<F>(fut: F) -> Box<hyper_task>
    where
        F: Future + Send + 'static,
        F::Output: IntoDynTaskType + Send + Sync + 'static,
    {
        loop {}
    }
    fn output_type(&self) -> hyper_task_return_type {
        loop {}
    }
}
impl Future for TaskFuture {
    type Output = Box<hyper_task>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}
ffi_fn! {
    #[doc = " Free a task."] fn hyper_task_free(task : * mut hyper_task) {
    drop(non_null!(Box::from_raw(task) ?= ())); }
}
ffi_fn! {
    #[doc = " Takes the output value of this task."] #[doc = ""] #[doc =
    " This must only be called once polling the task on an executor has finished"] #[doc
    = " this task."] #[doc = ""] #[doc =
    " Use `hyper_task_type` to determine the type of the `void *` return value."] fn
    hyper_task_value(task : * mut hyper_task) -> * mut c_void { let task = non_null!(&
    mut * task ?= ptr::null_mut()); if let Some(val) = task.output.take() { let p =
    Box::into_raw(val) as * mut c_void; if p == std::ptr::NonNull::< c_void >::dangling()
    .as_ptr() { ptr::null_mut() } else { p } } else { ptr::null_mut() } } ?=
    ptr::null_mut()
}
ffi_fn! {
    #[doc = " Query the return type of this task."] fn hyper_task_type(task : * mut
    hyper_task) -> hyper_task_return_type { non_null!(&* task ?=
    hyper_task_return_type::HYPER_TASK_EMPTY) .output_type() }
}
ffi_fn! {
    #[doc = " Set a user data pointer to be associated with this task."] #[doc = ""]
    #[doc = " This value will be passed to task callbacks, and can be checked later"]
    #[doc = " with `hyper_task_userdata`."] fn hyper_task_set_userdata(task : * mut
    hyper_task, userdata : * mut c_void) { if task.is_null() { return; } unsafe { (*
    task).userdata = UserDataPointer(userdata) }; }
}
ffi_fn! {
    #[doc = " Retrieve the userdata that has been set via `hyper_task_set_userdata`."] fn
    hyper_task_userdata(task : * mut hyper_task) -> * mut c_void { non_null!(&* task ?=
    ptr::null_mut()) .userdata.0 } ?= ptr::null_mut()
}
unsafe impl AsTaskType for () {
    fn as_task_type(&self) -> hyper_task_return_type {
        loop {}
    }
}
unsafe impl AsTaskType for crate::Error {
    fn as_task_type(&self) -> hyper_task_return_type {
        loop {}
    }
}
impl<T> IntoDynTaskType for T
where
    T: AsTaskType + Send + Sync + 'static,
{
    fn into_dyn_task_type(self) -> BoxAny {
        loop {}
    }
}
impl<T> IntoDynTaskType for crate::Result<T>
where
    T: IntoDynTaskType + Send + Sync + 'static,
{
    fn into_dyn_task_type(self) -> BoxAny {
        loop {}
    }
}
impl<T> IntoDynTaskType for Option<T>
where
    T: IntoDynTaskType + Send + Sync + 'static,
{
    fn into_dyn_task_type(self) -> BoxAny {
        loop {}
    }
}
impl hyper_context<'_> {
    pub(crate) fn wrap<'a, 'b>(cx: &'a mut Context<'b>) -> &'a mut hyper_context<'b> {
        loop {}
    }
}
ffi_fn! {
    #[doc = " Copies a waker out of the task context."] fn hyper_context_waker(cx : * mut
    hyper_context <'_ >) -> * mut hyper_waker { let waker = non_null!(& mut * cx ?=
    ptr::null_mut()) .0.waker().clone(); Box::into_raw(Box::new(hyper_waker { waker })) }
    ?= ptr::null_mut()
}
ffi_fn! {
    #[doc = " Free a waker that hasn't been woken."] fn hyper_waker_free(waker : * mut
    hyper_waker) { drop(non_null!(Box::from_raw(waker) ?= ())); }
}
ffi_fn! {
    #[doc = " Wake up the task associated with a waker."] #[doc = ""] #[doc =
    " NOTE: This consumes the waker. You should not use or free the waker afterwards."]
    fn hyper_waker_wake(waker : * mut hyper_waker) { let waker =
    non_null!(Box::from_raw(waker) ?= ()); waker.waker.wake(); }
}
