use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Dennis";
    let status = "100%";

    if command == "hello" {
        println!("Hello {}", name);
    } else if command == "status" {
        println!("Your status {}", status);
    } else {
        println!("That's not a valid command")
    }

    // println!("Args: {:?}", command);
}
