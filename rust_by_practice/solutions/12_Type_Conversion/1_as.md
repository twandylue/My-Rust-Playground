# 12.1.Convert by as

## Questions

[Link](https://practice.rs/type-conversions/as.html)

## Answers

1

```rust
// FIX the errors and FILL in the blank
// DON'T remove any code
fn main() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = decimal as u8 as char;
    let c2 = integer as char;

    println!("{}", integer);
    assert_eq!(integer, 'b' as u8 - 1);

    println!("Success!");
}
```

2

```rust
#[allow(overflowing_literals)]
fn main() {
    assert_eq!(u8::MAX, 255);
    // The max of `u8` is 255 as shown above.
    // so the below code will cause an overflow error: literal out of range for `u8`.
    // PLEASE looking for clues within compile errors to FIX it.
    // DON'T modify any code in main.
    //
    let v = 1000 as u8;

    println!("{}", v); // 232 TODO: WHY???

    println!("Success!");
}
```

3

```rust
#[allow(overflowing_literals)]
fn main() {
    assert_eq!(1000 as u16, 1000);

    assert_eq!(1000 as u8, 232);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
```

4

```rust
// FILL in the blanks
fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize;
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // p2 points to the 2nd element in values
    unsafe {
        // Add one to the second element
        *p2 += 1;
    }

    assert_eq!(values[1], 3);

    println!("Success!");
}
```

5

TODO:

```rust
fn main() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe { assert_eq!(std::mem::size_of_val(&*b), 13) }

    println!("Success!");
}
```
