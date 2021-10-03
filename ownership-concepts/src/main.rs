use std::thread;
use core::time::Duration;

fn str_stuff() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    println!("{:?}", s.as_ptr());

    // "Moving" makes s no longer accessible
//    let s2 = s;
//    println!("{} {}", s, s2);

    let s3 = s.clone();
    println!("Cloned deep copy of s: {}", s3);

    drop(s); // called by Rust implicitly at the end of scope 
}

fn gives_ownership() -> String {
    fn inner_func() -> i32 {
        return 0;
    }

    inner_func();
    let s1 = String::from("Hello");
    s1
}

fn takes_and_gives_back(s: String) -> String { s }

fn main() {
    str_stuff();

    let s1 = gives_ownership();
    let s2 = String::from("world");
    let s3 = takes_and_gives_back(s2);
    println!("{} {}", s1, s3);

    let closure = |num: i32| -> i32 {
        println!("calculating...");
        thread::sleep(Duration::from_secs(3));
        num
    };

    println!("Answer is... {}", closure(3));
}
