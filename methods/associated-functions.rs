#[derive(Debug)]

struct Rectangle {
    height: u32,
    width:u32
}

impl Rectangle {
    fn build(width: u32, height:u32)-> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let r1 = Rectangle::build(32, 52);
    println!("{:?}",r1);
}