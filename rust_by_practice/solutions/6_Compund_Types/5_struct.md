# 6.5.Struct

## Questions

[Link](https://practice.rs/compound-types/struct.html)

## Answers

1

```rust
// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("test"),
    };

    println!("Success!");
}
```

2

Unit struct don't have any fields.  
It can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

```rust
struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {}
fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

// Fill the blank to make the code work
fn do_something_with_unit(u: Unit) {}
```

3

```rust
// Fix the error and fill the blanks
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let v = Point(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Point) {
    // interesting
    let Point(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}
```

4

You can make a whole struct mutable when instantiating it, but Rust doesn't allow us to mark only certain fields as mutable.

```rust
// Fill the blank and fix the error without adding/removing new line
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}
```

5

```rust
// Fill the blank
struct Person {
    name: String,
    age: u8,
}

fn main() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person {
    Person { age, name }
}
```

6

```rust
// Fill the blank to make the code work
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("{:#?}", u2);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}
```

7

```rust
// Fill the blanks to make the code work
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:#?}", rect1); // Print debug info to stdout
}
```

## Partial move

Within the destructuring of a single variable, both by-move and by-reference pattern bindings can be used at the same time.  
Doing this will result in a partial move of the variable, which means that parts of the variable will be moved while other parts stay.  
In such a case, the parent variable cannot be used afterwards as a whole, however the parts that are only referenced (and not moved) can still be used.

Example

```rust
// Fill the blanks to make the code work
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:#?}", rect1); // Print debug info to stdout
}
```

8

```rust
// Fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = f.name;

    // ONLY modify this line
    // println!("{}, {}, {:?}", f.data, f);
    println!("{:?}", f.data);
}
```
