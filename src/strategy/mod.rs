pub mod count;

pub trait Strategy {
    fn is_open(&self) -> bool;
    fn allow_request(&self) -> bool;
    fn open(&mut self);
    fn close(&mut self);
    fn reset(&mut self);
}
