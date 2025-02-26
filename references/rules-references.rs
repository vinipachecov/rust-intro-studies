// fn main() {//     let s = String::from("Hello");
//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &s;
//     let r4 = &s;    
// }
// 

// fn main() {
//     let mut s = String::from("Hello");
//     let r1 = &mut s;
//     let r2 = &mut s;
// }


// fn main() {
//     let mut s = String::from("Hello");
//     {
//         let r1 = &mut s;
//     }    
//     let r2 = &mut s;
// }

fn main() {
    let s = String::from("Hello");
    let s2 = &s;
    let s3 = & mut s; // cannot borrow as mutable
    println!("{}", s3);
}
