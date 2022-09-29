fn main() {
    let rect = Rectangle::square(10);
    let area = Rectangle::area(&rect, &rect);

    println!("{:#?}", rect);

    println!("{}", rect.area(&rect));
    println!("{}", area);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square (size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area (&self, rect: &Rectangle) -> u32 {
        rect.height * rect.width
    }
}
