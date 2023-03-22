
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub enum CycList {
    Cons(i32, RefCell<Rc<CycList>>),
    Nil,
}
use self::CycList::{Cons, Nil};

impl CycList {
    pub fn tail(&self) -> Option<&RefCell<Rc<CycList>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}



