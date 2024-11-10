use core::num;
use std::io;
use std::cmp::Ordering;
use rand:: Rng;

fn main() {
    println!("Guess the number!");

    let random_number: u32 = rand::thread_rng().gen_range(1..=101);

    loop {
        print!("Please type your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read lines!");
    
        let guess: u32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
        };
    
        println!("Your guessed input : {}", guess);
    
        match guess.cmp(&random_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too Large")

        }
    }
}