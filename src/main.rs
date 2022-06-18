mod gen;

use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Please input your guess.");

    let u8secret_number: u8 = gen::generator();
    let mut guessed: bool = false;
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
