mod gen;

use std::cmp::Ordering;
use std::io;

fn main() {
    let u8secret_number: u8 = gen::generator();
    let mut guessed: bool = false;
    let mut bypass: String = String::new();


    println!("Do you want to see the secret number? (y/n)");
    io::stdin()
        .read_line(&mut bypass).expect("TODO: panic message");
    if bypass.trim() == "y" {
        println!("The secret number is {}", u8secret_number);
    }
    println!("Please input your guess.");
    while guessed == false {
        {
            println!("Please input your guess.");
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u8 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&u8secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    guessed = true;
                    println!("You win!");
                }
            }
        }
    }
}
