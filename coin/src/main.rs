enum Coin {
    Sen,
    Gosen,
    Ichiman(Keio),
}

#[derive(Debug)]
enum Keio {
    Fukuzawa,
    Kitazato,
}

fn value_in_coin(coin: &Coin) -> u32 {
    match coin {
        Coin::Sen => 1000,
        Coin::Gosen => 5000,
        Coin::Ichiman(keio) => {
            println!("肖像画は {:?} です。", keio);
            10000
        }
    }
}

fn main() {
    let coin = Coin::Ichiman(Keio::Fukuzawa);
    value_in_coin(&coin);
}
