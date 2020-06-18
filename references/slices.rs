fn main() {
    let a = String::from("Hello World");
    let r1 = &a[0..5];
    let r2 = &a[1..5];
    let r4 = &a[..5];
    let r4 = &a[3..];
    let r3 = &a[..];
    let b = &a[0..3];
    println!("{}",b);
}