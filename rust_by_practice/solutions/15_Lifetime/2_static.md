# 15.2.&'static and T:'static

## Questions

[Link](https://practice.rs/lifetime/static.html)

## Answers

1

```rust
/* Fill in the blank in two ways */
fn main() {
    let v: &str = "hello";
    need_static(v);

    println!("Success!")
}

fn need_static(r: &'static str) {
    assert_eq!(r, "hello");
}
```

2

Another way to make 'static lifetime is using Box::leak

```rust
#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut config: Option<&mut Config> = None;

/* Make it work without changing the function signatures of `init`*/
fn init() -> Option<&'static mut Config> {
    let mut c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    Some(Box::leak(c))
}

fn main() {
    unsafe {
        config = init();

        println!("{:?}", config)
    }
}
```

3

```rust
fn main() {
    // {
    // Make a `string` literal and print it:
    let static_string = "I'm in read-only memory";
    println!("static_string: {}", static_string);

    // When `static_string` goes out of scope, the reference
    // can no longer be used, but the data remains in the binary.
    // }

    println!("static_string reference remains alive: {}", static_string);
}
```

4

'static can be coerced to a shorter lifetime

```rust
// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
```

5

```rust
/* Make it work */
use std::fmt::Debug;

fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it2<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    const i: i32 = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);

    print_it1(&i);

    // but this one WORKS !
    print_it2(&i);
}
```

6

```rust
use std::fmt::Display;

fn main() {
    let mut string = "First".to_owned();

    string.push_str(string.to_uppercase().as_str());
    print_a(&string);
    print_b(&string);
    print_c(Box::leak(Box::new(string.clone()))); // Compilation error
    print_d(Box::leak(Box::new(string.clone()))); // Compilation error
    print_e(&string);
    print_f(&string);
    print_g(Box::leak(Box::new(string.clone()))); // Compilation error
}

fn print_a<T: Display + 'static>(t: &T) {
    println!("{}", t);
}

fn print_b<T>(t: &T)
where
    T: Display + 'static,
{
    println!("{}", t);
}

fn print_c(t: &'static dyn Display) {
    println!("{}", t)
}

fn print_d(t: &'static impl Display) {
    println!("{}", t)
}

fn print_e(t: &(dyn Display + 'static)) {
    println!("{}", t)
}

fn print_f(t: &(impl Display + 'static)) {
    println!("{}", t)
}

fn print_g(t: &'static String) {
    println!("{}", t);
}
```
