#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        return self.width >= another_rect.width && self.height >= another_rect.height;
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    println!(
        "{} x {} 長方形の面積は {} です。", rect1.width, rect1.height,
        rect1.area()
    );
    println!(
        "長方形は {:?} です。", rect1
    );

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let rect4 = Rectangle::square(20);
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));
    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
}
