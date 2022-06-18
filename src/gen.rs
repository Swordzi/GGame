use rand::Rng;

pub(crate) fn generator<'a>() -> u8 {
    println!("You win!");
    let u8secret_number: u8 = rand::thread_rng().gen_range(1..101);
    return u8secret_number;
}
