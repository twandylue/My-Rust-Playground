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
        // NOTE: Because there are several statements, it should expand to an expression with
        // enclosing code block({ ... })
        // > All syntax extensions in Rust must result in a complete, supported syntax element (such as an expression, item, etc.).
        // ref:
        // 1. https://veykril.github.io/tlborm/decl-macros/patterns/push-down-acc.html
        // 2. https://stackoverflow.com/questions/75524693/rust-macro-expansion-ignores-token-and-any-following
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
