use std::{mem, rc::Rc};

fn main() {
    let t = String::from("test");
    let a = &t[0..1];
    let a2 = &[1, 2, 3];

    println!("bytes: {}", mem::size_of_val(&a));
    println!("bytes: {}", mem::size_of_val(&a2));

    // println!("{}", std::mem::size_of::<[u8]>()); // compiler error
    // let test = &1;
    println!("{}", std::mem::size_of::<usize>()); // 8
    println!("{}", std::mem::size_of::<String>()); // 8
    println!("{}", std::mem::size_of::<&str>()); // 8
    println!("{}", std::mem::size_of::<&u8>()); // 8
    println!("{}", std::mem::size_of::<&[u8]>()); // 16
    println!("{}", std::mem::size_of::<&mut [u8]>()); // 16
    println!("{}", std::mem::size_of::<*mut [u8]>()); // 16
    println!("{}", std::mem::size_of::<*const [u8]>()); // 16
    println!("{}", std::mem::size_of::<Box<[u8]>>()); // 16
    println!("{}", std::mem::size_of::<Rc<[u8]>>()); // 16
}
