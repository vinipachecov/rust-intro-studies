fn main() {
    let mut v = vec! [1,2,3,4];
    //  to make changes remember to set & mut!!!
    for i in  & mut v {
        *i*=2;
        println!("{}", i);
    }

    //  leads to error due to ownership
    // for i in v {
    //     println!("{}", i);
    // }
}
