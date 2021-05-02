use std::collections::HashMap;

fn main() {
    // Vector

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let v2 = vec![1, 2, 3];

    println!("{}", v[1]);
    let does_not_exist = v2.get(100);
    println!("is_some = {}", does_not_exist.is_some());

    for i in &v2 {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    // String

    let data = "initial contents";
    let data = data.to_string();
    println!("{}", data);

    let mut s = String::from("foo");
    s.push_str("bar");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{} len={}", s, s.len());

    // HashMap

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);
    for (key, value) in &scores {
        println!("{}={}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 10);

    scores.entry(String::from("Red")).or_insert(99);
    scores.entry(String::from("Blue")).or_insert(10);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
