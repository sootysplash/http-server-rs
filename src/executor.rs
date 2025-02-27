
pub trait Executor {
    fn execute<F>(&self, _job : F) -> ()
    where F : FnOnce() + Send + 'static {
    }
    
    fn execute_mut<F>(&mut self, _job : F)
    where F : FnOnce() + Send + 'static {
    }
}