extern crate chrono;

use super::Strategy;
use self::chrono::offset::utc::UTC;

#[derive(Clone, Copy, Debug)]
pub struct PercentageStrategy {
    requests: u32,
    failures: u32,
    threshold: f64,
    timeout: i64,
    start_of_timeout: Option<i64>,
    is_open: bool
}

impl PercentageStrategy {
    pub fn new(threshold: f64, timeout: i64) -> Self {
        assert!(threshold > 0.0 && threshold < 1.0, "Threshold must be between 0.0 and 1.0");

        PercentageStrategy {
            requests: 0,
            failures: 0,
            threshold: threshold,
            timeout: timeout,
            start_of_timeout: None,
            is_open: false
        }
    }
}

impl Strategy for PercentageStrategy {
    fn allow_request(&mut self) -> bool {
        self.requests += 1;

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
        self.failures += 1;
        if (self.failures as f64 / self.requests as f64) >= self.threshold {
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
        self.requests = 0;
        self.failures = 0;
        self.start_of_timeout = None;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use strategy::Strategy;
    use std::{thread, time};

    #[test]
    #[should_panic(expected = "Threshold must be between 0.0 and 1.0")]
    fn test_threshold_precondition() {
        PercentageStrategy::new(0.0, 10000);
    }

    #[test]
    fn test_allow_request_with_closed_circuit() {
        let mut strategy = PercentageStrategy::new(0.5, 10000);
        assert!(strategy.allow_request());
    }

    #[test]
    fn test_allow_request_with_open_circuit() {
        let mut strategy = PercentageStrategy::new(0.5, 10000);
        for _ in 0..10 {
            strategy.failure();
        }

        assert_eq!(false, strategy.allow_request());
    }

    #[test]
    fn test_allow_request_with_half_open_circuit() {
        let mut strategy = PercentageStrategy::new(0.5, 100);
        for _ in 0..10 {
            strategy.failure();
        }

        thread::sleep(time::Duration::from_millis(1000));

        assert!(strategy.allow_request());
    }

    #[test]
    fn test_success() {
        let mut strategy = PercentageStrategy::new(0.5, 10000);
        for _ in 0..10 {
            strategy.failure();
        }

        strategy.success();

        assert!(strategy.allow_request());
    }

    #[test]
    fn test_open() {
        let mut strategy = PercentageStrategy::new(0.5, 10000);
        strategy.open();

        assert_eq!(false, strategy.allow_request());
    }

    #[test]
    fn test_close() {
        let mut strategy = PercentageStrategy::new(0.5, 10000);
        strategy.open();
        assert_eq!(false, strategy.allow_request());
        strategy.close();
        assert!(strategy.allow_request());
    }
}
