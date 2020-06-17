// fn main () {
//     let phrase = "Hello";
//     println!("{}", phrase);
// }

// fn main () {    
//     let phrase = String::from("Hello");
//     println!("{}", phrase);
// }


// fn main () {    
//     let mut phrase = String::new();
//     phrase = phrase + "123456";
//     println!("{}", phrase);
// }

fn main () {    
    let phrase = String::new();
    phrase.push_str("124");
    println!("{}", phrase);
}