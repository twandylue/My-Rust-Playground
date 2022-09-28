fn main() {
    let target = 20;
    generate_fibonacci_sequence(0, 1, target);
}

fn generate_fibonacci_sequence(pre_number: u32, mut cur_number: u32, countdown: u32) {

    if countdown == 0 {return}

    let temp = cur_number;
    cur_number = pre_number + cur_number;
    println!("{countdown}th Number: {cur_number}");

    generate_fibonacci_sequence(temp, cur_number, countdown - 1);
}

