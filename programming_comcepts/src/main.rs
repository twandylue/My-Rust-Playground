use std::collections::HashMap;

fn main() {
    let data = vec![
        ("zeo", "Engineering"),
        ("andy", "Engineering"),
        ("leo", "Sales"),
        ("amy", "Sales"),
        ("rick", "Engineering"),
    ];

    let map = add_people(data);
    let department = "Sales";
    println!("Total people: {:#?}", retrive_all_people(&map));
    println!("---");
    println!("{}: {:#?}", department, retrive_people(department, &map));
    println!("---");
    let department = "Engineering";
    println!("{}: {:#?}", department, retrive_people("Engineering", &map));
}

fn add_people(data: Vec<(&str, &str)>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for (name, department) in data {
        let list = map.entry(String::from(department)).or_insert(Vec::new());
        list.push(String::from(name));
    }

    return map;
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
            let mut ans = list.to_vec();
            ans.sort();
            ans
        }
    }
}
