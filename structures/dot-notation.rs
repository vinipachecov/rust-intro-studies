#[derive(Debug)]
struct User { 
    age: i32,
}

fn main() {
    let user1 = User {
        age: 68,      
    };
    let user2 = User {
        age: 10,      
    };
    println!("{:?}",user1.age);
    println!("{:?}",user2.age);
}