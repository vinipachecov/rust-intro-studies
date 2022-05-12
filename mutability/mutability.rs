// Gives error
// fn main() {
//     let a = 10;
//     a = 20;
//     println!("{}", a);
// }

fn calculation() -> i32 {
    return 1 + 1;
}

// Allow mutability 
fn main() {
    let mut a = 10;
    a = 20;
    const num: i32 = calculation();
    println!("{}", a);
}