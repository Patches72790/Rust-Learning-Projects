use std::thread;
use std::{
    net::TcpStream,
    thread::{JoinHandle, Thread},
};

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        Worker {
            id,
            thread: thread::spawn(|| {}),
        }
    }
}

pub struct ThreadPool {
    threads: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            threads.push(Worker::new(id));
        }

        ThreadPool { threads }
    }

    pub fn execute<F, T>(&self, f: F)
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
    }
}
