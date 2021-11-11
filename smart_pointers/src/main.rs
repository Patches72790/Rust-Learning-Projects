use crate::List::{Cons, Nil};
use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let b = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", b);

    let mb = MyBox::new(42);
    let mbr = mb.deref();
    println!("{:?}, {}", *mb, *mbr);

    let bs = MyBox::new(String::from("Wilson"));
    hello(&bs); // with deref coercion, deref is called as many times as needed
    hello(&(*bs)[..]); // crazy version with dereferencing and slice-making
}
