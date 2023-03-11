use std::{rc::Rc, thread::spawn};

use semaphore::Semaphore;

fn main() {
    let s = Semaphore::new(1);

    let sem_ref = Rc::new(s);
    let sem_ref2 = Rc::clone(&sem_ref);

    spawn(move || {});

    //s.acquire();
    //s.release();
}
