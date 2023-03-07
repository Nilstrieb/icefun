





































#[repr(transparent)]
pub(crate) struct SyncWrapper<T>(T);
impl<T> SyncWrapper<T> {
    
    
    
    
    
    
    
    
    
    pub(crate) fn new(value: T) -> Self {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub(crate) fn get_mut(&mut self) -> &mut T {
        loop {}
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[allow(dead_code)]
    pub(crate) fn into_inner(self) -> T {
        loop {}
    }
}
unsafe impl<T: Send> Sync for SyncWrapper<T> {}
