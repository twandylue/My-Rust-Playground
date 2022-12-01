# 4.2.Number

## Questions

[Link](https://practice.rs/basic-types/char-bool-unit.html)

## Answers

1

```rust
// Make it work
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    let c3 = "w";
    assert_eq!(size_of_val(c3), 1);

    println!("Success!");
}
```

2

```rust
// Make it work
fn main() {
    let c1 = '中';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}
```

3

```rust
// Make println! work
fn main() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!");
    }
}
```

4

```rust
// Make it work
fn main() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success!");
}
```

5

```rust
// Make it work, don't modify `implicitly_ret_unit` !
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
```

6

```rust
// Modify `4` in assert to make it work
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    // unit type does't occupy any memeory space
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}
```
