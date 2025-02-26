// Move function 
// The reason is to avoid double free memory and double ownership for a single 
// memory space.
fn main() {
    let a = String::from("hello"); 
    let b = a;
    println!("{} {}", b, a);

    let a = String::from("hello2"); 
    maybe_move(a);

    println!("after maybe move {}", a);
    //  value moved here
    //  |     println!("{} {}", b, a);
    //   |                          ^ value borrowed here after move
}

 fn maybe_move(&s1: String) {
    println!("Maybe move {:?}", &s1);
 }


fn working_move() {
    let a = String::from("hello"); 
    let b = a.clone(); // this generates a whole new copy from a to b.
    //  This means new value and new address in memory.
    println!("{} {}", b, a);
    //  value moved here
    //  |     println!("{} {}", b, a);
    //   |    `                      ^ value borrowed here after move
}