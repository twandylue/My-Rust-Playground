fn main() {
    let config_max: Option<i8> = None;
    // let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("{}", max)
    } else {
        ()
    }

    match config_max {
        Some(x) => println!("{}", x),
        _ => (),
    }
}

