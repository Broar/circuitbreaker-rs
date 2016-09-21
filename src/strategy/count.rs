extern crate chrono;

use super::{Strategy, CircuitStatus};
use self::chrono::offset::utc::UTC;

#[derive(Clone, Copy, Debug)]
pub struct CountStrategy {
    count: usize,
    threshold: usize,
    timeout: i64,
    start_of_timeout: Option<i64>,
    status: CircuitStatus
}

impl CountStrategy {
    pub fn new(threshold: usize, timeout: i64) -> Self {
        assert!(threshold > 0, "Threshold must be greater than 0");

        CountStrategy {
            count: 0,
            threshold: threshold,
            timeout: timeout,
            start_of_timeout: None,
            status: CircuitStatus::Closed
        }
    }
}

impl Strategy for CountStrategy {
    fn allow_request(&mut self) -> bool {
        if let Some(then) = self.start_of_timeout {
            let now = UTC::now().timestamp();
            let timeout_has_expired = (now - then) * 1000 >= self.timeout;

            if timeout_has_expired {
                self.status = CircuitStatus::HalfOpen;
            }
        }

        match self.status {
            CircuitStatus::Open => false,
            _ => true,
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

    fn status(&self) -> CircuitStatus {
        self.status
    }

    fn open(&mut self) {
        self.status = CircuitStatus::Open;
    }

    fn close(&mut self) {
        self.status = CircuitStatus::Closed;
    }

    fn reset(&mut self) {
        self.status = CircuitStatus::Closed;
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
    #[should_panic(expected = "Threshold must be greater than 0")]
    fn test_threshold_precondition() {
        CountStrategy::new(0, 10000);
    }

    #[test]
    fn test_allow_request_with_closed_circuit() {
        let mut strategy = CountStrategy::new(10, 10000);
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
