use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number");

    //generating secrete number
    let secrete_number = rand::thread_rng().gen_range(1..=100);

    loop {
            println!("Please input your guess.");
    
    //create a mutable variable to store user input  
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    //create shodow for line (reassign)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //prevent code from breaking on wrong input
        };
        println!("You guessed: {guess} \n the secrete number was {secrete_number}");

    //validating guess
        match guess.cmp(&secrete_number) {
            Ordering::Less => println!("Too samll!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            },
        }
    } 
    //let x: i16 = 5;
    //let y: i16 = 10;

    //println!("x = {x} and y + 2 = {}", y + 2);


}
