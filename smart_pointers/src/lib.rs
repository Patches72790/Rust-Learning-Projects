use std::rc::Rc;
mod linked_list;
mod tree;

pub use tree::*;
pub use linked_list::*;

pub enum MultiList {
    Cons(i32, Rc<MultiList>),
    Nil,
}
