fn main() {
    let a: (i32,bool,f64)= (220,true, 8.5);    
    new_print(a);    
    tuple_print();
}

fn new_print(x: (i32,bool,f64)) {
    let (a,y,z) = x;
    println!("{} {} {}", a,y,z);
}

fn tuple_print() {
    let tuple = (3,2,1);
    println!("{}", tuple.0);
}