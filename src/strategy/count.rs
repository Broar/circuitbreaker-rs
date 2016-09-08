use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[allow(dead_code)]
#[derive(Debug)]
pub struct CountStrategy {
    count: usize,
    threshold: usize,
    timeout: usize,
    is_open: bool
}

impl CountStrategy {

    #[allow(dead_code)]
    pub fn new(threshold: usize, timeout: usize) -> Self {
        CountStrategy {
            count: 0,
            threshold: threshold,
            timeout: timeout,
            is_open: false
        }
    }
}

impl BreakerStrategy for CountStrategy {
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn allow_request(&self) -> bool {
        false
    }

    pub fn open(&mut self) {
        self.is_open = true
    }

    pub fn close(&mut self) {
        self.is_open = false
    }

    pub fn reset(&mut self) {
        self.count = 0
    }
}
