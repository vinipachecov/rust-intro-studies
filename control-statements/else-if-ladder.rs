
fn main() {
    let a = 100;
    let b = 200;
    let c = 300;

    if a > b && a >c {
        println!("A is greatest!");
    } else if b > a && b > c {
        println!("B is greatest!");
    } else {
        println!("C is greatest!");
    }
}