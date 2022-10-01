# Enums and Pattern Matching

- [ references ](https://doc.rust-lang.org/book/ch06-00-enums.html)

## 6.2 The match Control Flow Construct

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1, 
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            32
        },
    }
}
```

```console
Alabama
32
```

--- 

```rust
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", none);
}

// 要寫出所有可能
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

```console
None
```

--- 

```rust
// use other to cover other variety
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // _ is a special pattern in rust, means matches any value and does not bind to that value
        _ => move_player(),
        // or could return (), means doing nothing
        _ => ()
    }

    fn add_fancy_hat(){}
    fn remove_fancy_hat(){}
    fn move_player(number: u32){}
}
```

--- 

## 6.3 Concise Control Flow with if let

### if let pattern 

```rust
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("{}", max)
    } else {
        ()
    }
}
```

works the same way as 

```rust
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(x) => println!("{}", x),
        _ => (),
    }
}
```

---

## Conclusion

- Option<T>: nullable

- Some(arg): non-nullable

- match: arm pattern


