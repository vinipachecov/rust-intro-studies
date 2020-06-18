 # LOOPS

 Rust has 3 types of loops
 - loop
 - while
 - for

 ## Loop

 Simple loop

 Syntax:
 ```rs
 loop {
     //statements
 }
 ```
- breaking condition must be explicitly given.
-  best when you want to run an infinite loop.

## While

Normal while loop that we use a loop with a condition to check.
Syntax:
```rs
let mut a = 0;
while a < 10 {
    a+=1;
}
```

## For loop
For loop is used to iterate over collections.
syntax: 
```rs
for var_name in range {
    //statements;
}
```
Note that while loop is slow because the compilar adds runtime coed to perform the conditional check on every element on every iteration through the loop.