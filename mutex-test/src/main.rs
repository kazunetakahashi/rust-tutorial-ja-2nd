use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(String::from("")));
    let mut handles = vec![];
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut string = counter.lock().unwrap();
            *string += &i.to_string();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
