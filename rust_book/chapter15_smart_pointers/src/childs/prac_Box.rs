use std::ops::Deref;

pub fn run() {
    let b = Box::new(5);
    println!("b: {}", b);

    let myb = MyBox::new(10);
    println!("myb: {}", myb.deref());

    let m1 = CustomSmartPointer {
        data: "First test".to_string(),
    };

    // print 'Second test' before 'Frist test'
    let m2 = CustomSmartPointer {
        data: "Second test".to_string(),
    };

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}
