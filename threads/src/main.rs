use std::thread;
use std::time::Duration;

fn thread_vector() {
    let my_closure = move |n: i32| {
        for j in 0..10 {
            println!("thread {} at j {}", n, j);
            thread::sleep(Duration::from_millis(1));
        }
    };

    for i in 0..5 {
        thread::spawn(move || my_closure(i));
    }

    thread::sleep(Duration::from_secs(2));
}

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread_vector();
}
