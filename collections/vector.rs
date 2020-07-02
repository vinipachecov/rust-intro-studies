fn main() {
    let mut v: Vec<i32> = Vec::new();
    for n in 0 .. 10 {
        v.push(n);
    }

    let mut cv:Vec<i32> = vec![1,2,3];
    println!("{:?}", v);
    println!("{:?}", cv);
} 