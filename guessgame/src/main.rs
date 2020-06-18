
extern crate rand;
use rand::{Rng, thread_rng, };
use std::io;

fn main() {
    println!("Guess a number");
    let mut rng = thread_rng();
    let secret = rng.gen_range(0,10);
    loop {
        println!("Input your guess");
        let mut guess = String::new();        
        io::stdin().read_line(&mut guess).expect("Failed");
        let guess:i32 = guess.trim().parse().expect("Failed");
        if guess == secret {
            println!("Guessed correctly");
            break;
        } else {
            println!("Try again");
            if guess > secret {
                println!("You have guessed a higher number");
            } else {
                println!("Value is smaller");
            }
        }

    }
}

