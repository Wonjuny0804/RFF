use std::io; // we need the IO for user to input and out data

// main is a unique name, always run first
// fn is shortened fro function
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    /*
     Storing Values with Variables
     the code below creates a variable to create
     data.

     in Rust variables are immutable by "default"
     if you want to mutate them need a special keyword
     "mut"
    */

    // the String::new()  is calling a function 
    // and it creates a new, empty string
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // Returns a io::Result


    println!("You guessed: {}", guess);

}
