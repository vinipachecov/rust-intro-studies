use std::collections::HashMap;
use std::io;

fn main() {
    let mut n = String::new();
    let mut name = String::new();
    let mut no = String::new();
    let mut v_name:Vec<String>=Vec::new();
    let mut v_no:Vec<String>=Vec::new();
    let mut c = 1;
    let mut choice = String::new();
    

    println!("How many contacts you want to store?");
    io::stdin().read_line(&mut n).expect("Failed reading number of contacts");
    let n:u32 = n.trim().parse().expect("Fail");


    while c <= n {
        name.clear();
        no.clear();
        println!("Enter name than no.");

        io::stdin().read_line(&mut name).expect("Failed reading no.");
        let name:String = name.trim().parse().expect("Fail");

        io::stdin().read_line(&mut no).expect("Failed reading no.");
        let no:String = no.trim().parse().expect("Fail");

        v_name.push(name);
        v_no.push(no);
        c+=1;
    }
    println!("{:?} {:?}", v_name, v_no);

    let contact:HashMap<&String, &String>=v_name.iter().zip(v_no.iter()).collect();

    println!("{:?}", contact);

    println!("Which name to search");

    io::stdin().read_line(&mut choice).expect("Failed reading no.");
    let choice:String = choice.trim().parse().expect("Fail");

    search(contact, &choice);
}

fn search(contact: HashMap<&String, &String>, choice: &String) {
    for (key, value) in contact {
        if key == choice {
            println!("Contact {} {}", key, value);
        }
    }
}