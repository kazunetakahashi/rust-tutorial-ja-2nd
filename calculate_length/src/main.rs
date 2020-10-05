fn main() {
    let s1 = String::from("hello");
    let (mut s2, len) = calculate_length0(s1);
    println!("The length of '{}' is {}.", s2, len);
    let r1 = &s2;
    let len = calculate_length1(r1);
    println!("The length of '{}' is {}.", s2, len);
    let r2 = &mut s2; // r1 はこれ以降書いてはいけない。
    add_world(r2);
    let len = calculate_length1(r2);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length0(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length1(s: &String) -> usize {
    s.len()
}

fn add_world(s: &mut String) {
    s.push_str(", world!");
}

// 次回： https://doc.rust-jp.rs/book/second-edition/ch04-03-slices.html#aスライス型
