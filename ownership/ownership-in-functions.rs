fn main () {
    let mut s = String::from("hello");
    take(s);
    s = println!("{}",s);    
//   |          - value moved here
// 4 |     println!("{}",s);
//   |                   ^ value borrowed here after move
}

fn take(s: String) ->String {
    println!("{}", s);
    return s;
}

