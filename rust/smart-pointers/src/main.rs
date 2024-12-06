use std::{cell::RefCell, rc::Rc, rc::Weak};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nada,
}

#[derive(Debug)]
enum List2 {
    Cons2(Rc<RefCell<i32>>, Rc<List2>),
    No,
}

#[derive(Debug)]
enum List3 {
    Cons3(i32, RefCell<Rc<List3>>),
    Nil,
}
impl List3 {
    fn tail(&self) -> Option<&RefCell<Rc<List3>>> {
        match self {
            Cons3(_, item) => Some(item),
            Nil => None,
        }
    }
}

use crate::List::{Cons, Nada};
use crate::List2::{Cons2, No};
use crate::List3::{Cons3, Nil};

fn ref_testing() {
    let x = Box::new(Cons(
        11,
        Box::new(Cons(
            22,
            Box::new(Cons(33, Box::new(Cons(44, Box::new(Nada))))),
        )),
    ));
    println!("{:?}", x);

    let y_2 = Rc::new(RefCell::new(20));
    let y = Cons2(
        Rc::new(RefCell::new(10)),
        Rc::new(Cons2(Rc::clone(&y_2), Rc::new(No))),
    );

    println!("{:?}", y);
    *y_2.borrow_mut() = 1234567890;
    println!("{:?}", y);
}

fn mutex_testing() {
    let a = Rc::new(Cons3(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons3(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    println!("a next item = {:?}", a.tail());
}

fn weak_rc() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn main() {
    // ref_testing();

    // mutex_testing();

    // weak_rc();
}
