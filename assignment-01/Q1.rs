use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("Failed to read char from input.");
    let res:char = input.trim().parse().expect("Failed parsinig");

    if res == 'a' || res == 'o' || res == 'e' || res == 'u' ||res == 'i'  {
        println!("Vowel!");
    } else {
        println!("Not a Vowel!");        
    }
}