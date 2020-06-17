use std::io;

/**
 * Fictional social media auth
 */

fn main() {
    let mut name = String::new();
    let mut age = String::new();
    let mut ch = String::new();

    println!("Enter name and age");
    io::stdin().read_line(&mut name).expect("Failed reading name");
    io::stdin().read_line(&mut age).expect("Failed reading age");
    let age:i32 = age.trim().parse().expect("failed parsing age!");

    println!("Do you want to create account?");
    io::stdin().read_line(&mut ch).expect("Failed reading option");
    ch = ch.trim().to_string();
    if ch == "y" {
        if age < 10 {
            println!("Your age is too low");
        } else {
            println!("Your account is created!");
            println!("Do want to upload a photo?");
            
            // requires to "reset" the previous values            
            ch.clear();

            io::stdin().read_line(&mut ch).expect("Failed reading option");            
            ch = ch.trim().to_string();
            println!("{}", ch);

            if ch =="y" {
                 if age < 13 {
                     println!("You cannot upload photo");
                 } else {
                     println!("You can upload your photo");
                 }
            } else {
                println!("Thanks for visiting");
            }

        }
    }

}