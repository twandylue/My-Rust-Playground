# 6.2.Array

## Questions

[Link](https://practice.rs/compound-types/array.html)

## Answers

1

```rust
fn main() {
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!");
}
```

2

```rust
fn main() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];

    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}
```

3

```rust
fn main() {
    // Fill the blank
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
```

4

```rust
fn main() {
    // Fix the error
    let _arr = [1, 2, 3];

    println!("Success!");
}
```

5

```rust
fn main() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0]; // Only modify this line to make the code work!

    assert!(ele == 'a');

    println!("Success!");
}
```

6

```rust
// Fix the error
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    // let _name1 = &names[2];
    let _name1 = &names[1];

    println!("Success!");
}
```

7

```rust
// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}", s)
}
```

8

```rust
// Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = &s;

    println!("Success!");
}
```

9

```rust
fn main() {
    // You can use escapes to write bytes by their hexadecimal values
    // fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}
```

10

```rust
/* Fill in the blank and fix the errors */
fn main() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}
```

11

```rust
fn main() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");
}
```

12

```rust
fn main() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}
```

