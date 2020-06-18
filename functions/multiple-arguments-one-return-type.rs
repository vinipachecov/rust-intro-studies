fn main() {
    let mut res:i32 = 0;
    res = add(2,3);
    // println!("{}", res);
    println!("{}", add(3,2));
}

fn add(a: i32, b:i32) -> i32 {
    // return a + b;
    a + b
}