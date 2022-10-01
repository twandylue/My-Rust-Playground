fn main() {
    let x = value(Coin::Nickel);
    println!("{:#?}", x);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1, 
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter => 25,
    }
}
