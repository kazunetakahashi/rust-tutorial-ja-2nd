fn main() {
    let s = String::from("ルベーグの優収束定理");
    let len = s.len();
    println!("{}", len);
    // println!("{}", &s[5..6]);
    for c in s.chars() {
        println!("{}", c);
    }
}
