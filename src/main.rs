fn main() {
    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1);

    println!("{}{}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
