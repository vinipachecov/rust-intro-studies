pub fn Login(username: String, pass: String)->i32 {
    if username =="123" && pass == "123456" {
        println!("Logged in");
        1
    } else {
        0
    }
}