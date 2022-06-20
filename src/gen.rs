use rand::Rng;

pub(crate) fn generator<'a>(x: u8) -> u8 {
    if x < 1 { // Check if it is the first call
        println!("Welcome to the game!");
        x + 1;
    } else { println!("You win!"); }
    let u8secret_number: u8 = rand::thread_rng().gen_range(1..101); // Generate a random number between 1 and 101
    return u8secret_number;
}
