use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use std::thread;

pub enum WaitResult {
    Success,
    Timeout,
    Canceled
}

pub trait CancelableLatch {
    fn new(count: usize) -> Self;
    fn count_down(&self);
    fn cancel(&self);
    fn wait(&self) -> WaitResult;
    fn wait_timeout(&self, timeout: Duration) -> WaitResult;
}

pub struct CancelableLatch1 {
    count: Mutex<usize>,
    cond: Condvar,
}

impl CancelableLatch for CancelableLatch1 {
    fn new(count: usize) -> CancelableLatch1 {
        unimplemented!()
    }
    fn count_down(&self) {
        unimplemented!()
    }
    fn cancel(&self) {
        unimplemented!()
    }
    fn wait(&self) -> WaitResult {
        unimplemented!()
    }
    fn wait_timeout(&self, timeout: Duration) -> WaitResult {
        unimplemented!()
    }
}