use std::rc::Rc;

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

enum List2<T> {
    Cons(T, Rc<List2<T>>),
    Nil,
}

fn main() {
    let _list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let a = Rc::new(List2::Cons(
        5,
        Rc::new(List2::Cons(10, Rc::new(List2::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = List2::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = List2::Cons(3, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
