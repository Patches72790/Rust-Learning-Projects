use std::clone::Clone;
use std::fmt::Debug;

#[derive(Debug)]
struct AVLTree<V>
where
    V: Debug + std::cmp::Ord,
{
    value: V,
    left: Option<Box<AVLTree<V>>>,
    right: Option<Box<AVLTree<V>>>,
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
        let left_items = match self.left.as_ref() {
            Some(t) => t.items(),
            None => vec![],
        };
        let this_item = self.value.clone();
        let right_items = match self.right.as_ref() {
            Some(t) => t.items(),
            None => vec![],
        };

        [&left_items[..], &[this_item], &right_items[..]].concat()
    }

    fn is_balanced(&self) -> bool {
        let left_height = self.left.as_ref().map_or(0, |left| left.height());
        let right_height = self.right.as_ref().map_or(0, |right| right.height());

        left_height.abs_diff(right_height) <= 1
    }

    pub fn balance_factor(&self) -> i8 {
        let left_height = self.left.as_ref().map_or(0, |left| left.height());
        let right_height = self.right.as_ref().map_or(0, |right| right.height());

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }

    pub fn height(&self) -> usize {
        1 + std::cmp::max(
            self.left.as_ref().map_or(0, |node| node.height()),
            self.right.as_ref().map_or(0, |node| node.height()),
        )
    }

    fn rebalance(&mut self) -> bool {
        match self.balance_factor() {
            -2 => {
                let right_node = self.right.as_mut().unwrap();

                if right_node.balance_factor() == 1 {
                    right_node.rotate_right();
                }

                self.rotate_left();
                true
            }
            2 => {
                let left_node = self.left.as_mut().unwrap();

                if left_node.balance_factor() == -1 {
                    left_node.rotate_left();
                }

                self.rotate_right();

                true
            }
            _ => false,
        }
    }

    ///
    ///           B
    ///         /  \
    ///        A    D
    ///           /  \
    ///          C   E
    ///
    ///           D
    ///         /  \
    ///        B    E
    ///      /  \
    ///     A   C
    ///
    fn rotate_left(&mut self) -> bool {
        if self.right.is_none() {
            return false;
        }

        let right_node = self.right.as_mut().unwrap();

        let right_left_tree = right_node.left.take();
        let right_right_tree = right_node.right.take();

        let mut new_left_tree = std::mem::replace(&mut self.right, right_right_tree);

        std::mem::swap(&mut self.value, &mut new_left_tree.as_mut().unwrap().value);

        let left_tree = self.left.take();

        let new_left_node = new_left_tree.as_mut().unwrap();
        new_left_node.right = right_left_tree;
        new_left_node.left = left_tree;

        self.left = new_left_tree;
        true
    }

    ///
    ///           D
    ///         /  \
    ///        B    E
    ///      /  \
    ///     A   C
    ///
    ///          B
    ///        /   \
    ///       A     D
    ///            / \
    ///           C   E
    ///
    fn rotate_right(&mut self) -> bool {
        if self.left.is_none() {
            return false;
        }

        let left_node = self.left.as_mut().unwrap();

        // get left and right subtrees
        let left_right_tree = left_node.right.take();
        let left_left_tree = left_node.left.take();

        // put A into left position
        let mut new_right_tree = std::mem::replace(&mut self.left, left_left_tree);

        // swap values of D and B without having to swap root node
        std::mem::swap(&mut self.value, &mut new_right_tree.as_mut().unwrap().value);

        // take E former right tree
        let right_tree = self.right.take();

        // Put C and E into place as left and right of D
        let new_right_node = new_right_tree.as_mut().unwrap();
        new_right_node.left = left_right_tree;
        new_right_node.right = right_tree;

        // set right node of root as D
        self.right = new_right_tree;
        true
    }

    pub fn insert(&mut self, value: V) {
        match value.cmp(&self.value) {
            std::cmp::Ordering::Less => match self.left.as_mut() {
                Some(v) => v.insert(value),
                None => self.left = Some(Box::new(AVLTree::new(value))),
            },
            std::cmp::Ordering::Greater => match self.right.as_mut() {
                Some(v) => v.insert(value),
                None => self.right = Some(Box::new(AVLTree::new(value))),
            },
            _ => self.value = value,
        }

        self.rebalance();
    }

    pub fn delete(&mut self, value: V) -> Option<V> {
        todo!()
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

    t.is_balanced();
    println!("{:?} -> {}", t.items(), t.height());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_balanced_imbalanced() {
        let t = &mut AVLTree::new("A");
        t.insert("B");
        t.insert("C");
        t.insert("D");
        t.insert("E");

        println!("{}", t);
        assert!(t.is_balanced());
    }
    #[test]
    fn test_is_balanced_imbalanced_2() {
        let t = &mut AVLTree::new(1);

        for i in 2..=100 {
            t.insert(i);
        }

        println!("{}", t);
        println!("{}", t.left.as_ref().unwrap().height());
        println!("{}", t.right.as_ref().unwrap().height());

        assert!(t.is_balanced());
    }

    #[test]
    fn test_is_balanced_balanced() {
        let t = &mut AVLTree::new("A");
        t.insert("B");
        println!("{}", t);
        assert!(t.is_balanced());
    }
    #[test]
    fn test_is_balanced_balanced_2() {
        let t = &mut AVLTree::new("C");
        t.insert("B");
        t.insert("E");
        t.insert("A");
        t.insert("F");
        println!("{}", t);
        assert!(t.is_balanced());
    }

    #[test]
    fn test_rotations() {
        let t = &mut AVLTree::new("C");
        t.insert("B");
        t.insert("E");
        t.insert("A");
        t.insert("F");
        println!("{}", t);

        t.rotate_left();
        t.rotate_right();

        println!("{:?}", t.items());
    }
}
