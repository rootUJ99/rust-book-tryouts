use std::rc::Rc;

#[derive(Debug)]
pub enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}
