
pub trait Executor {  
    fn execute_mut<F>(&mut self, _job : F)
    where F : FnOnce() + Send + 'static {
    }
}