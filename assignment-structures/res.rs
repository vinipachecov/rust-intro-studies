struct Student {
    name: String,
    c:i32,
    java:i32,
    rust:i32    
}

impl Student{
    fn highest(&self) {
        if self.c > self.java && self.c > self.rust {
            println!("Highest marks in C");
        } else if self.java > self.c && self.java > self.rust {
            println!("Highest marks in Java");
        } else {
            println!("Highest marks in Rust");
        }
    }

    fn build(name: String, c:i32, java:i32, rust:i32) -> Student {
        Student {
            name,
            c,
            java,
            rust
        }
    }
}

fn main() {
    let s1 = Student::build(String::from("Job"),10,25,32);
    s1.highest();
}