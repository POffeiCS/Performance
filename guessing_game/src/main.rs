use std::io;

fn guess() {
    println!("Guess your favourite number!!!::!!!)");
    let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("Failed to read the message");
    io::stdin().read_line(&mut guess).unwrap();
    println!("You guessed {guess}");
}



fn main() {
    println!("Welcome to Guess the Number Game::)");
    guess();
}
