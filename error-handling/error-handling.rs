
fn fun_that_throws() -> Result<String, &'static str> {
    return Err("Something went wrong");
}

fn create_string() -> String {
    return String::from("A new string");
}

fn main() {
    let new_str = create_string();
    println!("{}", new_str);
    let result = fun_that_throws();
    // match result {
    //     Ok(n) => println!("No error!"),
    //     Err(e) => println!("Ocorreu erro!"),
    // }
}