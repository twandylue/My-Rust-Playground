# 6.1.string

## Questions

[Link](https://practice.rs/compound-types/string.html)

## Answers

1

```rust
// Fix error without adding new line
pub fn main() {
    let s: &str = "hello, world";

    println!("Success!");
}
```

2

```rust
// Fix the error with at least two solutions
pub fn number_2() {
    let s: Box<str> = "hello, world".into();
    greetings(&s);

    fn greetings(s: &str) {
        println!("{}", s)
    }
}
```
