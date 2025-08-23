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
    data: Mutex<(usize, bool)>,
    cond: Condvar,
}

impl CancelableLatch for CancelableLatch1 {
    fn new(count: usize) -> Self {
        Self {
            data: Mutex::new((count, false)),
            cond: Condvar::new()
        }
    }

    fn count_down(&self) {
        let mut data = self.data.lock().unwrap();
        if !data.1 && data.0 > 0 {
            data.0 -= 1;
            self.cond.notify_one();
        }
    }

    fn cancel(&self) {
        let mut data = self.data.lock().unwrap();
        if !data.1 {
            data.1 = true;
            self.cond.notify_all();
        }
    }
    fn wait(&self) -> WaitResult {
        let data = self.data.lock().unwrap();
        let exit = self.cond.wait_while(data, |d| d.0 > 0 && !d.1).unwrap().1;
        if !exit { WaitResult::Success } else { WaitResult::Canceled }
    }
    fn wait_timeout(&self, timeout: Duration) -> WaitResult {
        let data = self.data.lock().unwrap();
        match self.cond.wait_timeout_while(data, timeout, |&mut d| d.0 > 0 && !d.1).unwrap()
        {
            (_, r) if r.timed_out() => WaitResult::Timeout,
            (d, _) if d.1 => WaitResult::Canceled,
            _ => WaitResult::Success,
        }

    }
}
