struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let r = Rectangle {
        length: 50,
        width: 30,
    };

    println!("Area {}", area(&r));
}

fn area(r: &Rectangle) -> u32 {
    r.length * r.width
}
