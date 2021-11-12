
use std::rc::Rc;

pub enum MultiList {
    Cons(i32, Rc<MultiList>),
    Nil,
}
