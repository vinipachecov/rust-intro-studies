// Move function 
// The reason is to avoid double free memory and double ownership for a single 
// memory space.
fn main() {
    let a = String::from("hello"); 
    let b = a;
    println!("{} {}", b, a);
    //  value moved here
    //  |     println!("{} {}", b, a);
    //   |                          ^ value borrowed here after move
}



fn working_move() {
    let a = String::from("hello"); 
    let b = a.clone();
    println!("{} {}", b, a);
    //  value moved here
    //  |     println!("{} {}", b, a);
    //   |                          ^ value borrowed here after move
}