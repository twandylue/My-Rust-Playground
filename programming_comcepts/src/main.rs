fn main() {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    let v = vec![1, 2, 3, 4, 4, 4, 4, 5, 5, 2, 2, 3, 3, 4];

    for number in v {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut max_count: u32 = 0;
    for (_, v) in &map {
        if max_count < *v {
            max_count = *v
        }
    }

    println!("{:#?}", map);
    println!("{:#?}", max_count)
}
