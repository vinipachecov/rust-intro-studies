fn main() {
    let a: (i32,bool,f64)= (220,true, 8.5);
    new_print(a);    
}

fn new_print(x: (i32,bool,f64)) {
    let (a,y,z) = x;
    println!("{} {} {}", a,y,z);
}