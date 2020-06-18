#[derive(Debug)]
struct User { 
    name: String,
    age: i32,
}

fn build(name: String, age:i32)->User {
    User {
        name,
        age
    }
}

fn main() {
    let name = String::from("Vini");
    let u1 = build(name, 20);       
    println!("{:?}",u1)
}