fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x); // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "   ";
    println!("spaces = '{}'", spaces);
    let spaces = spaces.len();
    println!("spaces = '{}'", spaces);

    let _guess: u32 = "23".parse().expect("Not a number!");
    // let _guess: u32 = "xxxx".parse().expect("Not a number!"); // 数字ではありません！

    let _heart_eyed_cat = '😻'; //ハート目の猫
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (tup_a, tup_b, tup_c) = tup;
    println!("tup_a = {}, tup_b = {}, tup_c = {}", tup_a, tup_b, tup_c);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!(
        "five_hundred = {}, six_point_four = {}, one = {}",
        five_hundred, six_point_four, one
    );
}
