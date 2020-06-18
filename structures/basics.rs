#[derive(Debug)]
struct User {
    email: String,
    age: i32,
}

fn main() {
    let user1 = User {
        email: String::from("t@t.com"),
        age: 50
    };
    println!("{:?}",user1);
}