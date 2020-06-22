extern crate facebook;
use facebook::*;

fn main() {
    let user = String::from("123");
    let pass = String::from("123456");
    let s = Login::Login(user,pass);
    if s == 1 {
        Post::post(String::from("How are you"))
    }
}
