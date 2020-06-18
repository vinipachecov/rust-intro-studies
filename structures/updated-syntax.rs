#[derive(Debug)]
struct User { 
    name: String,
    age: i32, 
}

fn main() {
    let u1 = User {
        name:String::from("Name"),
        age: 12,
    };
    let u2 = User {        
        ..u1
    };
    println!("{:?}",u2);
}