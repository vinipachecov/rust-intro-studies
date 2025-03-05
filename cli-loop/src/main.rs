use std::io;

fn main() {
    loop {
        let mut command_line = String::from("");
        io::stdin().read_line(&mut command_line).expect("Error");

        let data = str::trim(&command_line);
        println!("Command typed: {}", &data);                
    }
}


