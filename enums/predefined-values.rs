#[derive(Debug)]
enum Fruits {
    apple = 1.3,
    mango = 15,
    watermelon = 30
}

fn main() {
    let f = Fruits::mango;
    println!("{:?}",f as f64);
}