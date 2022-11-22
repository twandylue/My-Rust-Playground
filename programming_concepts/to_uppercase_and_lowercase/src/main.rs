// ref: https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust

fn main() {
    let mut t = String::from("test");
    let mut a: Vec<char> = t.chars().collect();
    a[0] = a[0].to_uppercase().nth(0).unwrap();
    t = a.iter().collect();

    println!("t: {:?}", t);
}
