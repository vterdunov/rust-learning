#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    let r1 = Rectangle {
        length: 50,
        width: 30,
    };
    let r2 = Rectangle {
        length: 40,
        width: 10,
    };

    let r3 = Rectangle {
        length: 45,
        width: 60,
    };

    println!("area rect1 {:#?}", r1.area());
    println!("can rect1 hold rect2 {}", r1.can_hold(&r2));
    println!("can rect1 hold rect3 {}", r1.can_hold(&r3));
}
