# Rust intro studies
This repository is a follow on code from the course on Udemy pointed in the description.


## Basics about Rust
Rust is a compiled Language and requires this process before executing our programs.
To do this we use the rustc command like:
```
rustc hello.rs
```

This will generate an executable file with the specific cpu instructions from your device. In the previous example it will be "hello".

### Printing different types
```rs
fn main() {
    println!("hello world");
    println!("{}",3.5);
    println!("{}",2);
}
```

- We have to use "{}", the placeholder notation, to bind our values.

### Data Types

- Every value in rust is of a certain data tye
- Rust is statically typed language.
- The compiler can usually infer what type we want to use based on the value and how we use it.
- we need to define type of variable in cases when many types are possible, such as when we are converting a string to a numeric type.

- integers
- floating point
- numbers
- booleans 
- characters

#### Variable types

### Mutable and local variables
Variables declared with ```let``` are used in local scope. So they cannot be declared in global scope like constants.
```rs
let mut varName = 10;
``` 
This is an example of a declaration of a variable that is not a constant.
### Immutable variable
- Constant variables require the type to provided, otherwise an error will be thrown.
- Constants can be declared in global scope while variables declared with *let* cannot.
- Constant variables cannot receive values from functions, they must be hard coded, otherwise the compile will throw an error.
```rs
const varName: i32 = 10;
``` 


#### Strings
A variable that receives something in double quotes is not a string, but a *Slice*
- Rust has only string type in the core language: string slice (&str)
- The string type is provided by rust's standard library rather than coded into the core language
- string is a growable, mutable utf-8 encoded type. In other words, strings are mutable. Slices are not mutable by the way.

### Ways to create a String
Declared way
```rs
fn main () {
    let phrase = "Hello";
    println!("{}", phrase);
    stringFromOperator()
}
```

Using from operator
```rs
fn stringFromOperator() {    
    let phrase = String::from("Hello");
    println!("{}", phrase);
}
```

Using new operator
```rs
fn stringNewOperator() {    
    let mut phrase = String::new();
    phrase = phrase + "123456";
    println!("{}", phrase);
}
```

Pushing a string value into a mutable string variable
```rs
fn new_string_push_method() {    
    let mut phrase = String::new();
    phrase.push_str("124");
    println!("{}", phrase);
}
```


## Operators
The operators supported in rust are

- +
- - (minus)
- / (division)
- * (multiplication)
- % (mod)
- >
- <
- >= (more or equal)
- <= (less or equal)
- == (equal operator)
- = 
- && (and)
- || (or)
- ! (negation operator)

## Comments

Comments in Rust can be done with double ```\\``` or with ```\**\```.

```rs
fn main() {
    let a = "something"
    // this will print something
    println!("{}", a)
}
```

```rs
fn main() {
    let a = "something"
    /* this will print something
    into the terminal where it gets executed.
    */
    println!("{}", a)
}
```

## Shadowing
Shadowing is the feature that allows the developer to declare a new variable with the same name as a previous variable. After shadowing, you cannot change the value of the shadowed value.

```rs
fn main() {
    let a = 10;
    // Here it will print 10 
    println!("Value = ", a);
    let a = 20;
    // Here it will print 20 
    println!("Value = ", a);
}
```

Shadowing, while it denies the possibility of changing the value, it allows the new variable to have a different type than the "original".
```rs
fn main() {
    let a = 10;
    // Here it will print 10 
    println!("Value = ", a);
    let a = "20";
    // Here it will print 20 
    println!("Value = ", a);
}
```

## Typecasting
Typecasting is not possible in rust by default. However typecasting can be used like:

```rs
fn main() {
    let a:i32 = 32;
    // it will raise warnings but will compile.
    let b:i64 = a.into();
}
```

```rs
fn main() {
    let a:i32 = 32;
    // it will not compile work.
    let b:i64 = a + 10;
    // this will work
    let b:i64 = a as i64 + 10;
}
```

## IO input by terminal
We can read a line with the standard library read_line method:

```rs
use std::io;
fn main() {
    let mut a = String:new();
    // pass a reference to our mutable variable a to store the input provided
    io::sdtin().read_line(&mut a).expect("Failed");
    let a:String = a.trim().parse().expect("Failed parsing")/;
    println!("{} World", a);
}
```

read_line must receive a mutable reference of the variable, so it can store the user input into it.
The method also is chained with a expect method to catch possible errors.

## Conditional statements

### If else

if else statements in rust doesn't require parenthesis in its syntax:

```rs
let a: i32 = 10;
if a < 20 {
    println!("Lower than 20!");
} else {
    println!("Bigger than 20!");
}
```

### If let
Similar to ternary expressions we can use if statements while assigning values to a variable:

```rs
let result = if 10 < 5 {
    println!("Assigning result var with value: 5");
    // last line should contain the value to be assigned
    5
} else {
    // last line should contain the value to be assigned
    println!("Assigning result var with value: 10");
    10
// here the last curly brace should receive a semi-colon.
};
```

## Loops

### Loop
Similar to while true:
```rs
fn main() {
    let mut counter = 0;
    loop {
        println!("Counter value {}", counter);
        counter += 1;
        if (counter == 10) {
            break;
        }

    }
}
```
### While loop

```rs
fn main() {
    let mut counter = 0;
    while counter != 10 {
        println!("Counter value {}", counter);
        counter += 1;
    }
}
```

### For loop

```rs
fn main() {    
    for counter in 0 .. 10 {
        println!("Counter value {}", counter);        
    }     
}
```

## Functions

Functions in rust can be declared:

- no arguments and no return type
```rs
fn function_name() {
    // function body
}
```
- multiple arguments and no return type
```rs
fn function_name(arguments list) {
    // function body
}
```
- multiple arguments and one return type
```rs
fn function_name(arguments list) -> return_type {
    // function body
}
```
- multiple arguments and multiple return types
```rs
fn function_name(arguments list) -> (return_type, return_type2) {
    // function body
}
```
- Function inside function
```rs
fn function_name() {
   fn fun_function() {
       // statements here
   }
}
```

### Function return statement 
In rust, return statements can be explicit:

```rs
fn add(a: i32, b:i32) {
    return a + b;
}
fn main() {
    println!("{}", add(1,1));
}
```

## Tuples in Rust

Tuples is a compound Datatype. Differently than scalar types where you have only, i.e i32, in tuples you can store different types in the same variable (tuple).

Tuples have a fixed length and cannot grow or shrink.
```rs
let tuple: (i32, f64, u8) = (324,4.9, 22)
```

Tuples can be accessed by using indexes (which start counting from 0): 
```rs
let tuple: (i32, f64, u8) = (324,4.9, 22)
// this will print 22
println!("{}", tuple.2)

// this will print 324
println!("{}", tuple.0)

// this will print 4.9
println!("{}", tuple.1)
```

