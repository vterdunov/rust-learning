#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let r = Rectangle {
        length: 50,
        width: 30,
    };

    println!("area {:#?}", r.area());
}
