pub use linked_list_stuff::LinkedList;
use std::rc::Rc;

pub enum MultiList {
    Cons(i32, Rc<MultiList>),
    Nil,
}

/**
 * This module contains stuff related to my initial
 * implementation of a linked list in Rust.
 */
pub mod linked_list_stuff {
    use std::mem;

    pub type Link<T> = Option<Box<LinkedNode<T>>>;

    pub struct LinkedNode<T> {
        pub data: T,
        pub next: Link<T>,
    }

    pub struct LinkedList<T> {
        pub head: Link<T>,
    }

    impl<T> LinkedList<T>
    where
        T: Copy,
    {
        pub fn new(data: T) -> LinkedList<T> {
            LinkedList {
                head: Some(Box::new(LinkedNode { data, next: None })),
            }
        }

        pub fn push(&mut self, data: T) {
            let new_node = LinkedNode {
                data,
                next: mem::replace(&mut self.head, None),
            };

            self.head = Some(Box::new(new_node));
        }

        pub fn pop(&mut self) -> Option<T> {
            match mem::replace(&mut self.head, None) {
                None => None,
                Some(node) => {
                    self.head = node.next;
                    Some(node.data)
                }
            }
        }
    }

    impl<T> std::fmt::Display for LinkedList<T>
    where
        T: std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut str_list = String::from("[");

            for node in self.head.as_ref() {
                let n = String::from(node.data.to_string()) + ",";
                &str_list.push_str(&n);
            }

            &str_list.push(']');
            write!(f, "{}", str_list)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::linked_list_stuff::*;
    #[test]
    fn test_new_linked_list() {
        let mut l = LinkedList::new(5);
    }

    #[test]
    fn test_list_string() {
        let l = LinkedList::new(42);
        let ls = l.to_string();

        println!("{}", ls);
    }
}
