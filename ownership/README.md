#Ownership

- One of Rust most unique feature.
- It enables Rust to make memory safety guarantees without needing a garbage collector.
- how rust lays data out in memory.


- Each value in rust has a variable that's called its owner
- there can only be one owner at a time.
- When the owner goes out of scope the value will be dropped.


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