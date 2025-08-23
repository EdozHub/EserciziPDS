use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use std::thread;

#[derive(PartialEq, Eq, Debug)]
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

pub struct CancelableLatchImpl {
    data: Mutex<(usize, bool)>,
    cond: Condvar,
}

impl CancelableLatch for CancelableLatchImpl {
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
        let data = self.cond.wait_while(data, |d| d.0 > 0 && !d.1).unwrap();
        if !data.1 { WaitResult::Success } else { WaitResult::Canceled }
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

#[test]
fn test_cancelable_latch_success() {
    let latch_success = Arc::new(CancelableLatchImpl::new(3));
    let mut handles = Vec::new();

    //thread che decrementano il contatore
    for i in 0..3 {
        let latch_clone_success_worker = Arc::clone(&latch_success);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(100 * i));
            println!("[Worker {}] Counting down...", i+1);
            latch_clone_success_worker.count_down();
        }));
    }

    //thread che attende il successo
    let latch_clone_success_waiter = Arc::clone(&latch_success);
    handles.push(thread::spawn(move || {
        println!("[Waiter] Waiting...");
        let result = latch_clone_success_waiter.wait();
        println!("[Waiter] Result of waitiing: {:?} (Success expected)", result)
    }));
    for handle in handles {
        handle.join().unwrap();
    }
}