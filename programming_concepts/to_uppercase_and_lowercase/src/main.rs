// ref: https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust

fn main() {
    let mut t = String::from("test");
    let mut a: Vec<char> = t.chars().collect();
    a[0] = a[0].to_uppercase().nth(0).unwrap();
    t = a.iter().collect();
    assert_eq!(t, String::from("Test"));
    println!("t: {:?}", t);

    let t2 = self::convert_to_capital("test");
    assert_eq!(t2, String::from("Test"));
    println!("t2: {:?}", t2);
}

fn convert_to_capital(word: &str) -> String {
    let mut c = word.chars();

    match c.next() {
        None => String::new(),
        Some(e) => e.to_uppercase().collect::<String>() + c.as_str(),
    }
}

