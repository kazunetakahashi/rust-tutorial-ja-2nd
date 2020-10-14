mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_wait_list();
    front_of_house::hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    // パンを指定して注文
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // 次の行は失敗する。
    // back_of_house::Breakfast::seasonal_fruit は pub ではないから。
    // meal.seasonal_fruit = String::from("blueberries");
}
