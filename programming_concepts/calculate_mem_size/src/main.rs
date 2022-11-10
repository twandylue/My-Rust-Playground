use std::{
    any::type_name,
    mem::{align_of, size_of},
};

struct struct1 {
    func: Box<dyn Fn(&str) -> String>,
    type1: u8,
    type2: u32,
}

struct struct2 {
    func: Box<dyn Fn(&str) -> String>,
    type1: u8,
    type2: u8,
}

struct struct3<'a> {
    type1: &'a str,
}

struct struct4 {
    type1: usize,
}

fn main() {
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
        type_name::<struct4>(),
        size_of::<struct4>(),
        align_of::<struct4>(),
    );

    println!(
        "type name: {}, memory size: {} bytes, memory alignment: {} bytes",
        type_name::<usize>(),
        size_of::<usize>(),
        align_of::<usize>(),
    );
}
