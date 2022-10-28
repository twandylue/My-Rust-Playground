# Common Collections

- [ references ](https://doc.rust-lang.org/book/ch08-01-vectors.html)

## 8.1 Storing Lists of Values with Vectors

- Vectors can only store values that are the same type.
- Using an `Enum` to store multiple types

```rust
// specify data type
let v: Vec<i32> = Vec::new();

// auto specify data type by macro vec!
// equal to Vec<i32>
let v2 = vec![1,2,3];
```

---

```rust
fn main() {
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[100];
    println!("{}", third);

    // .get() may reture null
    let third: Option<&i32> = v.get(2);
    match third {
        None => println!("There is no third element."),
        Some(third) => println!("{}", third)
    }
    
    // sugar syntax
    if let Some(third) = third {
        println!("{}", third)
    } else {
        println!("There is no third element.")
    }
}
```

--- 

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // immutable borrow

    v.push(6); // error: already use immutable borrow in the vector

    println!("The first element is: {}", first);
}
```

---

```rust
fn main() {
    let mut v = vec![1,2,3,1222,333];
    for i in &mut v {
        /*
        In order to change the mutable reference refers to, 
        we have to use the * dereference operator to get the value in i before we can use the += operator.
        */
        *i += 50; 
    }

    println!("{:#?}", v);
}
```

---


