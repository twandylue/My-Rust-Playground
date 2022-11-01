fn main() {
    let mut number = 100;
    println!("number before :{}", number);
    num_add(&mut number);
    println!("number after :{}", number);

    let mut c = card {
        number: 100,
        name: String::from("A"),
    };

    println!("card number before: {}", c.number);
    card_number_add(&mut c);
    println!("card number after: {}", c.number);

    println!("card name before: {}", c.name);
    println!("address of card name before: {:p}", &c);
    card_change_name(&mut c);
    println!("card name after: {}", c.name);
    println!("address of card name after: {:p}", &c);
}

fn card_change_name(card: &mut card) {
    // card.name.clear();
    card.name = String::from("B");
}

fn card_number_add(card: &mut card) {
    // WARNING: auto-deferencing
    // (*card).number += 1;
    card.number += 1;
}

fn num_add(number: &mut i32) {
    // *number += 1;
    *number += 1;
}

struct card {
    number: i32,
    name: String,
}
