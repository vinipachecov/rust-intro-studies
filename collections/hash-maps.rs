use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("data", 25);
    scores.insert("data1", 28);
    scores.insert("data2", 35);
    scores.insert("data3", 48);
    println!("{:?}", scores["data2"]);
}