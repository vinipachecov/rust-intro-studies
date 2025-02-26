# References 

- No limit to immutable referencers

- Only one mutable refence in a scope

- Can't create mutable reference if your program uses more than one immutable references.

# Data races
A data race is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is beign used to write to the data.
- There's no mechanism being used to synchronize access to the data.

# Rules of references

1) There can be any number of immutable references of a variable.
2)  Only one mutable borrowing for referencing per scope:
```rs
fn main() {
    let mut s = String::from("Hello");
    {
        let r1 = &mut s;
    }    
    let r2 = &mut s;
}
```
This avoids having multiple writers in a single mutable reference

3) Dangle references: You can't use a reference of a deallocated variable:
```rs
fn main() {
    let a = dangle();
}

fn dangle()-> &String {
    let d = String::from("something")
    &d;
}
```

