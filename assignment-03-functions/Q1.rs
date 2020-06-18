fn main() {
    let res = factorial(5);
    println!("{}", res);
}

// for loop
fn factorial(number: i32) ->i32 {    
    let mut fact = 1;
    for step in 1 .. number + 1 {
        fact = fact * step;
    }
    return fact;
}
