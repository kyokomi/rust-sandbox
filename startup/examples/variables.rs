fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    // 整数型
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // 浮動小数点型
    let x = 2.0;
    let y: f32 = 3.0;
    println!("The value of x,y is: {},{}", x, y);

    // 数値演算
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("{} {} {} {} {}", sum, difference, product, quotient, remainder);

    // 論理値型
    let t = true;
    let f: bool = false;
    println!("{} {}", t, f);

    // 文字列型
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);

    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} {} {}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);

    // 配列型
    let a = [1, 2, 3, 4, 5];
    println!("{} {} {}", a[0], a[1], a[2]);
    let months = ["January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"];
    println!("{} {} {}", months[0], months[1], months[2]);

    // error
    //println!("The value of element is: {}", a[10]);
}