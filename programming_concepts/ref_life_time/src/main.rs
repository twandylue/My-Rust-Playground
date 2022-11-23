use std::fmt::Display;

fn main() {
    let novel = String::from("My name is Andy");
    let test = String::from("test");
    let ret = self::longest_with_an_announcement(novel.as_str(), test.as_str(), "String");

    assert_eq!(String::from("My name is Andy"), ret);

    let mut bytes = ByteIter { remainder: b"1" };
    let byte1 = bytes.next();
    let byte2 = bytes.next();
    if byte1 == byte2 {}
}

struct ByteIter<'remainder> {
    remainder: &'remainder [u8],
}

impl<'remainder> ByteIter<'remainder> {
    // ERROR because of lifetime annotation
    // fn next<'mut_self>(&'mut_self mut self) -> Option<&'mut_self u8> {
    fn next<'mut_self>(&'mut_self mut self) -> Option<&'remainder u8> {
        if self.remainder.is_empty() {
            None
        } else {
            let byte = &self.remainder[0];
            self.remainder = &self.remainder[1..];
            Some(byte)
        }
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    assert_eq!(
        String::from("Announcement: String"),
        format!("Announcement: {}", ann)
    );

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
