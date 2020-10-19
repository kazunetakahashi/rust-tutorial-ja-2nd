use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();
    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("クエリ文字列が取得できませんでした。"),
    };
    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("ファイル名が取得できませんでした。"),
    };
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let mut f = File::open(config.filename)?; // .expect("ファイルが見つかりませんでした。");
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  //.expect("ファイルの読み取りに問題がありました。");
  // println!("テキストは以下の通りです：\n{}", contents);
  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
  for line in results {
    println!("{}", line);
  }
  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
    .collect()
}
