fn main() {
    let number = generate_fibonacci_sequence(60);
    println!("{}", number);
}

fn generate_fibonacci_sequence(depth: i64) -> i64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..depth {
        let temp = b;
        b = a + b;
        a = temp;
    };

    return a
}

