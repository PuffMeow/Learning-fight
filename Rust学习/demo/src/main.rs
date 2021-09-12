use crate::List::{Cons, Nil};

fn main() {
    //形成类似于这样的一个链表结构： 1->2->3->null
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
