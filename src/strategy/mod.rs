pub mod count;
pub mod percentage;

pub trait Strategy: Send {
    fn allow_request(&mut self) -> bool;
    fn success(&mut self);
    fn failure(&mut self);
    fn is_open(&self) -> bool;
    fn open(&mut self);
    fn close(&mut self);
    fn reset(&mut self);
}
