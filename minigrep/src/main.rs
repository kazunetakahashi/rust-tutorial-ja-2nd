// extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("引数解析時に問題が起こりました：{}", err);
        process::exit(1);
    });
    /*
    println!(
        "{} というファイルの中から {} を探しています。",
        config.filename, config.query
    );
    */
    if let Err(e) = minigrep::run(config) {
        eprintln!("アプリケーションエラー：{}", e);
        process::exit(1);
    }
}
