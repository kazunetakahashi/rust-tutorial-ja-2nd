use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(25);
    scores.entry(String::from("Blue")).or_insert(50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    //
    println!("{:?}", count_chars("The Promised Neverland"));
    //
    let f = File::open("neverland.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("ファイル {:?} を開く際に問題がありました。", error);
        }
    };
    let mut story = String::new();
    f.read_to_string(&mut story)
        .expect("文字列への変換に失敗しました。");
    for (key, val) in count_words(&story) {
        println!("{}: {}", key, val);
    }
}

fn count_chars(s: &str) -> HashMap<char, i32> {
    let mut cnt = HashMap::new();
    for c in s.chars() {
        let it = cnt.entry(c).or_insert(0);
        *it += 1;
    }
    cnt
}

fn count_words(s: &str) -> HashMap<&str, i32> {
    let mut cnt: HashMap<&str, i32> = HashMap::new();
    for c in s.split_whitespace() {
        let it = cnt.entry(c).or_insert(0);
        *it += 1;
    }
    cnt
}

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2(filename: &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}
