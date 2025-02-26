

/*
Double quotes are not strings, but slices in Rust.

In Rust the STR type is provided by the standard library 
and not from the core language.

String is growable mutable and UTF-8 encoded type.

string slices (let phrase = "Hello World") are not mutable

Strings are.


New String  

let mut mutable_str = String::new();

New string from a string value

let mut mutable_str = String::from("Hello world");
*/
fn main () {
    let phrase = "Hello";
    println!("{}", phrase);
    stringFromOperator();
    new_string_slice();
}

fn stringFromOperator() {    
    let phrase = String::from(" World");    
    
    let mut phrase_complete = String::from("Hello");
    phrase_complete.push_str(&phrase);
    println!("{}", phrase_complete);
}


fn stringNewOperator() {    
    let mut phrase = String::new();
    phrase = phrase + "123456";
    println!("{}", phrase);
}

fn new_string_push_method() {    
    let mut phrase = String::new();
    phrase.push_str("124");
    println!("{}", phrase);
}

fn new_string_slice() {
    let a = "Hello";

    let b = String::from(a) + " World";

    println!("Hello world from combined slice => {}", b);
}