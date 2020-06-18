fn main() {
    let mut acc = 0;
    loop {
        acc += 1;
        println!("{}",acc);
        if acc > 10 {
            break;
        }
    }
}