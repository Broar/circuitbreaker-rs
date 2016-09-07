trait BreakerStrategy {
    fn is_open() -> bool;
    fn open();
    fn close();
    fn reset();
}
