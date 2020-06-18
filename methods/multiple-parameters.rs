#[derive(Debug)]

struct Rectangle {
    height: u32,
    width:u32
}

impl Rectangle {
    fn can_hold(&self,other: &Rectangle)->bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let r1 = Rectangle { width: 50, height: 30 };
    let r2 = Rectangle { width: 30, height: 50 };
    let can_hold = r1.can_hold(&r2);
    println!("{}",can_hold);
}