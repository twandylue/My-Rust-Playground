use std::mem;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut a = &mut Person {
        name: String::from("andy"),
        age: 28,
    };

    let mut b = &mut Person {
        name: String::from("Paul"),
        age: 30,
    };

    println!("before:");
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    assert_eq!(a.name, "andy".to_string());
    assert_eq!(a.age, 28);
    assert_eq!(b.name, "Paul".to_string());
    assert_eq!(b.age, 30);

    // let tmp = a;
    // a = b;
    // b = tmp;
    mem::swap(&mut a, &mut b); // do not need to deal with the ownerships

    println!("after:");
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    assert_eq!(a.name, "Paul".to_string());
    assert_eq!(a.age, 30);
    assert_eq!(b.name, "andy".to_string());
    assert_eq!(b.age, 28);
}
