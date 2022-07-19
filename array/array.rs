fn main() {
    let a: [i32;4] = [22,32,55,66];
    let mut b: [i32;5] = [0;5];
    b[2] =5;
    println!("{:?}",a);
    println!("{:?}",b);
    print_array(b);


    // Error
    // let mut c: [i32;5];
    // c[3] = 5;
    // println!("{:?}",c);
}

fn print_array(x: [i32;5]) {
    println!("x len = {}",x.len());
    // for n in 0 .. 5 {
    //     println!("{}", x[n]);
    // }
    
    for n in x.iter() {        
        println!("{}", n);
    }

    
}