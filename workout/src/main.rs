use std::collections::hash_map::Entry::*;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

/*
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    // ゆっくり計算します
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
*/

struct Cacher<'a, T, U, V>
where
    T: Fn(&U) -> &V,
    U: std::cmp::Eq + std::hash::Hash,
{
    calculation: T,
    hash: HashMap<&'a U, &'a V>,
}

impl<'a, T, U, V> Cacher<'a, T, U, V>
where
    T: Fn(&U) -> &V,
    U: std::cmp::Eq + std::hash::Hash,
{
    fn new(calculation: T) -> Cacher<'a, T, U, V> {
        Cacher {
            calculation,
            hash: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &'a U) -> &V {
        match self.hash.entry(arg) {
            Occupied(e) => e.get(),
            Vacant(e) => e.insert((self.calculation)(arg)),
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            // 今日は{}回腕立て伏せをしてください！
            "Today, do {} pushups!",
            expensive_result.value(&intensity)
        );

        println!(
            // 次に、{}回腹筋をしてください！
            "Next, do {} situps!",
            expensive_result.value(&intensity)
        );
    } else {
        if random_number == 3 {
            // 今日は休憩してください！水分補給を忘れずに！
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                // 今日は、{}分間走ってください！
                "Today, run for {} minutes!",
                expensive_result.value(&intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
