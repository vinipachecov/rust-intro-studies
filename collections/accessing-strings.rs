fn main() {
    let s1 = String::from("Hello");    

    for c in s1.chars() {
        print!("{}", c);
    } 

    for c in "Hello".chars() {
        println!("{}", c);
    } 
}