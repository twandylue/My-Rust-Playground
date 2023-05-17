macro_rules! literal {
    ($name:ident) => {
        stringify!($name).to_string()
    };
}

// TODO:
// macro_rules! add_fun{
//     ($name:ident)  => {
//         add()
//     }
// }

fn main() {
    println!("Hello");
    println!("{}", stringify!(1 + 2 + f()));
    println!("{}", literal!(andy));
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
