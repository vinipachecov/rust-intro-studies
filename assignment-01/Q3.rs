use std::io;

fn main() {
    let mut per = String::new();
    println!("Enter your percentage:");
    io::stdin().read_line(&mut per).expect("Failed");
    let per:i32 = per.trim().parse().expect("Failed parsing");
    if per >=90 {
        println!("A GRADE");
    } else if per >=80 {
        println!("B GRADE");
    }
     else if per >=70 {
        println!("C GRADE");
    }
     else {
        println!("D GRADE");
    }
 
}