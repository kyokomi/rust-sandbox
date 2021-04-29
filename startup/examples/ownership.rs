fn main() {
    let s = String::from("hello");
    println!("{}", s);

    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // s1はこのあと使えない
    println!("{}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone(); // ヒープデータが実際にコピーされる
    println!("{} {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s); // sの値が関数にムーブされ、ここではもう有効ではない

    // println!("{}", s); // compile error

    let x = 5;
    makes_copy(x); // i32はCopy
    println!("{}", x); // compile errorにならない

    let mut s1 = String::from("hello");
    let r1 = &mut s1;

    //let r2 = &mut s1; // 2つ可変な参照をもてないのでerror

    let len = calculate_length(r1);
    println!("The length of '{}' is {}", s1, len);

    let s = String::from("hello world");
    let word = first_word(&s[..]);
    //s.clear();
    println!("The text of '{}' size {}", word, word.len());
}

fn takes_ownership(some_string: String) {
    println!("some_string = {}", some_string);
}

fn makes_copy(x: i32) {
    println!("x = {}", x);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}