use std::collections::LinkedList;

fn main() {
    let ptr = PORTB::ptr;
    let list = LinkedList::new();

    list.push_front("abc");
    println!("{}", list);
}
