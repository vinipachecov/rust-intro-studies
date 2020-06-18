fn main() {
    let a = dangle();
}

fn dangle()-> &String {
    let d = String::from("something")
    &d;
}