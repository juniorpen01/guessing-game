use std::io;

use rand::Rng;

struct Guesser {
    secret_number: i32,
}
impl Guesser {
    fn new() -> Self {
        let mut rng = rand::rng();
        let secret_number = rng.random_range(1..=100);
        Guesser { secret_number }
    }
    fn guess(&self, num: i32) -> std::cmp::Ordering {
        num.cmp(&self.secret_number)
    }
}

struct GuessReader;
impl GuessReader {
    fn read() -> i32 {
        loop {
            println!("Please input your guess.");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("failed to read line");

            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {guess}");

            break guess;
        }
    }
}

struct GuessResultProcessor;
impl GuessResultProcessor {
    fn process(result: std::cmp::Ordering) -> bool {
        use std::cmp::Ordering::*;
        match result {
            Less => {
                println!("Too small!");
                false
            }
            Greater => {
                println!("Too big!");
                false
            }
            Equal => {
                println!("You win!");
                true
            }
        }
    }
}

struct Game;
impl Game {
    fn play() {
        println!("Guess the number!");

        let guesser = Guesser::new();
        loop {
            let guessed_correctly =
                GuessResultProcessor::process(guesser.guess(GuessReader::read()));
            if guessed_correctly {
                break;
            }
        }
    }
}

fn main() {
    Game::play();
}
