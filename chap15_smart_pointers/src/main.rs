
mod box_pointer;
mod drop_trait;
mod deref_trait;
mod rc_pointer;
mod refcell_pointer;
mod ref_cycle;
use box_pointer::List::{Cons, Nil};
use rc_pointer::RcList;
use ref_cycle::{CycList, Node};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    // box pointer in action
    let sp = Box::new(16);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3,Box::new( Nil))))));

    println!("{:?}", list);

    // dref functionality
    let x = 5;
    let y = deref_trait::MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("{} {}", x, *y);

    // drop trit in action
    let c = drop_trait::CustomSmartPointer {
        data: String::from("yoooo")
    };

    drop(c);

    let d = drop_trait::CustomSmartPointer {
        data: String::from("nooooo")
    };

    println!("custom pointers created");
    
    // refrence counting
    let e = Rc::new(RcList::Cons(1, Rc::new(RcList::Cons(2, Rc::new(RcList::Nil)))));
    println!("count after creation of e {}", Rc::strong_count(&e));
    let f = RcList::Cons(7, Rc::clone(&e));
    println!("count after creation of f {}", Rc::strong_count(&e));
    {
        let g = RcList::Cons(8, Rc::clone(&e));
    }
    println!("count after creation of g {}", Rc::strong_count(&e));
    println!("{:?} \n{:?}", e, f);


    // reference cycles

    let h = Rc::new(CycList::Cons(7, RefCell::new(Rc::new(CycList::Nil))));

    println!("h initial rc count {}", Rc::strong_count(&h));

    println!("h next item {:?}", h.tail());

    let i = Rc::new(CycList::Cons(8, RefCell::new(Rc::clone(&h))));

    println!("h  rc count after i {}", Rc::strong_count(&h));

    println!("i  rc count {}", Rc::strong_count(&i));

    println!("i next item {:?}", i.tail());

    if let Some(link) = h.tail() {
        *link.borrow_mut() = Rc::clone(&i)
    }

    println!("h strong count after borrow_mut {}", Rc::strong_count(&h));
    println!("i strong count after borrow_mut {}", Rc::strong_count(&i));

    // println!("the next item {:?}", h.tail()); // this line will induce memory leak

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf strong {} weak {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );

    // println!("leaf parent {:?}", leaf.parent.borrow().upgrade());
    {
        let branch = Rc::new(Node {
            value: 4,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });

        println!("leaf strong {} weak {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

        println!("leaf strong {} weak {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
        );
    
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    }

    // println!("branch with leaf {:?}", branch);

    println!("leaf parent {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong {} weak {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );


}
