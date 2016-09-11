extern crate chrono;

use super::Strategy;
use self::chrono::DateTime;

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

impl Strategy for CountStrategy {
    fn is_open(&self) -> bool {
        self.is_open
    }

    fn allow_request(&self) -> bool {
        false
    }

    fn open(&mut self) {
        self.is_open = true;
    }

    fn close(&mut self) {
        self.is_open = false;
    }

    fn reset(&mut self) {
        self.count = 0;
    }
}
