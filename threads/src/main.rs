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
    //    thread_vector();


}
