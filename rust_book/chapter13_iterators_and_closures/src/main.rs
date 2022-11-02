use ::chapter13_iterators_and_closures::shoes_in_size;
use ::chapter13_iterators_and_closures::Shoe;

fn main() {
    let s = Shoe {
        size: 10,
        style: String::from("test"),
    };

    let a = shoes_in_size(vec![s], 10);
    println!("s: {:?}", a);
}
