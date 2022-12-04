# 6.3.slice

## Questions

[Link](https://practice.rs/compound-types/slice.html)

## Answers

1

```rust
// Fix the errors, DON'T add new lines!
fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world";

    println!("Success!");
}
```

2 (TODO:)

```rust
fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    println!("{:?}", slice);
    println!("{}", std::mem::size_of_val(&'人'));
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}
```

3

```rust
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}
```

4

```rust
fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];

    println!("{}", slice1); // he

    assert_eq!(slice1, slice2);

    println!("Success!");
}
```

5

```rust
fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3];

    assert!(slice == "你");

    println!("Success!");
}
```

6

```rust
// Fix errors
fn main() {
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_word` need a `&str` type.
    // It works because `&String` can be implicitly converted to `&str, If you want know more ,this is called `Deref`
    let word = first_word(&s);

    println!("the first word is: {}", word);

    s.clear(); // error!

    // println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    &s[..1]
}
```
