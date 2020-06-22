#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cent(c: Coin)-> u32 {
    match c {
        Coin::Penny => {
            println!("Penny!");
            return 1
        },
        Coin::Nickel => {
            println!("Nickel");
            5
        },
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("{}, value in cents", value_in_cent(Coin::Dime));    
}