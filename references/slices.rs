fn main() {
    let a = String::from("Hello World");
    let r1 = &a[0..5];
    println!("r1: {}",r1);
    let r2 = &a[1..5];
    println!("r2 : {}",r2);
    let r3 = &a[..5];
    println!("r3 : {}",r3);
    let r4 = &a[3..];

    println!("r4 : {}",r4);
    let r5 = &a[..];

    println!("r5 : {}",r5);
    
    let b = &a[0..3];
    println!("{}",b);
}