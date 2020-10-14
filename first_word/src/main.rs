fn main() {
    let string = String::from("My name is Kazune Takahashi.");
    println!("{}", first_word(&string));
    let string_literal = "My name is Kazune Takahashi.";
    println!("{}", first_word(&string_literal));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
