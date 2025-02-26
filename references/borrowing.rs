// fn main() {
//     let  s = String::from("Hello");
//     print(& s);  // s will be dropped here     
// }

// fn print(s: & String) {
//     println!("{}",s);    
// }


fn main() {
    let mut s = String::from("Hello");
    print(&mut s); // needs mut keyword to allow func to change "s"
    println!("S still exists in the main scope: {}",s);
}

fn print(s: &mut String) {
    println!("{}",s);
    s.push_str(" world");
}

