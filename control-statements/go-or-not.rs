use std::io;

fn main() {
    let mut ch = String::new();
    println!("Are your friends coming ?");
    io::stdin().read_line(&mut ch).expect("Failed reading line.");

    if ch.trim().contains("y")  {
        println!("Yeah, lets go to the movies!");
    } else {
        println!("Maybe next time.");
    }

    // OR

    
    if ch.trim() == "y"  {
        println!("Yeah, lets go to the movies!");
    } else {
        println!("Maybe next time.");
    }

}