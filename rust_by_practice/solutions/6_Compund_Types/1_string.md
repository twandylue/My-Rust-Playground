# 6.1.String

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
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}", s)
}
```

3

```rust
// Fill the blank
fn main() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
```

4

```rust
// Fix all errors without adding newline
fn main() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}
```

5

```rust
// Fill the blank
fn main() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");
    println!("original: {}", s);

    assert_eq!(s1, "I like cats");

    println!("Success!");
}
```

6

```rust
// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
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
