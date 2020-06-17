fn main() {
    const MAX_HEIGHT:i32 = 90;        
    println!("{}", MAX_HEIGHT);
}

//  Leads to error
fn main2() {
    const MAX_HEIGHT:i32 = 90;    
    MAX_HEIGHT=91;
    println!("{}", MAX_HEIGHT);
}
/*
error[E0070]: invalid left-hand side of assignment
 --> constants.rs:4:15
  |
4 |     MAX_HEIGHT=91;
  |     ----------^
  |     |
  |     cannot assign to this expression
*/