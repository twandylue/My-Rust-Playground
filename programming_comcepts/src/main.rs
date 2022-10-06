use std::collections::HashMap;

fn main() {
    let data = vec![
        ("zeo", "Engineering"),
        ("andy", "Engineering"),
        ("leo", "Sales"),
        ("amy", "Sales"),
        ("rick", "Engineering"),
    ];

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for (name, department) in data {
        let list = map.entry(String::from(department)).or_insert(Vec::new());
        list.push(String::from(name));
    }

    // println!("{:#?}", map);
    println!("Total people: {:#?}", retrive_all_people(&map));
    println!("{:#?}", retrive_people("Sales", &map));
    println!("{:#?}", retrive_people("Engineering", &map));
}

fn retrive_all_people(map: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut total: Vec<String> = Vec::new();
    for (_, list) in map.into_iter() {
        for name in list {
            total.push(String::from(name));
        }
    }

    total.sort();
    return total;
}

fn retrive_people(department: &str, map: &HashMap<String, Vec<String>>) -> Vec<String> {
    let result = map.get(department);
    match result {
        None => panic!(),
        Some(list) => {
            let mut v = list.to_vec();
            v.sort();
            return v;
        }
    }
}
