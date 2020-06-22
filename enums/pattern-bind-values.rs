#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alaska,
    Arizona
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
        Coin::Quarter(state) => {
            println!("{:?}",state);
            25
        },
    }
}

fn main() {
    println!("{}, value in cents", value_in_cent(Coin::Quarter(UsState::Alaska)));    
}