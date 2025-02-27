#[derive(Debug)]
struct User { 
    name: String,
    age: i32, 
}

fn example() {
    let u1 = User {
        name:String::from("Name"),
        age: 12,
    };
    let u2 = User {        
        name: u1.name.clone(),
        age: u1.age.clone(),
    };
    println!("{:?}",u2);
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

    example();
}