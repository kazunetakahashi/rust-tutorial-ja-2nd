use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("引数解析時に問題が起こりました：{}", err);
        process::exit(1)
    });
    println!(
        "{} というファイルの中から {} を探しています。",
        config.filename, config.query
    );
    if let Err(e) = run(config) {
        println!("アプリケーションエラー：{}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("プログラムの引数が足りません。");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?; // .expect("ファイルが見つかりませんでした。");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("ファイルの読み取りに問題がありました。");
    println!("テキストは以下の通りです：\n{}", contents);
    Ok(())
}
