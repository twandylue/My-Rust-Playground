use std::collections::HashMap;

fn main() {
    hash_map_with_closure();
    vector_with_closure();
}

struct Target {
    func: Box<dyn Fn(&str) -> String>,
}

fn hash_map_with_closure() {
    let target = Target {
        func: Box::new(|x: &str| -> String {
            println!("{}", x);
            x.to_string()
        }),
    };

    let mut hash_map: HashMap<i32, Box<dyn Fn(&str) -> String>> = HashMap::new();
    let first_key = 1;
    hash_map.insert(first_key, target.func);
    let result = hash_map.get(&first_key).unwrap();

    assert_eq!(
        result("here is closure in HashMap"),
        "here is closure in HashMap".to_string()
    );
}

fn vector_with_closure() {
    let func = |x: &str| -> String {
        println!("{}", x);
        x.to_string()
    };

    let target = Target {
        func: Box::new(func),
    };

    let mut vector: Vec<Box<dyn Fn(&str) -> String>> = Vec::new();
    vector.push(target.func);

    let result = vector[0]("here is closure in vector");

    assert_eq!(result, func("here is closure in vector"));
}
