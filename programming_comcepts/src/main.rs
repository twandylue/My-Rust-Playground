fn main() {
    use std::collections::HashMap;

    let mut v:Vec<i32> = vec![];
    let mut v1:Vec<i32> = vec![3, 2,5,1,23,52,62,643,63,73,4,6,10];
    v1.sort();

    for number in 1..=10 {
        v.push(number);
    }

    println!("{:?}", v);
    println!("{:?}", v[v.len()/2 -1]);

    // TODO: Median
    println!("{:?}", v1);
    println!("{:?}", v1[v1.len()/2 -1]);

}
