pub mod count;

pub trait Strategy: Send {
    fn allow_request(&mut self) -> bool;
    fn success(&mut self);
    fn failure(&mut self);
    fn status(&self) -> CircuitStatus;
    fn open(&mut self);
    fn close(&mut self);
    fn reset(&mut self);
}

#[derive(Clone, Copy, Debug)]
pub enum CircuitStatus {
    Open,
    Closed,
    HalfOpen
}
