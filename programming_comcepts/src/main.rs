fn main() {
    let row = vec![
        Cell::Int(32), 
        Cell::Float(3.23), 
        Cell::Text(String::from("blue"))
    ];

    println!("{:?}", row)
}

#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}
