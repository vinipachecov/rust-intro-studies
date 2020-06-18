// 3. Find the number of digits in a number;

fn main() {
    let mut num = 3244;
    let mut acc = 0;
    while num !=0 {
        acc+=1;
        num/=10;
    }
    println!("Number of digits = {}", acc);
}