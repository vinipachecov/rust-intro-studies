// 
fn main() {
    let a = 10; 
    println!("{}", a);
    fun();
}

fn fun() {
    let a = 20;
    println!("{}", a);
    // ^ not found in this scope
}