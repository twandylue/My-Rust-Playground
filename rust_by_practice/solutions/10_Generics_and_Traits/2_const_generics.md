# 10.2.Constant Generics

## Questions

[Link](https://practice.rs/generics-traits/const-generics.html)

## Answers

Const generics are generic arguments that range over constant values, rather than types or lifetimes.
This allows, for instance, types to be parameterized by integers.
In fact, there has been one example of const generic types since early on in Rust's development: the array types [T; N], for some type T and N: usize.
However, there has previously been no way to abstract over arrays of an arbitrary size: if you wanted to implement a trait for arrays of any size, you would have to do so manually for each possible value.
For a long time, even the standard library methods for arrays were limited to arrays of length at most 32 due to this problem.

Example

```rust
fn foo<const N: usize>() {}

fn bar<T, const M: usize>() {
    foo::<M>(); // Okay: `M` is a const parameter
    foo::<2021>(); // Okay: `2021` is a literal
    foo::<{ 20 * 100 + 20 * 10 + 1 }>(); // Okay: const expression contains no generic parameters

    foo::<{ M + 1 }>(); // Error: const expression contains the generic parameter `M`
    foo::<{ std::mem::size_of::<T>() }>(); // Error: const expression contains the generic parameter `T`

    let _: [u8; M]; // Okay: `M` is a const parameter
    let _: [u8; std::mem::size_of::<T>()]; // Error: const expression contains the generic parameter `T`
}

fn main() {}
```

1

<T, const N: usize> is part of the struct type, it means Array<i32, 3> and Array<i32, 4> are different types.

```rust
struct Array<T, const N: usize> {
    data: [T; N],
}

fn main() {
    let arrays = [
        Array { data: [1, 2, 3] },
        Array {
            data: [1.0, 2.0, 3.0],
        },
        Array { data: [1, 2] },
    ];

    println!("Success!");
}
```

2

```rust
use std::fmt::Debug;

// Fill in the blanks to make it work.
fn print_array<T: Debug, const U: usize>(arr: [T; U]) {
    println!("{:?}", arr);
}
fn main() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}
```

3

```rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

// fix the errors in main
fn main() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // &str is a string reference, containing a pointer and string length in it, so it takes two word long, in x86-64, 1 word = 8 bytes
    check_size([(); 31].map(|_| "hello你好".to_string())); // String is a smart pointer struct, it has three fields: pointer, length and capacity, each takes 8 bytes
    check_size(['中'; 191]); // A char takes 4 bytes in Rust
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
```
