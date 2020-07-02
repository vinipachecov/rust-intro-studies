use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Green", 20);
    scores.insert("Black",30);    
    let score = scores.get("Blue");
    println!("{:?}", scores);

    for (key, value) in &scores {
        println!("{}", value);        
    }
}