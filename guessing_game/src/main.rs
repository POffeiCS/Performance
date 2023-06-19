use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn compare(guess: u32, rand: u32) {
    match guess.cmp(&rand) {
        Ordering::Less => println!("Too Low::guess is {guess}"),
        Ordering::Greater => println!("Too High::guess is {guess}"),
        Ordering::Equal => println!("Winner::guess is {guess} and rand is {rand}"),
    }
}

fn os_random() -> u32 {
    let random_number: u32 = rand::thread_rng().gen_range(1..=200);
    random_number
}

fn guess() {
    let rand: u32 = os_random();
    loop {
        println!("Guess your favourite number!!!::!!!)");
        let mut guess: String = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the input");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        compare(guess, rand);
        
        if guess == rand {
            break;
        }
    }  
}



fn main() {
    println!("Welcome to Guess the Number Game::)");
    guess();
}
