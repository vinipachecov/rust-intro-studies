/**
 * Supported operators
 *  +
 *  -
 *  /
 *  *
 *  %
 *  >
 *  <
 *  >=
 *  <=
 *  ==
 *  =
 *  &&
 *  ||
 * !
 */

fn main() {
  println!("add {}", 159 + 159);
  println!("minus {}", 2 - 2);
  println!("division {}", 2 / 2);
  println!("mult {}", 159 * 159);  

  // acc operations
  let mut a = 10;
  a +=a;
  println!("{}", a);


  // inequality operations    
  println!(" 3> 5 {}",  3 > 5);
  println!(" 3 < 5 {}",  3 <  5);


 // equality operations    
  println!(" 3 >= 5 {}",  3 >= 5);
  println!(" 3 <= 3 {}",  3 <= 3);

  // AND operations    
  println!(" 3 >= 5 && 4>2 {}",  3 >= 5 && 4>2);
  println!(" 3 <= 3 && 2 >1 {}",  3 <= 3 && 2 > 1);  

  // NOT operations    
  println!("!(3 >= 5) {}",  !(3 >= 5));
  println!("!(3 <= 3) {}",  !(3 <= 3));  
}