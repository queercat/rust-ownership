use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, None};
use crate::Value::{Boolean, Number};

#[derive(Debug)]
enum Value {
    Number(f64),
    Boolean(bool)
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<Value>>, Rc<List>),
    None()
}

impl List {
    pub fn nth(&self, index: i64) -> Option<Rc<RefCell<Value>>> {
        let mut depth = 0i64;
        let mut node = self;

        while depth < index {
            let (value , next) = match node {
                Cons(value, next) => {
                    node = next;
                    depth += 1;
                    (value, next)
                },
                _ => return Option::None
            };

            return Some(Rc::clone(value))
        }

        return Option::None
    }
}

fn main() {
    let value = Rc::new(RefCell::new(Boolean(true)));

    let tail = Rc::new(Cons(Rc::clone(&value), Rc::new(None())));
    let body = Rc::new(Cons(Rc::new(RefCell::new(Number(0f64))), Rc::clone(&tail)));
    let head = Cons(Rc::new(RefCell::new(Number(1337f64))), Rc::clone(&body));

    *value.borrow_mut() = Boolean(false);

    println!("{:?}", head);

    *head.nth(2).unwrap().borrow_mut() = Number(420f64);

    println!("{:?}", head);

    *head.nth(2).unwrap().borrow_mut() = Number(4200f64);

    println!("{:?}", head)
}
