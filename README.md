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
## Arrays

- Collection of values
- All values must be of same type (different than tuples)
- Length is fixed (similar to tuples)
- can be accessed by indexes starting from 0

Syntax:
```rs
let a = [1,2,3,5];
let b: [i32;5] = [1,2,3,4,5];
let c: [i32;5] = [0;5];

// prints the whole array!
println!("{:?}", b);
```

## Ownership

- One of Rust most unique feature.
- It enables Rust to make memory safety guarantees without needing a garbage collector.
- how rust lays data out in memory.


- Each value in rust has a variable that's called its owner
- there can only be one owner at a time.
- When the owner goes out of scope the value will be dropped.

### Memory management with Stack
Just like in normal stacks it works as "last in, first out."
<i>The rust stack stores values in the order it gets them and removes them in the opposite order</i>

- Adding an item in the stack is a push operation
- Removing an item from the stack is a pop operation

Stacks must use a known and fixed size. <strong>Stacks doesn't grow at runtime.</strong>

### Memory management with Heaps
Heaps can store data with a size unknown at compile time or a size that might change during it.

Example:
String -> push_str(): You don't know how much string memory slots will be required before this operation occurs. In this situation a Stack will not be possible to be used since it requires a fixed ammount of memory known at compile time. 

<strong>Soluion</strong>: Heap, which is less "organized"
Behind the scenes, the operating system will find a memory slot big enough to store the requested amount of space and use it.

When in use, the heap returns a pointer, i.e a address of the allocated space.
Pushing values into a heap is not considered allocating, since it is already allocated.
### Stack vs Heap

Stack it is fast because of the way it access the data.
Stack it is fast because of the fixed known memory ammount. 

Heap when allocating a large amount of space can take time.

## Variable scope

```rs
fn main() {
    let a = 10;
}

fn fun() {
    a = 20 // error
}
```

## Memory allocation

String type in order to support a mutable, growable piece of text, we need to allcoate na amount of memory on the heap, unkown at compile time, to hold the contents.
- The memory must be requested from the operating system at runtime.
- We need a way of returning this memory to the operating system when we're done with our String.


Languages with a garbage collector(GC), the GC keeps track and cleans up memory that isn't beign used anymore, and we don't need to think about it.

Without a GC, it's our responsibility to identify when memory is no longer being used and call code to explicitly return it.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

Example:

```rs
fn main() {
    let s = String::from("hello"); //s is valid from this point forward
    // do stuff with s
}
//this scope is now over, and s is no longer valid
```

When a variable goes out of scope, Rust calls a special function for us. this function is called drop, Rust calls drop automatically at the closing bracket.

### Double free memory problem in ownership

This is a problem related to more than one variable having æccess to the same memory space.
With this situation an error called "double free memory" can happens and lead to a bug or crash in the program. 
To solve this rust uses the "move" pattern which avoids data (memory space) that are not static to have multiple ownership.

```rs
fn main() {
    let a = String::from("hello"); 
    let b = a;
    println!("{} {}", b, a);
    //  value moved here
    //  |     println!("{} {}", b, a);
    // 
```

If we need to copy a variable we will need to copy the data into another variable:

```rs
fn working_move() {
    let a = String::from("hello"); 
    let b = a.clone();
    println!("{} {}", b, a);
    //  value moved here
    //  |     println!("{} {}", b, a);
    //   |                          ^ value borrowed here after move
}
```

Note that this doens't happen with stack-only data (variables that live in the stack memory) which has fixed memory size in compile time.

## Memory references

Rust allows users to have mutable and immutable references. However, it does not allow more than once mutable reference per scope:

```rs
fn main() {
    let mut s1 = String::from("Hello")
    let mut s2 = String::from("World") // rust compiler will throw error here!
}
```

Same will happen if an immutable reference is attemp to be passed to as mutable:

```rs
fn main() {
    let s = String::from("Hello");
    let s2 = &s;
    let s3 = & mut s; // cannot borrow as mutable | error here
    println!("{}", s3);
}
```

### Dangle References

```rs
fn main() {
    let s = dangle();
}

fn dangle() -> &String {
    let d = String::from("world");
    &d;
}// when out of scope, variable d will be dropped and its content will also 
// be deleted.
```

### Slices
Slice lets you reference contiguous sequence of elements. 

Example:
```rs
fn main() {
    let a = String::from("Hello World");

    // 'Hello '
    let r1 = &a[0 .. 5];

    let r2 = &a[0 ..=5];

    let r3 = &a[ .. 5];

    let r4 = &a[0 ..];

    let r5 = &a[..];
}
```

## Cargo, Rust package manager
Cargo is the package manager for Rust, so any time there a need for using an external library, cargo will be used.
Some useful commands:

New project
- cargo new proj_name --bin 

New library
- cargo new proj_name --lib

Build project
- cargo build

Check project compilation
- cargo check

Run project
- cargo run

Cargo also simplify the usage of rust across different OS because the commands are the same.

## Crates
Crate is a package of Rust code. Itcan be a binary or a library depending the cargo new flag.

## Structure
A struct or _structure_, is a custom data type that lets you name and package together multiple related values.

tuples and structs are different but similar.

Like tuples, the pieces of a struct can be of different types.

Unlike with tuples, you'll name each pice of data so it`s clear what the values mean.

As a result of these names, structs are more flexible than tuples.
    * this way you don't have to rely on th order of the data to specify or access values.

```rs
// use camel case for naming structs
struct User {
    // property_name: type;
    username: String;
    email: String;
}

fn main() {
    let user1 = User {
        email: String::from("a@a.com"),
        username: String::from("My first name")
    };

    println!("{}", user1.email);
}
```

To print a struct entire object 

```rs
// use camel case for naming structs
#[derive(Debug)]
struct User {
    // property_name: type;
    username: String;
    email: String;
}

fn main() {
    let user1 = User {
        email: String::from("a@a.com"),
        username: String::from("My first name")
    };

    // note the :? inside the curly brackets
    println!("{:?}", user1);
}
```

### Dot notation to access properties 

To get a specific value from a struct, we can use dot notation.

For example:

```rs
struct User {    
    username: String;
    email: String;
}

fn main() {
    let user1 = User {
        email: String::from("a@a.com"),
        username: String::from("My first name")
    };

    // accessing only email prop
    println!("{}", user1.email);
}
```

Changing values from a struct:

```rs
struct User {    
    username: String;
    email: String;
}

fn main() {
    let mut user1 = User {
        email: String::from("a@a.com"),
        username: String::from("My first name")
    };
    user.email = String::from("b@b.com"),

    // accessing only email prop
    println!("{}", user1.email);
}
```


### Returning Instance from Function

```rs
#[derive(Debug)]
struct User {    
    age: i32
}

fn main() {
    let user1  = build(20);
    
    println!("{:?}", user1);
}

fn build(age: i32) -> User {
    User {
        age: age
    }
}
```

### Field init Shorthand


```rs
#[derive(Debug)]
struct User {    
    age: i32
}

fn main() {
    let user1  = build(20);
    
    println!("{:?}", user1);
}

fn build(age: i32) -> User {
    User {
        // shorthand where we don't need to set if a prop from a struct has the same name from another variable
        age
    }
}
```

### Copying values from one struct to another without breaking ownership


```rs
#[derive(Debug)]
struct User {    
    age: i32
}

fn main() {
    let user1  = build(20);
    // this will break
    // let user2  = build(user1.age);

    let user2  = build(user1.age.clone());

    
    println!("{:?}", user1);
}

fn build(age: i32) -> User {
    User {
        age: age
    }
}
```


### Structure methods
Methods are similar to functions: they are declared with the fn keyword and a name.

They can have parameters and return a value.

However, methods are different from function in that they are defined within the context of a struct

Their first parameter is always self, which represents the instance of the struct.

```rs
#[derive(Debug)]
struct Rectangle {    
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

// without & the self variable will receive (move) the value to it. Once the function returns
// the struct value will be lost.
    // fn area(self) -> u32 {
    //     self.width * self.height
    // }
}

fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 200
    }    

    println!("{}", rect1.area());
}
```

#### More parameters to struct methods


```rs
#[derive(Debug)]
struct Rectangle {    
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 200
    }; 
    let rect2 = Rectangle {
        width: 500,
        height: 400
    };   

    let rect2 = Rectangle {
        width: 50,
        height: 40
    };   

    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));

    println!("Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));
}
```

#### Associated functions
When we define a function within impl blocks that don't take self as a parameter. These are called associated functions.

They are created associated to a struct.

They are still functions, not methods, because they don't have an instance of the struct to work with.

For example: 

```rs
String::from("")
```

Associated functions are often used for constructors that will return a new instance of the struct.

```rs
#[derive(Debug)]
struct Rectangle {    
    width: u32,
    height: u32,
}

impl Rectangle {
    fn build(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle::square(20);
    let rect2 = Rectangle::build(10,30);

    println!("{:?}", rect2);

    println!("{}", rect1.area());
}
```


### Enums
Enums allows to define a type by enumarting its possible values

Example: having 2 types of IP addresses:
- IPV4
- IPV6

```rs

enum IpAddrKind {
    V4,
    V6
}

fn main() {
    let a = IpAddrKind::V4;
    let b = IpAddrKind::V6;

    println!("{:?}", a);
    println!("{:?}", b);
}
```


### Storing values in enum

Using a enum as a type for fields in a structure:

```rs
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}


fn main () {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1",)
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),        
    };
    println!("{:?}", home);
    println!("{:?}", loopback);
}
```

Another ways of using enums is to have predefined values 


```rs
enum DaysOfTheWeek {
    Monday,
    Tuesday,    
    //....
}
```

Using different types inside Enum:

```rs
enum IpAddr {
    V4(String),
    V6(String)
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}
```


Using multiple different types inside Enum:

```rs
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String)
}

fn main() {
    let home = IpAddr::V4(String::from(127,0,0,1));
    let loopback = IpAddr::V6(String::from("::1"));
}
```

### Option Enum

Option, which is an enum defined by the standard library.

Option is used in many places because it encodes the very common scenario in which a value could be something or could be nothing.

This feature can prevent bugs that are extremely common in other programming languages.

For example NUll is not present in rust.

Rust solution to Null -> Option

As such, rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.

This enum is:
```rs
 enum Option<T> {
    Some(T),
    None
 }
 ```

 By being included in the Rust prelude, it is allowed to be called in code without the normal enum syntax Option::Some.

 ```rs
fn main() {
    let num = Some(5);
    let text = Some("Hello");

    println!("{:?} {:?}", num,  text);
    // Some(5) Some("Hello")
}
```

A program that would not compile:
```rs
// This program will not compile
fn main() {
    let num = None;
    let text = None;

    println!("{:?} {:?}", num,  text);
}
```
Variables from Some will not inherit internal type properties
```rs
// This program will not compile
fn main() {
    let num = Some(5) + 5; // Compilation error here
    let text = None;

    println!("{:?} {:?}", num,  text);
}
```
