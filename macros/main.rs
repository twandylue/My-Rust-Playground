macro_rules! literal {
    ($name:ident) => {
        stringify!($name).to_string()
    };
}

macro_rules! mans {
    ($($x:expr),*) => {
        {
            let mut temp = Human::Man(0);
            $(
                if let Human::Man(ref mut num) = temp {
                    *num += $x;
                }
            )*

            temp
        }
    };
}

macro_rules! females {
    ($($x:expr),*) => {
        {
            let mut temp = Human::Female(0);
            $(
                if let Human::Female(ref mut num) = temp {
                    *num += $x
                }
             )*
             temp
        }
    }
}

fn main() {
    println!("{}", stringify!(1 + 2 + f()));

    println!("{name}", name = literal!(andy));
    println!("{temp:?}", temp = mans!(1, 2, 3));
    println!("{temp:?}", temp = females!(1, 2, 3));
}

#[derive(Debug)]
enum Human {
    Man(i32),
    Female(i32),
}
