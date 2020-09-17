use rand::Rng;
use std::io;

fn main() {
    println!("数あてゲーム");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret_number = {}", secret_number);
    println!("入力：");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("行の読み込みに失敗しました。");
    println!("入力された予想： {}", guess);
}
