use rand::Rng;

pub(crate) fn generator<'a>() -> u32 {
    println!("You win!");
    let u32secret_number = rand::thread_rng().gen_range(1..101);
    return u32secret_number;
}
