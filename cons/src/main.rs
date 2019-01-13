enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}
enum ListB {
    Cons(i32, Box<ListB>),
    Nil,
}

use std::rc::Rc;

fn main() {
    simple();
    rc_way();
}
fn simple() {
    // 1 -> 2 -> 3 -> Nil
    let list = ListB::Cons(
        1,
        Box::new(ListB::Cons(
            2,
            Box::new(ListB::Cons(3, Box::new(ListB::Nil))),
        )),
    );
}

fn rc_way() {
    // b = 3 -> 5
    // a      = 5 -> 10 -> Nil
    // c = 4 -> 5
    let a = Rc::new(ListRc::Cons(
        5,
        Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ListRc::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ListRc::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
