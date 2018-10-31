fn main() {
    let f = fib(45);
    println!("{}", f)
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
