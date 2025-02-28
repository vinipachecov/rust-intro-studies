fn main() {
     let num = Some(5);
     let text = Some("Hello");

     println!("{:?} {:?}", num, text);
}

// throws error
// fn main() {
//      let num = None;
//      let text = None;

//      println!("{:?} {:?}", num, text);
// }

// fn main() {
//      let num: Option<i32> = None;
//      let text: Option<String> = None;

//      println!("{:?} {:?}", num, text);
// }

// Throws error
// fn main() {
//      let num: Option<i32> = Some(5) + 5;
//      let text: Option<String> = None;

//      println!("{:?} {:?}", num, text);
// }