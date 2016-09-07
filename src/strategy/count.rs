use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[allow(dead_code)]
#[derive(Debug)]
pub struct CountStrategy {
    count: AtomicUsize,
    threshold: AtomicUsize,
    timeout: AtomicUsize,
    is_open: AtomicBool
}

impl CountStrategy {

    #[allow(dead_code)]
    pub fn new(threshold: usize, timeout: usize) -> Self {
        CountStrategy {
            count: AtomicUsize::new(0),
            threshold: AtomicUsize::new(threshold),
            timeout: AtomicUsize::new(timeout),
            is_open: AtomicBool::new(false)
        }
    }
}

impl BreakerStrategy for CountStrategy {
    fn is_open(&self) -> bool {
        self.is_open.load(Ordering::Relaxed)
    }

    fn allow_request(&self) -> bool {
        false
    }

    fn open(&mut self) {
        self.is_open.compare_and_swap(false, true, Ordering::Relaxed);
    }

    fn close(&mut self) {
        self.is_open.compare_and_swap(true, false, Ordering::Relaxed);
    }

    fn reset(&mut self) {
        self.count.store(0, Ordering::Relaxed);
    }
}
