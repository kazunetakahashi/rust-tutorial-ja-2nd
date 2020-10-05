use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数あてゲーム");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("秘密の数字 = {}", secret_number);
    loop {
        println!("入力：");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み込みに失敗しました。");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("入力された予想： {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
