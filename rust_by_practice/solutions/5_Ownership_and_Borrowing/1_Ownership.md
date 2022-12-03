# 5.1.Ownership

## Questions

[Link](https://practice.rs/ownership/ownership.html)

## Answers

1

```rust
fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}", x, y);
}
```

2

```rust
// Don't modify code in main!
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
```

3

```rust
fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}
```

4

```rust
// Fix the error without removing code line
fn main() {
    let s = String::from("hello, world");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &str) {
    println!("{}", s)
}
```

5

```rust
// don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}
```

6

Mutability can be changed when ownership is transferred.

```rust
fn main() {
    let s = String::from("hello, ");

    // Modify this line only !
    let mut s1 = s;

    s1.push_str("world");

    println!("Success!");
}
```

7

```rust
fn main() {
    let x = Box::new(5);

    let mut y = Box::new(3);

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}
```

## Partial Move

Example

```rust
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}
```

8

```rust
fn main() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
}
```

9

```rust
fn main() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    // let (s1, s2) = (t.0.clone(), t.1.clone());
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
```
