fn main() {
    hashmaps();
}

fn _vectors() {
    // let v: Vec<i32> = Vec::new();

    // let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
}

use std::collections::HashMap;
fn hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Yellow"), 25);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

