trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// <Type as Trait>::function(receiver_if_method, next_arg, ...);
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
