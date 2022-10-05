fn main() {
    let words: Vec<String> = vec![
        String::from("first"),
        String::from("apple"),
        String::from("andy"),
    ];
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];

    for mut word in words {
        let first_letter = match word.chars().nth(0) {
            None => panic!(),
            Some(first) => first,
        };

        let mut result: bool = false;
        for vowel in &vowels {
            if first_letter == *vowel {
                word.push_str("-hay");
                result = true;
            }
        }

        if !result {
            word = format!("{}-{}ay", &word[1..], first_letter);
        }

        println!("{:#?}", word);
    }
}
