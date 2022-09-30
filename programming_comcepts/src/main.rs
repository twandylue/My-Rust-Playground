fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 35,
        height: 55,
    };

    let rect3 = Rectangle {
        width: 20,
        height: 30,
    };

    println!("{:#?}", rect1.area());
    println!("{:#?}", rect1.can_hold(&rect2));
    println!("{:#?}", rect1.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
