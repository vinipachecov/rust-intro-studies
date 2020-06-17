use std::io;

fn main() {
    let mut operation = String::new();
    let a = 10;
    let b = 20;
    println!("Choose operation to be performed: ");
    println!("1. + \n2. - \n3. * \n4. /");
    io::stdin().read_line(&mut operation).expect("Failed reading operation");
     operation = operation.trim().to_string();

    if operation =="+" {
        println!("{}",a + b);
    } else if operation == "-" {
        println!("{}",a - b);
    } else if operation == "/" {
        println!("{}",a / b);
    } else  {
        println!("{}",a * b);
    } 
}