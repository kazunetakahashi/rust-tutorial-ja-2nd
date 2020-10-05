fn main() {
    another_function(5);

    let _x = 5;
    let y = {
        let _x = 3;
        _x + 1 // 式
    }; // 文
    println!("The value of y is: {}", y);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50, 100];
    for element in a.iter() {
        // 値は{}です
        println!("the value is: {}", element);
    }
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x); // xの値は{}です
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
