// Fix error without adding new line
pub fn number_1() {
    let s: &str = "hello, world";

    println!("Success!");
}

// Fix the error with at least two solutions
pub fn number_2() {
    let s: Box<str> = "hello, world".into();
    greetings(&s);

    fn greetings(s: &str) {
        println!("{}", s)
    }
}
