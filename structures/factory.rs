#[derive(Debug)]
struct Rectangle {    
    width: u32,
    height: u32,
}

impl Rectangle {
    fn build(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}


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
    println!("{:?}",u1);
    let rect1 = Rectangle::square(20);
    let rect2 = Rectangle::build(10,30);

    println!("{:?}", rect2);

    
}