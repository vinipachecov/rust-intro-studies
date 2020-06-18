# Methods

* methods are similar to functions: they are declared with the fn keyword and their name
* They can have parameters and a return value
* However, methods are different from functions in that they are defined within the context of a struct
* Their first parameter is alwys "self", which represents the instance of the struct 
 Example:
 ```rs
 struct Retangle {
     width: u32,
     height: u32,
 }

 impl Rectangle {
     fn area(&self)->u32 {
         self.width * self.height
     }
 }
 ```