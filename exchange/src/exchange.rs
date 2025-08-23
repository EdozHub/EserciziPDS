use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

pub struct Exchange<T: Send> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T: Send> Exchange<T> {
    pub fn exchange(&self, msg: T) -> Option<T> {
        let _ = self.tx.send(msg);
        self.rx.recv().ok()
    }

    pub fn make_exchangers() -> (Exchange<T>, Exchange<T>) {
        let (tx1, rx1) = channel::<T>();
        let (tx2, rx2) = channel::<T>();
        (
            {Exchange {tx: tx1, rx: rx2}},
            {Exchange {tx: tx2, rx: rx1}}
        )
    }
}


#[test]
fn total_test(){
    let (x1, x2) = Exchange::make_exchangers();
    let mut handles = Vec::new();
    handles.push(thread::spawn(move || {
        for i in 1..10 {
            let ret = x1.exchange(i);

            match ret {
                Some(data) => println!("Exchanger 1 received {}", data),
                None => println!("Exchanger 1 received None")
            }
        }
    }));

    handles.push(thread::spawn(move || {
        for i in 1..10 {
            let ret = x2.exchange(i);

            match ret {
                Some(data) => println!("Exchanger 2 received {}", data),
                None => println!("Exchanger 2 received None")
            }
        }
    }));

    for handle in handles {
        handle.join().unwrap();
    }
}
