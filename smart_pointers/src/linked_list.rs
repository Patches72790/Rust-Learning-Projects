use std::mem;

pub type Link<T> = Option<Box<LinkedNode<T>>>;

pub struct LinkedNode<T> {
    pub data: T,
    pub next: Link<T>,
}

/**
 * This module contains stuff related to my initial
 * implementation of a linked list in Rust.
 */
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

    pub fn length(&self) -> usize {
        let mut len = 0;
        let mut current = &self.head;

        while let Some(node) = &current {
            len = len + 1;
            current = &node.next;
        }

        len
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
        write!(f, "<list>")
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_pop_from_linked_list() {
        let mut l = LinkedList::new(5);
        l.push(6);
        l.push(7);
        let first = &l.pop().unwrap();
        let second = &l.pop().unwrap();
        let third = &l.pop().unwrap();

        assert_eq!(*first, 7);
        assert_eq!(*second, 6);
        assert_eq!(*third, 5);
    }

    #[test]
    fn test_push_to_list() {
        let mut l = LinkedList::new(5);
        l.push(6);
        l.push(7);

        assert_eq!(l.length(), 3);
    }
}
