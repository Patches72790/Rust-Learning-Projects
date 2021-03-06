use std::sync::{mpsc, Condvar};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn shared_memory() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

pub fn message_passing() {
    let (tx, rx) = mpsc::channel();
    let cv = Arc::new(Condvar::new());
    let mutex = Arc::new(Mutex::new(0));

    let special_field = std::cell::Cell::new(5);
    let tx1 = tx.clone();
    let cv1 = Arc::clone(&cv);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        cv1.notify_one();
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        cv.wait(mutex.lock().unwrap()).unwrap();
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got {}", received);
    }
}
