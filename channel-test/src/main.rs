extern crate rand;
use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time;

fn send_people(tx: std::sync::mpsc::Sender<&'static str>, v: std::vec::Vec<&'static str>) {
    thread::spawn(move || {
        for m in v {
            let t = rand::thread_rng().gen_range(0, 2000);
            sleep(time::Duration::from_millis(t));
            tx.send(m).unwrap();
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    let ta = vec!["谷村博士", "すなえもん", "まのひと君", "だめぽ先生"];
    send_people(tx, ta);
    let teacher = vec!["一井先生", "麻生先生", "鳥居先生", "吉野先生"];
    send_people(tx1, teacher);
    for received in rx {
        println!("{}が到着しました。", received);
    }
    // let received = rx.recv().unwrap();
}
