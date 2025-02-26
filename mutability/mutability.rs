// Gives error
// fn main() {
//     let a = 10;
//     "let" keyword DOES NOT mutating values in a variable
//     a = 20;
//     println!("{}", a);
// }

const GLOBAL_MAX:i32 = 10000;

fn calculation() -> i32 {
    return 1 + 1;
}

// Allow mutability 
fn main() {    
    

    let mut a = 10;
    println!("A before {}", a);
    a = 20;
    let num: i32 = calculation();
    println!("A after {}", a);

    println!("{}", GLOBAL_MAX);
}