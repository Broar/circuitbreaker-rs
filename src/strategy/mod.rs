pub mod count;

pub trait Strategy {
    fn allow_request(&self) -> bool;
    fn success(&mut self);
    fn failure(&mut self);
    fn is_open(&self) -> bool;
    fn open(&mut self);
    fn close(&mut self);
    fn reset(&mut self);
}
