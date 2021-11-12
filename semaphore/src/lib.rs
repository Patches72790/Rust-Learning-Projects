use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    condvar: Condvar,
    counter: Mutex<isize>,
}

impl Semaphore {
    pub fn new(state: isize) -> Semaphore {
        Semaphore {
            condvar: Condvar::new(),
            counter: Mutex::new(state),
        }
    }

    pub fn acquire(&self) {
        let mut count = self.counter.lock().unwrap();

        while *count <= 0 {
            count = self.condvar.wait(count).unwrap();
        }

        *count -= 1;
    }

    pub fn release(&self) {
        let mut count = self.counter.lock().unwrap();

        *count += 1;

        self.condvar.notify_one();
    }
}
