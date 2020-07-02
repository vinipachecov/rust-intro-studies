 fn main() {
     let a = String::from("hello");
     let b = String::from(" World");

    //  note the reference for the second param
    //  let s = a + &b;
    let s = format!("{} {}",a,b);
     println!("{}",s);

 }