use std::io;

fn main() {
    println!("Hello, world! Example keyboard read");

    println!("Enter the username:");

    let mut username = String::new(); // new Static method => ''
    // Result -> success or error
    io::stdin().read_line(&mut username); // reference Username
    let username = username.trim();
    let mut age =  String::new();
    println!("Enter the age of user:");
    io::stdin().read_line(&mut age);
    let age =  age.trim();
    let age: i32 =  age.parse().unwrap();

    println!("The value of username is: {}", username);
    println!("The username has {} years old.", age);
}
