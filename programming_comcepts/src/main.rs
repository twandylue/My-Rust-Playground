fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest_numb(&number_list));
}

fn largest_numb(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if largest < number {
            largest = number; 
        }
    }

    return largest; 
}
