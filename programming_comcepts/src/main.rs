fn main() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("r: {}", r);
    }

    // println!("r: {}", r);
}
