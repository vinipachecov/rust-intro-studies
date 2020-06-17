/**
 * Shadowing: * 
 *  - You can declare a new variable wit h the same name as 
 * a previous variable.(redeclaration)
 * 
 * shadowing is different than marking a variable as mut
 */

fn main () {
    let a = 10;
    println!("{}", a);
    let a = 20;
    println!("{}", a);

    // allows redeclaration of variables 
    let mut a:i32 = 10;
    println!("{}", a);
    let a = 'c';
    println!("{}", a);
}