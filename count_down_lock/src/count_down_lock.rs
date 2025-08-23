use std::thread;
use std::sync::{Arc, Mutex, Condvar, WaitTimeoutResult};
use std::time::Duration;

pub struct CountDownLock {
    count: Mutex<usize>,
    cv: Condvar
}

impl CountDownLock {
    pub fn new(count: usize) -> CountDownLock {
        CountDownLock {
            count: Mutex::new(count),
            cv: Condvar::new()
        }
    }

    pub fn count_down(&self) {
        let mut count = self.count.lock().unwrap();
        if *count == 0 {
            return
        }
        *count -= 1;
        if *count == 0 {
            self.cv.notify_all();
        }
    }

    pub  fn wait(&self) {
        let mut count = self.count.lock().unwrap();
        count = self.cv.wait_while(count, |&mut c| c > 0).unwrap();
    }

    pub fn wait_timeout(&self, timeout: Duration) -> WaitTimeoutResult {
        let count = self.count.lock().unwrap();
        self.cv.wait_timeout_while(count, timeout, |&mut c| c > 0).unwrap().1
    }
}


#[test]
fn test_count_down_lock_1() {
    let count_down_value = 3;
    let lock = Arc::new(CountDownLock::new(count_down_value));

    //Scenario 1: Tutti i thread count_down e uno aspetta
    let lock_clone_wait = Arc::clone(&lock);
    let waiter_thread = thread::spawn(move || {
        println!("[Waiter] Waiting for lock");
        lock_clone_wait.wait();
        println!("[Waiter] Count reached to 0");
    });

    let mut worker_threads = Vec::new();
    for i in 0..count_down_value {
        let lock_clone_worker = Arc::clone(&lock);
        worker_threads.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(100 * (i + 1) as u64));
            println!("[Worker {}] Calling count_down", i+1);
            lock_clone_worker.count_down();
        }));
    }
    for thread in worker_threads {
        thread.join().unwrap();
    }
    waiter_thread.join().unwrap();
}

#[test]
fn test_count_down_lock_2() {
    let timed_lock = Arc::new(CountDownLock::new(1));

    //Scenario 2: Timeout

    let timed_lock_clone_wait = timed_lock.clone();
    let timeout_thread = thread::spawn(move || {
        println!("[Timeout Waiter] Waiting with timeout");
        let result = timed_lock_clone_wait.wait_timeout(Duration::from_secs(1));
        if result.timed_out() {
            println!("[Timeout Waiter] Timed out");
        }
        else {
            println!("[Timeout Waiter] Condition met before timeout!");
        }
    });

    let timed_lock_clone_worker = Arc::clone(&timed_lock);
    let worker_thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(2000));
        println!("[Short Worker] Calling count_down...");
        timed_lock_clone_worker.count_down();
    });
    worker_thread.join().unwrap();
    timeout_thread.join().unwrap();

}