fn main() {
    let string = String::from("Hi man");
    let slice = first_world(&string);
    println!("{}", slice);
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
