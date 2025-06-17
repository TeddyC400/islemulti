
pub trait Server {
    fn start(&self);
    fn stop(&self);
    fn restart(&self);
} 
