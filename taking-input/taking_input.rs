use std::io;

fn main() {
    let mut a = String::new();
    println!("Please enter a String");

    // IO is a module with the input and output - This call blocks the program until the read receives a \n
    io::stdin().read_line(&mut a).expect("Failed");

    let a: bool = a.trim().parse().expect("Failed to convert my number i32.");
    println!("{} World", a);
}