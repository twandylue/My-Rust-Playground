use std::{cell::RefCell, mem, rc::Rc};

fn main() {
    let t = String::from("test");
    let a = &t[0..1];
    let a2 = &[1, 2, 3];

    assert_eq!(mem::size_of_val(&a), 16);
    assert_eq!(mem::size_of_val(&a2), 8);

    // println!("{}", std::mem::size_of::<[u8]>()); // compiler error
    assert_eq!(std::mem::size_of::<usize>(), 8);
    assert_eq!(std::mem::size_of::<String>(), 24);
    assert_eq!(std::mem::size_of::<&String>(), 8);
    assert_eq!(std::mem::size_of::<&str>(), 16);
    assert_eq!(std::mem::size_of::<&&str>(), 8);
    assert_eq!(std::mem::size_of::<&u8>(), 8);
    assert_eq!(std::mem::size_of::<&[u8]>(), 16);
    assert_eq!(std::mem::size_of::<&mut [u8]>(), 16);
    assert_eq!(std::mem::size_of::<*mut [u8]>(), 16);
    assert_eq!(std::mem::size_of::<*const [u8]>(), 16);
    assert_eq!(std::mem::size_of::<Box<[u8]>>(), 16);
    assert_eq!(std::mem::size_of::<Rc<[u8]>>(), 16);
    assert_eq!(std::mem::size_of::<Box<i32>>(), 8);
    assert_eq!(std::mem::size_of::<Rc<i32>>(), 8);
    assert_eq!(std::mem::size_of::<RefCell<i32>>(), 16);
}
