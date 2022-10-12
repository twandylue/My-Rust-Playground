fn main() {
    let both_integer = Point {x: 5, y: 10};
    let both_float = Point {x: 3.0, y: 4.0};
    println!("{:?}", both_integer.x());
    println!("{:?}", both_integer);
    println!("{:?}", both_float.distance_from_origin());
}

#[derive(Debug)]
struct Point<T>
{
    x: T,
    y: T,
}

// By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
