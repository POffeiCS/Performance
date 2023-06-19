use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn compare(guess: u32, rand: u32) {
    match guess.cmp(&rand) {
        Ordering::Less => println!("Too Low::guess = {guess} < rand = {rand}"),
        Ordering::Greater => println!("Too High::guess = {guess} > rand = {rand}"),
        Ordering::Equal => println!("Winner::guess = {guess} == rand = {rand}"),
    }
}

fn os_random() -> u32 {
    let random_number: u32 = rand::thread_rng().gen_range(1..=200);
    random_number
}

fn guess() {
    let rand: u32 = os_random();
    println!("Guess your favourite number!!!::!!!)");
    let mut guess: String = String::new();
    // io::stdin().read_line(&mut guess).expect("Failed to read the message");
    io::stdin().read_line(&mut guess).unwrap();
    
    let guess: u32 = guess.trim().parse().expect("I'm expecting a number");

    compare(guess, rand);
}



fn main() {
    println!("Welcome to Guess the Number Game::)");
    loop {
        guess();
    }
}
