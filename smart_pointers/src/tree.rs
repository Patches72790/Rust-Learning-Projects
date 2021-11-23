use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

/**
 * Refcell variant allows for runtime checking of borrowing invariants
 * namely many immutable borrows, but only one mutable borrow at a time.
 */

#[derive(Debug)]
pub struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn add_node(&mut self, data: i32) {
        let new_node = Node::new(data);
        let mut position = 0;

        for child in self.children.borrow_mut().iter() {
            if child.value > data {
                break;
            }
            position += 1;
        }

        let rc_node = Rc::new(new_node);
        self.children.borrow_mut().insert(position, rc_node);
    }

    pub fn new(value: i32) -> Node {
        Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }

    pub fn level_order_print(&self, depth: u32) {
        let mut spaces = String::from("");
        for _ in 0..depth {
            spaces.push_str("--");
        }

        println!("{}<{}> depth: {}", spaces, self.value, depth);

        for child in self.children.borrow().iter() {
            child.level_order_print(depth + 1);
        }
    }
}

pub fn test_my_tree() {
    let mut my_node = Node::new(25);
    my_node.add_node(20);
    my_node.add_node(30);
    my_node.add_node(40);

    my_node.level_order_print(0);
}

pub fn tree_node_stuff() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        parent: RefCell::new(Weak::new()),
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // increment the weak count of leaf.parent
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());
}
