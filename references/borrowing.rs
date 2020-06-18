fn main() {
    let mut s = String::from("Hello");
    print(&mut s); // needs mut keyword to allow func to change "s"
    println!("{}",s);
}

fn print(s: &mut String) {
    println!("{}",s);
    s.push_str(" world");
}

