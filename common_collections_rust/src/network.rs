use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (k, v) in &scores{
        println!("{k} --> {v}");
    }


    let mut usermap: HashMap<String, u32> = HashMap::new();
    map.insert("apple".to_string(), 3);
    map.insert("banana".to_string(), 7);
    map.insert("orange".to_string(), 5);

    for (k, v) in &map {
        println!("{}--> {}", k, v * 2);
    }
}

