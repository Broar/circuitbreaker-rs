extern crate chrono;

use super::Strategy;
use self::chrono::offset::utc::UTC;

#[derive(Debug)]
pub struct CountStrategy {
    count: usize,
    threshold: usize,
    timeout: i64,
    start_of_timeout: Option<i64>,
    is_open: bool
}

impl CountStrategy {
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
                    (now - then) * 1000 >= self.timeout
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
        self.start_of_timeout = None;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use strategy::Strategy;
    use std::{thread, time};

    #[test]
    fn test_allow_request_with_closed_circuit() {
        let strategy = CountStrategy::new(10, 10000);
        assert!(strategy.allow_request());
    }

    #[test]
    fn test_allow_request_with_open_circuit() {
        let mut strategy = CountStrategy::new(10, 10000);
        for _ in 0..10 {
            strategy.failure();
        }

        assert_eq!(false, strategy.allow_request());
    }

    #[test]
    fn test_allow_request_with_half_open_circuit() {
        let mut strategy = CountStrategy::new(10, 100);
        for _ in 0..10 {
            strategy.failure();
        }

        thread::sleep(time::Duration::from_millis(1000));

        assert!(strategy.allow_request());
    }

    #[test]
    fn test_success() {
        let mut strategy = CountStrategy::new(10, 10000);
        for _ in 0..10 {
            strategy.failure();
        }

        strategy.success();

        assert!(strategy.allow_request());
    }

    #[test]
    fn test_open() {
        let mut strategy = CountStrategy::new(10, 10000);
        strategy.open();

        assert_eq!(false, strategy.allow_request());
    }

    #[test]
    fn test_close() {
        let mut strategy = CountStrategy::new(10, 10000);
        strategy.open();
        assert_eq!(false, strategy.allow_request());
        strategy.close();
        assert!(strategy.allow_request());
    }
}
