use std::{
    any::type_name,
    mem::{align_of, size_of},
};

struct struct0 {
    func: Box<dyn Fn(&str) -> String>,
}

struct struct1 {
    func: Box<dyn Fn(&str) -> String>,
    type1: u32,
}

struct struct2 {
    func: Box<dyn Fn(&str) -> String>,
    type1: u8,
}

struct struct3<'a> {
    type1: &'a str,
}

struct struct4 {
    type1: String,
}

struct struct5 {
    type1: usize,
}

fn main() {
    assert_eq!(size_of::<struct0>(), 16);
    assert_eq!(align_of::<struct0>(), 8);

    assert_eq!(size_of::<struct1>(), 24);
    assert_eq!(align_of::<struct1>(), 8);

    assert_eq!(size_of::<struct2>(), 24);
    assert_eq!(align_of::<struct2>(), 8);

    assert_eq!(size_of::<struct3>(), 16);
    assert_eq!(align_of::<struct3>(), 8);

    assert_eq!(size_of::<&str>(), 16);
    assert_eq!(align_of::<&str>(), 8);

    assert_eq!(size_of::<struct4>(), 24);
    assert_eq!(align_of::<struct4>(), 8);

    assert_eq!(size_of::<String>(), 24);
    assert_eq!(align_of::<String>(), 8);

    assert_eq!(size_of::<Vec<&str>>(), 24);
    assert_eq!(align_of::<Vec<&str>>(), 8);

    assert_eq!(size_of::<struct5>(), 8);
    assert_eq!(align_of::<struct5>(), 8);

    println!(
        "type name: {}, memory size: {} bytes, memory alignment: {} bytes",
        type_name::<struct0>(),
        size_of::<struct0>(),
        align_of::<struct0>(),
    );

    println!(
        "type name: {}, memory size: {} bytes, memory alignment: {} bytes",
        type_name::<struct1>(),
        size_of::<struct1>(),
        align_of::<struct1>(),
    );

    println!(
        "type name: {}, memory size: {} bytes, memory alignment: {} bytes",
        type_name::<struct2>(),
        size_of::<struct2>(),
        align_of::<struct2>(),
    );

    println!(
        "type name: {}, memory size: {} bytes, memory alignment: {} bytes",
        type_name::<struct3>(),
        size_of::<struct3>(),
        align_of::<struct3>(),
    );

    println!(
        "type name: {}, memory size: {} bytes, memory alignment: {} bytes",
        type_name::<struct5>(),
        size_of::<struct5>(),
        align_of::<struct5>(),
    );

    println!(
        "type name: {}, memory size: {} bytes, memory alignment: {} bytes",
        type_name::<usize>(),
        size_of::<usize>(),
        align_of::<usize>(),
    );
}
