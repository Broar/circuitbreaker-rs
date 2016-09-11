extern crate chrono;

use super::Strategy;
use self::chrono::offset::utc::UTC;

#[allow(dead_code)]
#[derive(Debug)]
pub struct CountStrategy {
    count: usize,
    threshold: usize,
    timeout: i64,
    start_of_timeout: Option<i64>,
    is_open: bool
}

impl CountStrategy {

    #[allow(dead_code)]
    pub fn new(threshold: usize, timeout: i64) -> Self {
        CountStrategy {
            count: 0,
            threshold: threshold,
            timeout: timeout,
            start_of_timeout: None,
            is_open: false
        }
    }
}

impl Strategy for CountStrategy {
    fn allow_request(&self) -> bool {
        if !self.is_open {
            true
        }

        else {
            match self.start_of_timeout {
                Some(then) => {
                    let now = UTC::now().timestamp();
                    now - then >= self.timeout
                },

                None => {
                    false
                }
            }
        }
    }

    fn success(&mut self) {
        self.reset();
    }

    fn failure(&mut self) {
        self.count += 1;
        if self.count >= self.threshold {
            self.start_of_timeout = Some(UTC::now().timestamp());
            self.open();
        }
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn open(&mut self) {
        self.is_open = true;
    }

    fn close(&mut self) {
        self.is_open = false;
    }

    fn reset(&mut self) {
        self.is_open = false;
        self.count = 0;
    }
}
