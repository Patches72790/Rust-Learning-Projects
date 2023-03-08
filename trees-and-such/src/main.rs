use std::cell::RefCell;
use std::clone::Clone;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
struct AVLTree<V>
where
    V: Debug + std::cmp::Ord,
{
    value: V,
    left: Option<Rc<RefCell<AVLTree<V>>>>,
    right: Option<Rc<RefCell<AVLTree<V>>>>,
}

impl<V> AVLTree<V>
where
    V: Debug + std::cmp::Ord + Clone,
{
    pub fn new(value: V) -> Self {
        AVLTree {
            value,
            left: None,
            right: None,
        }
    }

    /// This returns the in order traversal of the items of this tree
    pub fn items(&self) -> Vec<V> {
        let left_items = match &self.left.as_ref() {
            Some(t) => t.borrow().items(),
            None => vec![],
        };
        let this_item = self.value.clone();
        let right_items = match &self.right.as_ref() {
            Some(t) => t.borrow().items(),
            None => vec![],
        };

        [&left_items[..], &[this_item], &right_items[..]].concat()
    }

    fn is_balanced(&self) -> bool {
        // ABS (height(left_subtree) - height(right_subtree)) <= 1
        todo!()
    }

    pub fn height(&self) -> usize {
        let l = match &self.left {
            Some(v) => v.try_borrow().ok().unwrap().height(),
            None => 0,
        };
        let r = match &self.right {
            Some(v) => v.try_borrow().ok().unwrap().height(),
            None => 0,
        };

        1 + l.max(r)
    }

    fn _fixup(&mut self) {
        todo!("Need to implement fixup rotations for AVL tree!")
    }

    fn _rot_left(&mut self) {
        todo!()
    }

    fn _rot_right(&mut self) {
        todo!()
    }

    pub fn insert(&mut self, value: V) {
        match value.cmp(&self.value) {
            std::cmp::Ordering::Less => match &self.left {
                Some(v) => v.borrow_mut().insert(value),
                None => self.left = Some(Rc::new(RefCell::new(AVLTree::new(value)))),
            },
            std::cmp::Ordering::Greater => match &self.right {
                Some(v) => v.borrow_mut().insert(value),
                None => self.right = Some(Rc::new(RefCell::new(AVLTree::new(value)))),
            },
            _ => self.value = value,
        }
    }
}

impl<V> std::fmt::Display for AVLTree<V>
where
    V: Debug + std::cmp::Ord,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let t = &mut AVLTree::new(1);
    t.insert(2);
    t.insert(3);
    t.insert(10);

    println!("{:?} -> {}", t.items(), t.height());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let t = &mut AVLTree::new("A");
        t.insert("B");
        t.insert("C");
        t.insert("D");

        println!("{}", t);
        println!("{}", t.height());
        println!("{:?}", t.items());

        panic!();
    }
}
