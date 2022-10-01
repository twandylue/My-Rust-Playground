fn main() {
    let number = generate_fibonacci_sequence(40);
    println!("{}", number);
}

fn generate_fibonacci_sequence(depth: i32) -> i32 {
    if depth <= 1 { depth }
    else {
        generate_fibonacci_sequence(depth-1) + generate_fibonacci_sequence(depth-2)
    }
}

