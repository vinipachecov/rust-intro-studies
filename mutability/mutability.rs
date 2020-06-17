// Gives error
fn main() {
    let a = 10;
    a = 20;
    println!("{}", a);
}

// Allow mutability 
fn main() {
    let mut a = 10;
    a = 20;
    println!("{}", a);
}