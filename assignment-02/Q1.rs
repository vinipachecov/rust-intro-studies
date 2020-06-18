// 1. Write a program to print all even numbers between 1 to 100.

fn main () {
    for number in 1 .. 101 {
        if number %2 == 0 {
            println!("number {}", number);
        }
    }
}