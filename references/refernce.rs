// Passing reference
// rust Move won't drop the data referenced but only the reference
fn main() {
    let s = String::from("Hello");
    print(&s);
    println!("{}", s);
}

fn print(s1: &String) {
    println!("{}", s1);
}