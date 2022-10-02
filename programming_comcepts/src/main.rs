fn main() {
    let number_1 = generate_fibonacci_sequence(30);
    let number_2 = generate_fibonacci_sequence_recursion(30);
    assert_eq!(number_1, number_2);
    println!("Number_1: {}, Number_2: {}", number_1, number_2);
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

fn generate_fibonacci_sequence_recursion(depth: i64) -> i64 {
    if depth <= 1 {depth}
    else {
        generate_fibonacci_sequence_recursion(depth-1) + generate_fibonacci_sequence_recursion(depth-2)
    }
}
