
/**
 * No typecasting by default
 *  we use "as" keywor to typecast to another type
 */
fn main() {
    // let a:i32 = 10;
    // let b:i64 = a as i64;
    // println!("{}",b);

    // let a:i32 = 10;
    // let b:i64 = a as i64 + 10;
    // println!("{}",b);

    let a:i32 = 10;
    let b:i64 = a.into();
    println!("{}",b);

    let a:i32 = 10;    
    let a:i64 = a as i64 + 10;
    println!("{}",a);
}