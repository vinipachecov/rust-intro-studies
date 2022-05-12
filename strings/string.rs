fn main () {
    let phrase = "Hello";
    println!("{}", phrase);
    stringFromOperator()
}

fn stringFromOperator() {    
    let phrase = String::from("Hello");
    println!("{}", phrase);
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