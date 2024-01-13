use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!();
    println!("!!!!!WELCOME TO THE GUESSING GAME!!!!!");
    println!();

    //generate a random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is : {}", secret_number);

    loop {
        println!();
        println!("Please enter your guess : ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("{}","INVALID INPUT".red());
                continue;
            }
        };
        println!("You guessed : {}", guess);


        //compare guess with secret number

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Guess is too small".red()),
            Ordering::Greater => println!("{}","Guess is too big".red()),
            Ordering::Equal => {
                println!();
                println!("{}","!!!!!YOU WIN!!!!!".green());
                println!();
                break;
            },
        }
    }
}
