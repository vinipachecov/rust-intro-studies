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


#### Strings

- Rust has only string type in the core language: string slice (&str)
- The string type is provided by rust's standard library rather than coded into the core language
- string s a growable, mutable utf-8 encoded type.


