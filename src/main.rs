use std::io;

fn main() {

    println!("Guess the number");

    println!("Please input your guess.");
    
    //create a mutable variable to store user input  
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {guess} \n");

    let x: i16 = 5;
    let y: i16 = 10;

    println!("x = {x} and y + 2 = {}", y + 2);
    //generating secrete number


}
