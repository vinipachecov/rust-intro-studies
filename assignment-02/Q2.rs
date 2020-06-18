/**
 * 2. Create a quiz application. 
 * Ask a question from user and give 3 chances to attemp the question.
 *  If he answers the question correctly than quit the application.
 * 
 **/

 use std::io;
 
 fn main() {

    let mut user_answer = String::new();  
    println!("which is the longets river in the world");

    for n in 1 .. 4 {
        user_answer.clear();
        io::stdin().read_line(&mut user_answer).expect("failed!");
        user_answer=user_answer.trim().to_string();
        if user_answer == "Nile" {
            println!("Correct");
            break;
        } else if n!=3 {
            println!("Try again..");
        } else {
            println!("You lost...");
        }
    }
 }
  

 
 