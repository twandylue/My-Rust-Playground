# 18.1.Closure

## Questions

[Link](https://practice.rs/functional-programing/closure.html)

## Answers

1

```rust
/* Make it work with least amount of changes*/
fn main() {
    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    print();
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;

    println!("{}", color);
}
```

2

```rust
/* Make it work
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
fn main() {
    let mut count = 0;

    let mut inc = move || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc(); // 1

    let _reborrow = &count;
    println!("{}", _reborrow); // 0

    inc(); // 2
    println!("{}", _reborrow); // 0

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;
    println!("{}", _count_reborrowed); // 0

    assert_eq!(count, 0);
}
```

3.1

```rust
/* Make it work in two ways, none of them is to remove `take(movable)` away from the code
*/
fn main() {
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        take(movable);
    };

    // consume();
    consume(); // 3
}

fn take<T>(_v: T) {}
```

3.2

```rust
/* Make it work in two ways, none of them is to remove `take(movable)` away from the code
*/
fn main() {
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        take(&movable);
    };

    consume(); // 3
    consume(); // 3
}

fn take<T>(_v: T) {}
```

## Type inferred

The following four closures has no difference in input and return types.

```rust
#![allow(unused)]
fn main() {
  fn  add_one_v1   (x: u32) -> u32 { x + 1 }
  let add_one_v2 = |x: u32| -> u32 { x + 1 };
  let add_one_v3 = |x|             { x + 1 };
  let add_one_v4 = |x|               x + 1  ;
}
```

4

```rust
fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    /* Make it work, only change the following line */
    let n = example_closure(String::from("5"));
}
```

## Fn, FnMut, FnOnce

When taking a closure as an input parameter, the closure's complete type must be annotated using one of the following traits:

- Fn: the closure uses the captured value by reference (&T)
- FnMut: the closure uses the captured value by mutable reference (&mut T)
- FnOnce: the closure uses the captured value by value (T)

5.1

```rust
/* Make it work by changing the trait bound, in two ways*/
fn fn_once<F>(func: F)
where
    F: Fn(usize) -> bool,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len())
}
```

5.2

```rust
/* Make it work by changing the trait bound, in two ways*/
fn fn_once<F>(func: F)
where
    F: (FnOnce(usize) -> bool) + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len())
}
```

6

```rust
fn main() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}", s);
}

/* Fill in the blank */
fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}
```

7

```rust
/* Fill in the blank */

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F)
where
    // The closure takes no input and returns nothing.
    F: FnOnce(),
{
    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32
where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
```

8

```rust
/* Fill in the blank */
fn main() {
    let mut s = String::new();

    let update_string = |str| -> String {
        s.push_str(str);
        s
    };

    exec(update_string);
}

fn exec<'a, F: FnOnce(&'a str) -> String>(f: F) {
    f("hello");
}
```

## Input functions

Since closure can be used as arguments, you might wonder can we use functions as arguments too? And indeed we can.

9

```rust
/* Implement `call_me` to make it work */
fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}

fn main() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
```

10.1

```rust
/* Fill in the blank using two aproaches,
and fix the errror */
fn create_fn() -> impl Fn(i32) -> i32 {
    let num = 5;

    // How does the following closure capture the environment variable `num`
    // &T, &mut T, T ?
    move |x| x + num
}

fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}
```

10.2

```rust
/* Fill in the blank using two aproaches,
and fix the errror */
fn create_fn() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    // How does the following closure capture the environment variable `num`
    // &T, &mut T, T ?
    Box::new(move |x| x + num)
}

fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}
```

11

```rust
/* Fill in the blank and fix the error*/
fn factory(x: i32) -> Box<dyn FnOnce(i32) -> i32> {
    let num = 5;

    if x > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x + num)
    }
}

fn main() {}
```

## Closure in structs

```rust
struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher { query, value: None }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn main() {}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 1);
}
```
