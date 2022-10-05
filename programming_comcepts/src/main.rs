fn main() {
    use std::collections::HashMap;

    // let text = String::from("hello world wonderful world") + &"hello";
    let text = format!("{} {} {}", "test", "test", "test");

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
