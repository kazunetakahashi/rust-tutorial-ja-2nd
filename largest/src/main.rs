use std::cmp;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let string1 = String::from("long string is long");
    let res;
    {
        let string2 = String::from("xyz");
        res = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", res);
    }
    // println!("The longest string is {}", res);
}

fn largest<T>(v: &Vec<T>) -> T
where
    T: std::cmp::Ord + Copy,
{
    assert!(v.len() >= 1, "The list size <= 0.");
    let mut maxi = v[0];
    for i in v.iter() {
        maxi = cmp::max(maxi, *i);
    }
    maxi
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
