use crate::List::{Cons, Nil};
use smart_pointers::LinkedList;
use smart_pointers::MultiList;
use std::mem::drop;
use std::ops::Deref;
use std::rc::Rc;

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

#[derive(Debug)]
struct MySmartPointer {
    thing: String,
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropping my box now: {}", self.thing)
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

    // DROP TRAIT CUSTOM
    let myp = MySmartPointer {
        thing: String::from("Stuff"),
    };

    drop(&myp);
    println!("{:?}", myp);

    let ml = Rc::new(MultiList::Cons(
        5,
        Rc::new(MultiList::Cons(10, Rc::new(MultiList::Nil))),
    ));

    println!("Ref count of ml: {}", Rc::strong_count(&ml));

    // use Rc clone to increase ref count of multi list
    let ml1 = MultiList::Cons(3, Rc::clone(&ml));
    println!("Ref count of ml: {}", Rc::strong_count(&ml));
    let ml2 = MultiList::Cons(4, Rc::clone(&ml));
    println!("Ref count of ml: {}", Rc::strong_count(&ml));

    let mut l = LinkedList::new(42);
    l.push(5);
    l.push(6);
    println!("My list: {}", l);
}
