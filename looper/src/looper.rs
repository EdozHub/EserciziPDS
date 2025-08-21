use std::sync::mpsc::{channel, Sender};
use std::thread;

pub struct Looper<M: Send + 'static> {
    sender: Sender<M>
}

impl<M: Send + 'static> Looper<M> {
    pub fn new(
        process: impl Fn(M) + Send + 'static,
        cleanup: impl FnOnce(M) + Send + 'static
    ) -> Self {
        let (tx, rx) = channel();
        thread::spawn(move || {
            loop {
                let m = rx.recv();
                match m {
                    Ok(m) => process(m),
                    Err(_) => break,
                }
                cleanup();
            }
        });
        Looper{sender: tx}
    }

    pub fn send(&self, m: M) {
        self.sender.send(m).unwrap()
    }
}